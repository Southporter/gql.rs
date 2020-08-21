// (position, line, col, value)
#[derive(Debug, Clone)]
pub enum Token<'a> {
    Start,
    End,
    Bang(usize, usize, usize),
    Dollar(usize, usize, usize),
    Amp(usize, usize, usize),
    OpenParen(usize, usize, usize),
    CloseParen(usize, usize, usize),
    Spread(usize, usize, usize),
    Colon(usize, usize, usize),
    Equals(usize, usize, usize),
    At(usize, usize, usize),
    OpenSquare(usize, usize, usize),
    CloseSquare(usize, usize, usize),
    OpenBrace(usize, usize, usize),
    CloseBrace(usize, usize, usize),
    Pipe(usize, usize, usize),
    Name(usize, usize, usize, &'a str),
    Int(usize, usize, usize, i64),
    Float(usize, usize, usize, f64),
    Str(usize, usize, usize, &'a str),
    BlockStr(usize, usize, usize, &'a str),
    Comment(usize, usize, usize, &'a str),
}

use std::mem;

impl<'a> Token<'a> {
    pub fn new() -> Token<'a> {
        Token::Start
    }

    pub fn is_same_type(&self, other: &Token) -> bool {
        return mem::discriminant(self) == mem::discriminant(other);
    }
}

use std::fmt;

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Token<{:?}>", self)
    }
}

use std::cmp::{Eq, PartialEq};

impl<'a> PartialEq for Token<'a> {
    fn eq(&self, other: &Token) -> bool {
        match self {
            Token::Name(_, _, _, value) => {
                matches!(other, Token::Name(_, _, _, value2) if *value2 == *value)
            }
            Token::Str(_, _, _, value) => {
                matches!(other, Token::Str(_, _, _, value2) if *value2 == *value)
            }
            Token::BlockStr(_, _, _, value) => {
                matches!(other, Token::BlockStr(_, _, _, value2) if *value2 == *value)
            }
            Token::Int(_, _, _, value) => {
                matches!(other, Token::Int(_, _, _, value2) if value2 == value)
            }
            Token::Float(_, _, _, value) => {
                matches!(other, Token::Float(_, _, _, value2) if value2 == value)
            }
            _ => mem::discriminant(self) == mem::discriminant(other),
        }
    }
}

impl<'a> Eq for Token<'a> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_type() {
        assert!(Token::Start == Token::Start);
        assert!(Token::End != Token::Start);
        assert!(Token::Bang(0, 0, 0) == Token::Bang(12, 2, 3));
        assert!(Token::Amp(0, 0, 0) != Token::Float(0, 0, 0, 0.0));
        assert!(Token::Dollar(0, 0, 0) != Token::OpenBrace(0, 1, 1));
        assert!(Token::Int(0, 0, 0, 0) != Token::Float(0, 0, 0, 0.0));
    }

    #[test]
    fn compare_value() {
        assert!(Token::Int(0, 0, 0, 10) == Token::Int(12, 3, 14, 10));
        assert!(Token::Float(0, 0, 0, 3.14) == Token::Float(3, 1, 4, 3.14));
        assert!(Token::Name(0, 0, 0, "id") == Token::Name(3, 3, 3, "id"));
        assert!(Token::Str(0, 0, 0, "Comment") == Token::Str(1, 2, 1, "Comment"));
        assert!(Token::BlockStr(0, 0, 0, "Comment") == Token::BlockStr(1, 2, 1, "Comment"));

        assert!(Token::Int(0, 0, 0, 10) != Token::Int(12, 3, 14, 11));
        assert!(Token::Float(0, 0, 0, 3.14) != Token::Float(3, 1, 4, 3.14159));
        assert!(Token::Name(0, 0, 0, "id") != Token::Name(3, 3, 3, "val"));
        assert!(Token::Str(0, 0, 0, "Comment") != Token::Str(1, 2, 1, "Your comment here"));
        assert!(
            Token::BlockStr(0, 0, 0, "Comment") != Token::BlockStr(1, 2, 1, "Your comment here")
        );
    }
}
