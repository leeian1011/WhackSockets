use std::error::Error;

#[derive(Debug)]
pub enum WhackError {
    BadRequest,
    InvalidPort,
}

impl std::fmt::Display for WhackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Error for WhackError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        match self {
            WhackError::BadRequest => "unparseable http request",
            WhackError::InvalidPort => "port in use?",
        }
    }
}
