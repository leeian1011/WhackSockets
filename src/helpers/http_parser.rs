use core::slice::Iter;

pub struct HttpRequest {
    raw_text: String,
}

impl HttpRequest {
    pub fn new(raw_text: String) -> Self {
        Self { raw_text }
    }

    pub fn read_lines(&mut self) -> Vec<&str> {
        self.raw_text.split("\r\n").collect::<Vec<_>>()
    }
}
