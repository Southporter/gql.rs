use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};
use std::str::CharIndices;
use std::iter::Peekable;

pub fn bang<'a>(input: &'a str, char_index: &mut Peekable<CharIndices>, state: &Lexer) -> Option<(Token<'a>, Lexer)> {
    if let Some((index, _)) = char_index.next() {
        let old_col = state.col;
        let new_index = index + 1;
        Some((Token::new(
                TokenKind::Bang,
                index,
                new_index,
                state.line,
                old_col,
                Some(&input[index..new_index])
        ), Lexer {
            position: new_index,
            line: state.line,
            col: state.col + 1,
        }))

    } else {
        None
    }
}

pub fn open_brace<'a>(input: &'a str, char_index: &mut Peekable<CharIndices>, state: &Lexer) -> Option<(Token<'a>, Lexer)> {
    Some((
        Token::new(
            TokenKind::OpenBrace,
            0,
            0,
            0,
            0,
            Some("}")
        ),
        Lexer {
            position: state.position,
            line: state.line,
            col: state.col
        }
    ))
}

pub fn close_brace<'a>(input: &'a str, char_index: &mut Peekable<CharIndices>, state: &Lexer) -> Option<(Token<'a>, Lexer)> {
    Some((Token::new(
            TokenKind::CloseBrace,
            0,
            0,
            0,
            0,
            Some("}")
        ),
        Lexer {
            position: state.position,
            line: state.line,
            col: state.col
        }
    ))
}

pub fn string<'a>(input: &'a str, char_index: &mut Peekable<CharIndices>, state: &Lexer) -> Option<(Token<'a>, Lexer)> {
    None
}

