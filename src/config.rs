use std::ops::RangeInclusive;

use clap::Parser;

const PORT_RANGE: RangeInclusive<usize> = 1..=65535;

fn validate_port(s: &str) -> Result<u16, String> {
    let port: usize = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a port number"))?;
    if PORT_RANGE.contains(&port) {
        Ok(port as u16)
    } else {
        Err(format!(
            "port not in range {}-{}",
            PORT_RANGE.start(),
            PORT_RANGE.end()
        ))
    }
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Path to the electron app
    #[arg()]
    pub app: String,

    /// Additional arg for the electron app
    #[arg(short, long)]
    pub arg: Vec<String>,

    /// Path to the javascript file to be injected
    #[arg(short, long)]
    pub script: Vec<String>,

    /// The remote debugging host
    #[arg(long, default_value_t = String::from("127.0.0.1"))]
    pub host: String,

    /// The remote debugging port
    #[arg(short, long, default_value_t = 8315, value_parser = validate_port)]
    pub port: u16,

    /// Timeout in ms for injecting scripts
    #[arg(short, long, default_value_t = 10_000)]
    pub timeout: u64,

    /// Delay in ms to wait after spawning the process
    #[arg(short, long, default_value_t = 10_000)]
    pub delay: u64,

    /// Enable prelude script
    #[arg(long)]
    pub prelude: bool,
}

impl Config {
    pub fn parse_auto() -> Config {
        Config::parse()
    }
}
