mod helpers;
use helpers::{error::WhackError, http_parser::HttpRequest};
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

pub struct WebSocketServer {
    connection: TcpListener,
}

impl WebSocketServer {
    pub fn new(port: u16) -> Result<Self, Box<dyn std::error::Error>> {
        if port > 9999 {
            return Err(Box::new(WhackError::new("Invalid port number")));
        }

        let host = format!("127.0.0.1:{port}");
        Ok(Self {
            connection: TcpListener::bind(host)?,
        })
    }

    pub fn listen(&self) -> Result<WebSocketConnection, Box<dyn std::error::Error>> {
        let (stream, _) = self.connection.accept()?;
        println!("got a message ");
        let mut ws = WebSocketConnection::new(stream);
        println!("hiya after message");
        match ws.init_handshake() {
            Ok(()) => {}
            Err(e) => println!("{:?}", e),
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
        println!("inside init");
        let mut s = String::new();
        let _ = self.stream.read_to_string(&mut s)?;
        println!("after read to stri");
        println!("got a message {s}");
        let mut x = HttpRequest::new(s);
        for line in x.read_lines() {
            println!("{line}");
        }
        Ok(())
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

        while let Ok(_) = x.listen() {}
    }
}
