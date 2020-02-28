// use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum ExtractErrorKind {
    UnmatchQuote { line: usize, col: usize },
    UnknownCharacter { line: usize, col: usize },
    UnhandledCase,
    // Custom(&'static str)
}

