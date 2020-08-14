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
