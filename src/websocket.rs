use std::net::TcpStream;

use log::{debug, info};
use tungstenite::{stream::MaybeTlsStream, Message};

pub struct WebSocket {
    address: String,
    socket: tungstenite::WebSocket<MaybeTlsStream<TcpStream>>,
}

impl WebSocket {
    pub fn connect(address: &str) -> Result<Self, tungstenite::Error> {
        let url = url::Url::parse(&address).expect("Should be a valid address");

        let (socket, response) = tungstenite::connect(url)?;
        info!("WebSocket connected (status: {})", response.status());

        debug!("Response headers: {:#?}", response.headers());

        Ok(WebSocket {
            address: String::from(address),
            socket,
        })
    }

    pub fn send(&mut self, msg: &str) -> Result<(), tungstenite::Error> {
        self.socket
            .write_message(Message::Text(String::from(msg)))?;

        Ok(())
    }

    pub fn receive(&mut self) -> Result<Message, tungstenite::Error> {
        let msg = self.socket.read_message()?;

        Ok(msg)
    }

    pub fn send_and_receive(&mut self, msg: &str) -> Result<Message, tungstenite::Error> {
        self.send(msg)?;
        self.receive()
    }

    pub fn close(&mut self) -> Result<(), tungstenite::Error> {
        self.socket.close(None)
    }
}

impl Drop for WebSocket {
    fn drop(&mut self) {
        debug!("Closing WebSocket (address: {:?})", self.address);
        _ = self.close();
    }
}
