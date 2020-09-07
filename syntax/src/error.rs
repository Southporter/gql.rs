use crate::lexer::LexErrorKind;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    BadValue,
    DocumentEmpty,
    ArgumentEmpty,
    ObjectEmpty,
    EOF,
    LexError(LexErrorKind),
    UnexpectedToken { expected: String, received: String },
    UnexpectedKeyword { expected: String, received: String },
    NotImplemented,
}

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub struct ValidationError {
    pub message: String,
}

impl ValidationError {
    pub fn new(message: &str) -> ValidationError {
        ValidationError {
            message: String::from(message),
        }
    }
}
