use std::error::Error;
use std::fmt;
#[derive(Debug)]
pub struct CodeError {
    line: u32,
    place: String,
    message: String,
}

impl fmt::Display for CodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[line {}] Error {}: {}",
            self.line, self.place, self.message
        )
    }
}

impl Error for CodeError {}

impl CodeError {
    pub fn new(line: u32, place: String, message: String) -> CodeError {
        CodeError {
            line,
            place,
            message,
        }
    }
}
