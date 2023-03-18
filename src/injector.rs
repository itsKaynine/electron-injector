use std::process::{Child, Command};
use std::{fs, thread, time};

use anyhow::{bail, Result};
use log::{debug, info, warn};
use portpicker;
use serde_json::json;

use crate::assets;
use crate::config::Config;
use crate::protocol::{DevtoolPage, EvaluateResponse};
use crate::websocket::WebSocket;

struct UserScript {
    file_path: String,
    content: String,
}

pub struct Injector {
    config: Config,
    port: u16,
}

impl Injector {
    pub(crate) const INJECT_LOOP_SLEEP_MS: u64 = 1000;
    pub(crate) const WAIT_DEBUGGING_PORT_TIMEOUT_MS: u64 = 30_000;

    fn get_available_port(config: &Config) -> u16 {
        if portpicker::is_free_tcp(config.port) {
            info!("Using port: {}", config.port);
            return config.port;
        }

        info!(
            "Port {} is not available, finding another port",
            config.port
        );

        let port = portpicker::pick_unused_port().expect("Port should be available");
        info!("Found available port: {}", port);

        port
    }

    pub fn new() -> Self {
        // Parse CLI args
        let config = Config::parse_auto();

        // Get port
        let port = Injector::get_available_port(&config);

        Injector { config, port }
    }

    pub fn run(&self) -> Result<()> {
        info!("Running injector");
        debug!("{:#?}", self.config);

        // Spawn child process
        _ = self.spawn_process()?;

        // Prepare prelude script
        let prelude_script = self.get_prelude_script().unwrap_or(String::new());

        // Prepare user scripts
        let user_scripts = self.get_user_scripts();

        // Create timeout duration
        let timeout_duration = time::Duration::from_millis(self.config.timeout);

        // Declare a vec to store found page ids
        let mut found_page_ids: Vec<String> = Vec::new();

        // Inject loop
        let start_time = time::Instant::now();
        loop {
            // Refresh devtool pages
            let devtool_pages = self
                .get_devtool_pages()
                .expect("Should be able to get devtool pages");

            debug!("{:#?}", devtool_pages);

            // Loop through pages
            for page in devtool_pages {
                if found_page_ids.contains(&page.id) {
                    continue;
                }

                // Create WebSocket
                let mut ws = WebSocket::connect(&page.web_socket_debugger_url)
                    .expect("To connect to websocket");

                // Inject prelude
                if self.config.prelude {
                    info!("Injecting prelude script (id: {})", page.id);
                    _ = self
                        .evaluate(&mut ws, &prelude_script)
                        .expect("Should be able to evaluate JS");
                }

                // Inject scripts
                for user_script in user_scripts.iter() {
                    // Inject using evaluate
                    info!("Injecting script: {}", user_script.file_path);
                    _ = self
                        .evaluate(&mut ws, &user_script.content)
                        .expect("Should be able to evaluate JS");
                }

                // Save page id
                found_page_ids.push(page.id.clone());
            }

            // Check devtool pages again
            let updated_devtool_pages = self
                .get_devtool_pages()
                .expect("Should be able to get devtool pages");

            // Stop if already found all pages
            if found_page_ids.len() == updated_devtool_pages.len() {
                info!("Stopping injection loop");
                break;
            }

            // Timed out
            if start_time.elapsed() >= timeout_duration {
                bail!("Injection loop timed out");
            }

            // Sleep before next loop iteration
            thread::sleep(time::Duration::from_millis(Self::INJECT_LOOP_SLEEP_MS));
        }

        info!("Injection success");
        Ok(())
    }

    fn get_devtool_pages(&self) -> Result<Vec<DevtoolPage>, reqwest::Error> {
        let url = format!("http://{}:{}/json/list", &self.config.host, &self.port);

        let client = reqwest::blocking::Client::new();
        let response = client.get(url).send()?.error_for_status()?;

        let pages_response = response.json::<Vec<DevtoolPage>>()?;
        Ok(pages_response)
    }

    fn get_prelude_script(&self) -> Option<String> {
        // No need to load if not enabled anyways
        if !self.config.prelude {
            return None;
        }

        // Load from embedded file
        let file = assets::JS::get("prelude.js").unwrap();
        let script =
            std::str::from_utf8(file.data.as_ref()).expect("Script should be a valid UTF-8 file");

        Some(String::from(script))
    }

    fn get_user_scripts(&self) -> Vec<UserScript> {
        let scripts: Vec<UserScript> = self
            .config
            .script
            .iter()
            .map(|s| {
                let content =
                    fs::read_to_string(s).expect("Should have been able to read the file");

                UserScript {
                    file_path: s.to_string(),
                    content,
                }
            })
            .collect();

        return scripts;
    }

    fn spawn_process(&self) -> Result<Child> {
        // Prepare args
        let mut args = vec![format!("--remote-debugging-port={}", &self.port)];
        args.extend(self.config.arg.iter().map(|a| a.clone()));

        // Spawn child process
        debug!(
            "Spawning electron app: {} (args: {:#?})",
            &self.config.app, args
        );
        let child = Command::new(&self.config.app).args(args).spawn()?;

        // Wait for process
        info!("Waiting for {}ms", self.config.delay);
        thread::sleep(time::Duration::from_millis(self.config.delay));

        // Create timeout duration
        let timeout_duration = time::Duration::from_millis(Self::WAIT_DEBUGGING_PORT_TIMEOUT_MS);

        // Wait until remote debugging port is available
        info!("Waiting for remote debugging port");
        let start_time = time::Instant::now();
        loop {
            // Connected
            if self.get_devtool_pages().is_ok() {
                info!("Connected to remote debugging port");
                break;
            }

            // Timed out
            if start_time.elapsed() >= timeout_duration {
                bail!("Unable to connect to remote debugging port");
            }
        }

        Ok(child)
    }

    fn evaluate(&self, ws: &mut WebSocket, expression: &str) -> Result<()> {
        // Create payload
        // https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-evaluate
        let payload = json!({
            "id": 1,
            "method": "Runtime.evaluate",
            "params": {
                "expression": expression,
                "objectGroup": "inject",
                "includeCommandLineAPI": true,
                "silent": true,
                "userGesture": true,
                "awaitPromise": true,
            },
        });

        // Serialize payload to JSON
        let payload_json = serde_json::to_string(&payload)?;

        // Send message and get the result
        let result_msg = ws.send_and_receive(&payload_json)?;
        debug!("[Runtime.evaluate] Raw message: {:#?}", result_msg);

        // Ignore if not a text
        if !result_msg.is_text() {
            warn!(
                "[Runtime.evaluate] Unexpected result from WebSocket: {:#?}",
                result_msg
            );
            return Ok(());
        }

        // Convert message to text
        let result_json = result_msg.to_text()?;

        // Parse response
        let response: EvaluateResponse = serde_json::from_str(result_json)?;

        debug!("[Runtime.evaluate] Parsed response: {:#?}", response);

        // Handle exception
        if let Some(_) = response.result.exception_details {
            warn!(
                "[Runtime.evaluate] Caught exception while evaluating script: {:#?}",
                response
            );
            return Ok(());
        }

        Ok(())
    }
}
