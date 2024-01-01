use crate::helpers::{error::WhackError, http_parser::HttpParser};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

const BADREQUEST_RESPONSE: &'static str = "HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\nthis is a educational implementation of the WebSocket protocol \
    and it only supports simple parsing";

const HANDSHAKE_RESPONSE: &'static str = "HTTP/1.1 101 Switching Protocols\r\nConnection: Upgrade\r\nUpgrade: websocket\r\nSec-WebSocket-Accept: ";

pub struct FrameData {
    content_length: usize,
    opcode: u8,
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
        _ = self.stream.read(buffer)?;
        WebSocketConnection::decode_frame(buffer);

        Ok(())
    }

    fn decode_frame(frame: &mut [u8]) -> FrameData {
        let second_byte = frame[1];
        let coded_length = second_byte & 0b111_1111;

        let content_length: usize = if coded_length <= 125 {
            coded_length as usize
        } else if coded_length == 126 {
            let third_byte = frame[3];
            let fourth_byte = frame[4];
            (third_byte as u16 + fourth_byte as u16) as usize
        } else {
            let third_byte = frame[3];
            let fourth_byte = frame[4];
            (third_byte as u64 + fourth_byte as u64) as usize
        };

        println!("content_length: {}", content_length);
        FrameData {
            content_length: 0,
            opcode: 0,
        }
    }
}
