use crate::token::{Token, TokenKind};
use crate::extract;
use std::str::CharIndices;
use std::iter::Peekable;

pub struct Lexer {
    pub position: usize,
    pub line: u32,
    pub col: u32,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            position: 0,
            line: 1,
            col: 0,
        }
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let state = Lexer::new();
    let mut tokens = Vec::new();
    let mut iter = input.char_indices().peekable();
    while let Some((t, state)) = next_token(input, &mut iter, &state) {
        tokens.push(t);
    }
    tokens
}

pub fn next_token<'a>(input: &'a str, char_index: &mut Peekable<CharIndices>, state: &Lexer) -> Option<(Token<'a>, Lexer)> {
    match char_index.peek() {
        Some(&(_, c)) => {
            match c {
                '!' => extract::bang(input, char_index, state),
                '{' => extract::open_brace(input, char_index, state),
                '}' => extract::close_brace(input, char_index, state),
                'a' ... 'z' | 'A' ... 'Z' => extract::string(input, char_index, state),
                _ => None
            }
        },
        None => None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_empty() {
        assert_eq!(tokenize(""), []);
    }

    #[test]
    fn lex_bang() {
        assert_eq!(tokenize("!"), [
                   Token::new(
                       TokenKind::Bang,
                       0,
                       1,
                       1,
                       0,
                       Some("!")
                   )
        ]);
    }

    #[test]
    fn lex_open_brace() {
        assert_eq!(tokenize("{"), [
                   Token::new(
                       TokenKind::OpenBrace,
                       0,
                       1,
                       1,
                       0,
                       Some("{")
                   )
        ]);
    }

    #[test]
    fn lex_strings() {
        assert_eq!(tokenize("text"), [
                   Token::new(
                       TokenKind::Str,
                       0,
                       3,
                       1,
                       0,
                       Some("text")
                   )
        ]);

    }
}
