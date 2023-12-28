use base64::Engine;
use crypto::{digest::Digest, sha1::Sha1};

use super::error::WhackError;

pub struct HttpParser {
    websocket_key: String,
}

impl HttpParser {
    pub fn new(raw_text: String) -> Result<Self, Box<dyn std::error::Error>> {
        let request = raw_text.split("\r\n").collect::<Vec<_>>();
        let mut request = request.iter();
        let http_method_line = request.next().ok_or(WhackError::BadRequest)?;
        if !is_required_line(http_method_line.trim_end()) {
            return Err(Box::new(WhackError::BadRequest));
        }

        while let Some(line) = request.next() {
            let mut key_value_pair = line.split(':').collect::<Vec<_>>();
            if !is_required_line(key_value_pair[0]) {
                continue;
            }
            let value = key_value_pair
                .pop()
                .expect("key_value_pairs should have been parsed into a Vector with length of 2");
            println!("{:?}", value.trim().to_string());

            return Ok(Self {
                websocket_key: value.trim().to_string(),
            });
        }

        Err(Box::new(WhackError::BadRequest))
    }

    pub fn accept_key(&mut self) -> String {
        self.websocket_key
            .push_str("258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
        // let x = format!("6TTLLu6eTuQUA5O5GQ9I2w==258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
        println!("{}", self.websocket_key);
        let mut hasher = Sha1::new();
        hasher.input_str(&self.websocket_key);
        let mut b: [u8; 20] = [0u8; 20];
        hasher.result(&mut b);
        base64::engine::general_purpose::STANDARD
            .encode(b.to_vec())
            .to_string()
    }
}

fn is_required_line(key: &str) -> bool {
    match key {
        "Sec-WebSocket-Key" | "GET / HTTP/1.1" => true,
        _ => false,
    }
}
