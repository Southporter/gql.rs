use crate::lexer::LexErrorKind;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    BadValue,
    DocumentEmpty,
    ArgumentEmpty,
    EOF,
    LexError(LexErrorKind),
    UnexpectedToken { expected: String, received: String },
    NotImplemented,
}

pub type ParseResult<T> = Result<T, ParseError>;
