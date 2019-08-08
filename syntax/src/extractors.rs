use crate::token::{Token, TokenKind};
use crate::lexer::Lexer;
use std::str::CharIndices;
use std::iter::Peekable;

pub fn extract_string<'a>(lex: Lexer<'a>, char_index: &mut Peekable<CharIndices>) -> Option<Token<'a>> {
    println!("Extracting a string");
    None
}

pub fn extract_string<'a>(lex: Lexer<'a>, char_index: &mut Peekable<CharIndices>) -> Option<Token<'a>> {
    let (index, c) = char_index.next();
    Token::new(
        TokenKind::Bang,
    )
}
