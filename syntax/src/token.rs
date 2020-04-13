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

impl<'a> Token<'a> {
    pub fn new() -> Token<'a> {
        Token::Start
    }
}

use std::fmt;

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Token<{:?}>", self)
    }
}

use std::mem;
use std::cmp::PartialEq;

impl<'a> PartialEq for Token<'a> {
    fn eq(&self, other: &Token) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}



