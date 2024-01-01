use crate::helpers::{error::WhackError, http_parser::HttpParser};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

const BADREQUEST_RESPONSE: &'static str = "HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\nthis is a educational implementation of the WebSocket protocol \
    and it only supports simple parsing";

const HANDSHAKE_RESPONSE: &'static str = "HTTP/1.1 101 Switching Protocols\r\nConnection: Upgrade\r\nUpgrade: websocket\r\nSec-WebSocket-Accept: ";

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
                self.stream.write_all(BADREQUEST_RESPONSE.as_bytes())?;
                return Err(Box::new(WhackError::BadRequest));
            }
        };

        self.stream.write_all(
            format!("{}{}\r\n\r\n", HANDSHAKE_RESPONSE, parser.accept_key()).as_bytes(),
        )?;
        Ok(())
    }

    pub fn recv(&mut self, buffer: &mut [u8]) -> Result<(), Box<dyn std::error::Error>> {
        let a: u8 = 0b11001001;
        let b: u8 = 0b11001001;
        let c: u8 = 0b10010010;
        let x = a.count_zeros() + a.count_ones();
        println!("test: {}", a & 1);

        println!("what shows here {}, prev: {}, sameas: {}", a << 1, b, c);

        println!("{x}");

        _ = self.stream.read(buffer)?;
        println!("{}", buffer[2].count_ones() + buffer[2].count_zeros());
        Ok(())
    }
}
