#[derive(Debug, PartialEq, Clone)]
pub enum WhitespaceType {
    Newline,
    Space,
    Tab,
}

// (position, line, col, value)
#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    Start,
    End,
    Bang(usize, u32, u32),
    Dollar(usize, u32, u32),
    Amp(usize, u32, u32),
    OpenParen(usize, u32, u32),
    CloseParen(usize, u32, u32),
    Spread(usize, u32, u32),
    Colon(usize, u32, u32),
    Equals(usize, u32, u32),
    At(usize, u32, u32),
    OpenSquare(usize, u32, u32),
    CloseSquare(usize, u32, u32),
    OpenBrace(usize, u32, u32),
    CloseBrace(usize, u32, u32),
    Pipe(usize, u32, u32),
    Name(usize, u32, u32, &'a str),
    Int(usize, u32, u32, i64),
    Float(usize, u32, u32, f64),
    Str(usize, u32, u32, &'a str),
    BlockStr(usize, u32, u32, &'a str),
    Comment(usize, u32, u32, &'a str),
    Whitespace(usize, u32, u32, WhitespaceType),
}

impl<'a> Token<'a> {
    pub fn new() -> Token<'a> {
        Token::Start
    }
}

use std::fmt;

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // match self {
        //     Start | End =>
        // }
        write!(f, "Token<{:?}>", self)
    }
}

