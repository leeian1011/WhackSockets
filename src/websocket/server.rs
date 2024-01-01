use super::connection::WebSocketConnection;
use crate::helpers::error::WhackError;
use std::net::TcpListener;

pub struct WebSocketServer {
    connection: TcpListener,
}

impl WebSocketServer {
    pub fn new(port: u16) -> Result<Self, Box<dyn std::error::Error>> {
        if port > 9999 {
            return Err(Box::new(WhackError::InvalidPort));
        }

        let host = format!("127.0.0.1:{port}");
        Ok(Self {
            connection: TcpListener::bind(host)?,
        })
    }

    pub fn listen(&self) -> Result<WebSocketConnection, Box<dyn std::error::Error>> {
        let (stream, _) = self.connection.accept()?;
        let mut ws = WebSocketConnection::new(stream);
        match ws.init_handshake() {
            Ok(()) => {}
            Err(e) => println!("err = {:?}", e),
        };
        Ok(ws)
    }
}
