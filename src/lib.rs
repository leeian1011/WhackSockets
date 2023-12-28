mod helpers;
use helpers::{error::WhackError, http_parser::HttpParser};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

const BADREQUEST_RESPONSE: &'static str = "HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\nthis is a educational implementation of the WebSocket protocol \
    and it only supports simple parsing";

const HANDSHAKE_RESPONSE: &'static str = "HTTP/1.1 101 Switching Protocols\r\nConnection: Upgrade\r\nUpgrade: websocket\r\nSec-WebSocket-Accept: ";

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

pub struct WebSocketConnection {
    stream: TcpStream,
}

impl WebSocketConnection {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    pub fn init_handshake(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer: [u8; 1024] = [0; 1024];
        let _ = self.stream.read(&mut buffer)?;

        let mut parser = match HttpParser::new(String::from_utf8_lossy(&buffer).to_string()) {
            Ok(parser) => parser,
            Err(_) => {
                println!("writing bad req");
                self.stream.write_all(BADREQUEST_RESPONSE.as_bytes())?;
                return Err(Box::new(WhackError::BadRequest));
            }
        };

        println!("writing good req");
        self.stream.write_all(
            format!("{}{}\r\n\r\n", HANDSHAKE_RESPONSE, parser.accept_key()).as_bytes(),
        )?;
        Ok(())
    }

    pub fn recv(&mut self) {
        let mut buffer = [0u8; 1024];

        _ = self.stream.read(&mut buffer);
        println!("{:?}", buffer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x = match WebSocketServer::new(1234) {
            Ok(x) => x,
            Err(e) => {
                println!("{:?}", e);
                return;
            }
        };

        while let Ok(mut ws) = x.listen() {
            println!("connected");
            ws.recv();
        }
    }
}
