// use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum ExtractErrorKind {
    UnmatchQuote { line: u32, col: u32 },
    UnknownCharacter { line: u32, col: u32 },
    UnhandledCase,
    Custom(&'static str)
}

