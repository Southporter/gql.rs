#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Start,
    End,
    Bang,
    Dollar,
    Amp,
    OpenParen,
    CloseParen,
    Spread,
    Colon,
    Equals,
    At,
    OpenSquare,
    CloseSquare,
    OpenBrace,
    CloseBrace,
    Pipe,
    Name,
    Int,
    Float,
    Str,
    BlockStr,
    Comment,
}

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    kind: TokenKind,
    start: usize,
    end: usize,
    line: u32,
    column: u32,
    value: Option<&'a str>,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, start: usize, end: usize, line: u32, column: u32, value: Option<&'a str>) -> Token<'a> {
        Token {
            kind,
            start,
            end,
            line,
            column,
            value,
        }
    }
}

