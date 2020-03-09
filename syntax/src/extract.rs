// use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum ExtractErrorKind {
    UnmatchQuote { line: usize, col: usize },
    UnknownCharacter { line: usize, col: usize },
    UnhandledCase,
    UnexpectedCharacter { line: usize, col: usize },
    UnableToConvert { line: usize, col: usize },
    EOF { line: usize, col: usize }
    // Custom(&'static str)
}

