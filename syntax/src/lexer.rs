//! The [`Lexer`] is responsible for turning a string into a series of [`Token`]s. This includes all the
//! rules for valid tokens and can throw errors if it encouters a unrecognized character or other
//! logical issue.
//!
//! The [`Lexer`] is typically used as an [`Iterator`]`. It will generate tokens lazily. If an error
//! is encountered, it will short circuit the token generation.
//!
//! A valid series of tokens will start and end with [`Start`] and [`End`] respectively.
//! These tokens to not link to any character or string of characters in the input string, but are
//! there for ergonomics.
//!
//! The [`Lexer`] will ignore all whitespace (tabs, spaces, newlines), as well as all commas. This is
//! in accordance with the GraphQL Spec.
//!
//!
//! # Examples
//!
//! Here is an example with a valid string.
//!
//! ```
//! use syntax::lexer::Lexer;
//! use syntax::token::{Token, Location};
//!
//! let mut lexer = Lexer::new(r#"
//! schema Schema {
//!   query: Query,
//!   mutation: Mutation
//! }
//! "#);
//! assert_eq!(lexer.next(), Some(Ok(Token::Start)));
//! assert_eq!(lexer.next(), Some(Ok(Token::Name(Location::new(1, 2, 1), "schema"))));
//! assert_eq!(lexer.next(), Some(Ok(Token::Name(Location::new(9, 2, 9), "Schema"))));
//! assert_eq!(lexer.next(), Some(Ok(Token::OpenBrace(Location::new(11, 2, 11)))));
//! assert_eq!(lexer.next(), Some(Ok(Token::Name(Location::new(14, 3, 3), "query"))));
//! assert_eq!(lexer.next(), Some(Ok(Token::Colon(Location::new(19, 3, 8)))));
//! assert_eq!(lexer.next(), Some(Ok(Token::Name(Location::new(21, 3, 10), "Query"))));
//! assert_eq!(lexer.next(), Some(Ok(Token::Name(Location::new(25, 4, 3), "mutation"))));
//! assert_eq!(lexer.next(), Some(Ok(Token::Colon(Location::new(34, 4, 12)))));
//! assert_eq!(lexer.next(), Some(Ok(Token::Name(Location::new(36, 4, 14), "Mutation"))));
//! assert_eq!(lexer.next(), Some(Ok(Token::CloseBrace(Location::new(38, 5, 1)))));
//! assert_eq!(lexer.next(), Some(Ok(Token::End)));
//! assert_eq!(lexer.next(), None);
//! ```
//!
//! The lexer will respond as follows if it encounters an error:
//!
//! ```
//! use syntax::lexer::Lexer;
//! use syntax::error::LexError;
//! use syntax::token::{Token, Location};
//!
//! let mut lexer = Lexer::new(r#""unmatched"#);
//! assert_eq!(lexer.next(), Some(Ok(Token::Start)));
//! assert_eq!(lexer.next(), Some(Err(LexError::UnmatchedQuote(Location {
//!   absolute_position: 0,
//!   line: 1,
//!   column: 2,
//! }))));
//! assert_eq!(lexer.next(), None);
//! ```
//!
//! [`LexError`]: ../error/enum.LexError.html
//! [`Lexer`]: enum.Lexer.html
//! [`Iterator`]: ../../std/iter/trait.Iterator.html
//! [`Token`]: ../token/enum.Token.html
//! [`Start`]: ../token/enum.Token.html#variant.Start
//! [`End`]: ../token/enum.Token.html#variant.End
//!
//!

use crate::error::LexError;
use crate::token::{Location, Token};
use log::debug;
use regex::Regex;
use std::iter::Iterator;
use std::iter::Peekable;
use std::str::CharIndices;

/// A Lexer is an iterator that takes an input GraphQL string and generates a series of [`Tokens`]` or
/// [`error`]s.
///
///
/// If an [`error`]occurs, then the Lexer ends iteration.
///
/// A Lexer will also keep track of its possition in the string. This allows for more robust
/// messages about where in the string a certain token or error is.
///
/// [`Tokens`]: ../token/enum.Token.html
/// [`error`]: ../error/enum.LexError.html
#[derive(Debug)]
pub struct Lexer<'a> {
    raw: &'a str,
    input: Peekable<CharIndices<'a>>,
    initialized: bool,
    ended: bool,
    position: usize,
    line: usize,
    col: usize,
}

type LexerItem<'a> = Result<Token<'a>, LexError>;

impl<'a> Lexer<'a> {
    /// Creates a new lexer that passes over the provided input string.
    /// The token series will
    pub fn new(input: &str) -> Lexer {
        Lexer {
            raw: input,
            input: input.char_indices().peekable(),
            initialized: false,
            ended: false,
            position: 0,
            line: 1,
            col: 1,
        }
    }

    fn get_next_token(&mut self) -> LexerItem<'a> {
        if let Some((i, next)) = self.input.peek() {
            let index = *i;
            match next {
                '!' => self.lex_bang(),
                '$' => self.lex_dollar(),
                '&' => self.lex_ampersand(),
                '|' => self.lex_pipe(),
                '@' => self.lex_at(),
                ':' => self.lex_colon(),
                '=' => self.lex_equals(),
                '{' => self.lex_open_brace(),
                '}' => self.lex_close_brace(),
                '(' => self.lex_open_paren(),
                ')' => self.lex_close_paren(),
                '[' => self.lex_open_square(),
                ']' => self.lex_close_square(),
                '#' => self.ignore_comments(),
                ' ' | '\t' | ',' => self.ignore_whitespace(),
                '\n' => self.ignore_newline(),
                '"' => self.lex_string(index),
                // TODO Make this multilingual
                'a'..='z' | 'A'..='Z' => self.lex_name(index),
                // TODO Make this handle scientific notation
                '0'..='9' | '-' => self.lex_number(index),
                '.' => self.lex_ellipsis(index),
                _ => self.make_unknown_character_error(),
            }
        } else {
            // This occurs when we have hit an extra newline at the end of the file
            self.ended = true;
            Ok(Token::End)
        }
    }

    fn lex_ellipsis(&mut self, index: usize) -> LexerItem<'a> {
        lazy_static! {
            static ref SPREAD: Regex = Regex::new("...").unwrap();
        }
        if SPREAD.is_match_at(self.raw, index) {
            let cur_col = self.col;
            let cur_pos = self.position;
            self.advance_n(3);
            Ok(Token::Spread(Location::new(cur_pos, self.line, cur_col)))
        } else {
            self.make_unexpected_character_error()
        }
    }

    fn lex_number(&mut self, init_pos: usize) -> LexerItem<'a> {
        lazy_static! {
            static ref FLOAT: Regex = Regex::new(r#"-?[0-9]+\.[0-9]+"#).unwrap();
            static ref INT: Regex = Regex::new(r#"-?[0-9]+"#).unwrap();
        }
        if FLOAT.is_match_at(self.raw, init_pos) {
            let mut locations = FLOAT.capture_locations();
            match FLOAT.captures_read_at(&mut locations, self.raw, init_pos) {
                Some(_) => match locations.get(0) {
                    Some((start, end)) => {
                        let cur_col = self.col;
                        let substr = self.raw.get(start..end).unwrap();
                        match substr.parse::<f64>() {
                            Ok(f) => {
                                self.advance_to(end);
                                Ok(Token::Float(Location::new(init_pos, self.line, cur_col), f))
                            }
                            Err(_) => self.make_conversion_error("Float"),
                        }
                    }
                    None => self.make_unknown_character_error(),
                },
                None => self.make_unexpected_character_error(),
            }
        } else if INT.is_match_at(self.raw, init_pos) {
            let mut locations = INT.capture_locations();
            match INT.captures_read_at(&mut locations, self.raw, init_pos) {
                Some(_) => match locations.get(0) {
                    Some((start, end)) => {
                        let substr = self.raw.get(start..end).unwrap();
                        match substr.parse::<i64>() {
                            Ok(i) => {
                                let tok = Token::Int(self.get_current_location(), i);
                                self.advance_to(end);
                                Ok(tok)
                            }
                            Err(_) => self.make_conversion_error("Int"),
                        }
                    }
                    None => self.make_unknown_character_error(),
                },
                None => self.make_unexpected_character_error(),
            }
        } else {
            self.make_conversion_error("Int or Float")
        }
    }

    fn lex_name(&mut self, init_pos: usize) -> LexerItem<'a> {
        let mut end_pos = 0;
        while let Some((_, c)) = self.input.peek() {
            if c.is_alphanumeric() || *c == '_' {
                self.input.next();
                end_pos += 1;
            } else {
                break;
            }
        }
        self.position += end_pos;
        let init_col = self.col;
        self.col += end_pos;
        end_pos += init_pos;
        Ok(Token::Name(
            Location::new(init_pos, self.line, init_col),
            self.raw.get(init_pos..end_pos).unwrap(),
        ))
    }

    fn lex_string(&mut self, init_pos: usize) -> LexerItem<'a> {
        lazy_static! {
            static ref BLOCK_START: Regex = Regex::new(r#"""""#).unwrap();
            static ref BLOCK: Regex = Regex::new(r#""""((?:\\.|[^"\\])*)""""#).unwrap();
            static ref SINGLE: Regex = Regex::new(r#""((?:\\.|[^"\\])*)""#).unwrap();
        }
        if BLOCK_START.is_match_at(self.raw, init_pos) {
            let mut locations = BLOCK.capture_locations();
            match BLOCK.captures_read_at(&mut locations, self.raw, init_pos) {
                Some(_) => match locations.get(1) {
                    Some((start_off, end_off)) => {
                        let (start, end) = locations.get(0).unwrap();
                        match self.input.position(|(i, _)| i == end) {
                            Some(pos) => self.position = pos,
                            None => (),
                        }
                        let tok = Token::BlockStr(
                            Location::new(start, self.line, self.col),
                            self.raw.get(start_off..end_off).unwrap(),
                        );

                        let substr = self.raw.get(start..end).unwrap();
                        let newlines = substr.lines().count();
                        self.line += newlines;
                        Ok(tok)
                    }
                    None => self.make_unmatched_quote_error(),
                },
                None => self.make_unmatched_quote_error(),
            }
        } else {
            let mut locations = SINGLE.capture_locations();
            match SINGLE.captures_read_at(&mut locations, self.raw, init_pos) {
                Some(_) => match locations.get(1) {
                    Some((start_off, end_off)) => {
                        let cur_col = self.col;
                        match self.input.position(|(i, _)| i == end_off) {
                            Some(pos) => {
                                self.position += pos + 1;
                                self.col += pos + 1;
                            }
                            None => (),
                        }
                        Ok(Token::Str(
                            Location::new(init_pos, self.line, cur_col),
                            self.raw.get(start_off..end_off).unwrap(),
                        ))
                    }
                    None => self.make_unmatched_quote_error(),
                },
                None => self.make_unmatched_quote_error(),
            }
        }
    }

    fn lex_bang(&mut self) -> LexerItem<'a> {
        let tok = Ok(Token::Bang(self.get_current_location()));
        self.advance();
        tok
    }

    fn lex_dollar(&mut self) -> LexerItem<'a> {
        let tok = Ok(Token::Dollar(self.get_current_location()));
        self.advance();
        tok
    }

    fn lex_ampersand(&mut self) -> LexerItem<'a> {
        let tok = Ok(Token::Amp(self.get_current_location()));
        self.advance();
        tok
    }

    fn lex_pipe(&mut self) -> LexerItem<'a> {
        let tok = Ok(Token::Pipe(self.get_current_location()));
        self.advance();
        tok
    }

    fn lex_at(&mut self) -> LexerItem<'a> {
        let tok = Ok(Token::At(self.get_current_location()));
        self.advance();
        tok
    }

    fn lex_close_square(&mut self) -> LexerItem<'a> {
        let tok = Ok(Token::CloseSquare(self.get_current_location()));
        self.advance();
        tok
    }

    fn lex_open_square(&mut self) -> LexerItem<'a> {
        let tok = Ok(Token::OpenSquare(self.get_current_location()));
        self.advance();
        tok
    }

    fn lex_close_paren(&mut self) -> LexerItem<'a> {
        let tok = Ok(Token::CloseParen(self.get_current_location()));
        self.advance();
        tok
    }

    fn lex_open_paren(&mut self) -> LexerItem<'a> {
        let tok = Ok(Token::OpenParen(self.get_current_location()));
        self.advance();
        tok
    }

    fn lex_close_brace(&mut self) -> LexerItem<'a> {
        let tok = Ok(Token::CloseBrace(self.get_current_location()));
        self.advance();
        tok
    }

    fn lex_open_brace(&mut self) -> LexerItem<'a> {
        let tok = Ok(Token::OpenBrace(self.get_current_location()));
        self.advance();
        tok
    }

    fn lex_equals(&mut self) -> LexerItem<'a> {
        let tok = Ok(Token::Equals(self.get_current_location()));
        self.advance();
        tok
    }

    fn lex_colon(&mut self) -> LexerItem<'a> {
        let tok = Ok(Token::Colon(self.get_current_location()));
        self.advance();
        tok
    }

    fn ignore_newline(&mut self) -> LexerItem<'a> {
        self.line += 1;
        self.col = 1;
        self.position += 1;
        self.input.next();
        self.get_next_token()
    }

    fn ignore_whitespace(&mut self) -> LexerItem<'a> {
        self.advance();
        self.get_next_token()
    }

    fn ignore_comments(&mut self) -> LexerItem<'a> {
        self.input.next(); // Consume #
        if let Some((new_line_index, _new_line)) = self.input.find(|(_index, c)| *c == '\n') {
            self.advance_to(new_line_index);
        }
        self.get_next_token()
    }

    fn make_unexpected_character_error(&mut self) -> LexerItem<'a> {
        self.ended = true;
        Err(LexError::UnexpectedCharacter(self.get_current_location()))
    }

    fn make_conversion_error(&mut self, expected_type: &'static str) -> LexerItem<'a> {
        self.ended = true;
        Err(LexError::UnableToConvert(
            self.get_current_location(),
            expected_type,
        ))
    }

    fn make_unknown_character_error(&mut self) -> LexerItem<'a> {
        self.ended = true;
        Err(LexError::UnknownCharacter(self.get_current_location()))
    }

    fn make_unmatched_quote_error(&mut self) -> LexerItem<'a> {
        self.ended = true;
        Err(LexError::UnmatchedQuote(Location::new(
            self.position,
            self.line,
            self.col + 1,
        )))
    }

    fn get_current_location(&mut self) -> Location {
        Location::new(self.position, self.line, self.col)
    }

    fn advance(&mut self) {
        self.input.next();
        self.position += 1;
        self.col += 1;
    }

    fn advance_n(&mut self, n: usize) {
        self.position += n;
        let new_pos = self.position - 1;
        self.col += n;
        self.input.position(|(i, _)| i == new_pos);
    }

    fn advance_to(&mut self, pos: usize) {
        self.position = pos;
        self.col = pos;
        self.input.position(|(i, _)| i == pos - 1);
    }
}

use std::fmt;
impl<'a> fmt::Display for Lexer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Lexer<position: {}, line: {}, col: {}>",
            self.position, self.line, self.col
        )
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexerItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ended {
            None
        } else if !self.initialized {
            debug!("Uninizialized");
            self.initialized = true;
            Some(Ok(Token::Start))
        } else if let Some(_) = self.input.peek() {
            let tok = self.get_next_token();
            debug!("Next Token: {:?}", tok);
            debug!("Next char: {:?}", self.input.peek());
            Some(tok)
        } else {
            debug!("Found a None in the string: Ending? {}", self.ended);
            if !self.ended {
                self.ended = true;
                Some(Ok(Token::End))
            } else {
                None
            }
        }
    }
}

/// Destruct the string into a Vec of tokens.
///
/// # Examples
/// ```
/// use syntax::lexer::tokenize;
/// let statement = r#"type Query {
///  hero(episode: Episode): Character
///  droid(id: ID!): Droid
/// }"#;
/// let tokens = tokenize(&statement);
/// assert!(tokens.is_ok());
/// println!("Tokens: {:?}", tokens);
/// ````
pub fn tokenize(input: &str) -> Result<Vec<Token>, LexError> {
    let state = Lexer::new(input);
    let results: Result<Vec<Token>, LexError> = state.collect();
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::LexError;
    use crate::token::Token;

    #[test]
    fn lex_empty() {
        let empty = tokenize("");
        assert!(empty.is_ok());
        assert_eq!(empty.unwrap(), vec![Token::Start, Token::End,]);
    }

    #[test]
    fn lex_bang() {
        println!("Testing bang");
        let one = tokenize("!");
        assert!(one.is_ok());
        assert_eq!(
            one.unwrap(),
            vec![
                Token::Start,
                Token::Bang(Location::new(0, 1, 1)),
                Token::End,
            ]
        );
    }

    #[test]
    fn lex_dollar() {
        println!("Testing dollar");
        let one = tokenize("$");
        assert!(one.is_ok());
        assert_eq!(
            one.unwrap(),
            vec![
                Token::Start,
                Token::Dollar(Location::new(0, 1, 1)),
                Token::End,
            ]
        );
    }

    #[test]
    fn lex_ampersand() {
        println!("Testing ampersand");
        let one = tokenize("&");
        assert!(one.is_ok());
        assert_eq!(
            one.unwrap(),
            vec![Token::Start, Token::Amp(Location::new(0, 1, 1)), Token::End,]
        );
    }

    #[test]
    fn lex_at_sign() {
        println!("Testing at sign");
        let one = tokenize("@");
        assert!(one.is_ok());
        assert_eq!(
            one.unwrap(),
            vec![Token::Start, Token::At(Location::new(0, 1, 1)), Token::End,]
        );
    }

    #[test]
    fn lex_pipe() {
        println!("Testing pipe");
        let one = tokenize("|");
        assert!(one.is_ok());
        assert_eq!(
            one.unwrap(),
            vec![
                Token::Start,
                Token::Pipe(Location::new(0, 1, 1)),
                Token::End,
            ]
        );
    }

    #[test]
    fn lex_colon() {
        println!("Testing colon");
        let one = tokenize(":");
        assert!(one.is_ok());
        assert_eq!(
            one.unwrap(),
            vec![
                Token::Start,
                Token::Colon(Location::new(0, 1, 1)),
                Token::End,
            ]
        );
    }

    #[test]
    fn lex_equals() {
        println!("Testing colon");
        let one = tokenize("=");
        assert!(one.is_ok());
        assert_eq!(
            one.unwrap(),
            vec![
                Token::Start,
                Token::Equals(Location::new(0, 1, 1)),
                Token::End,
            ]
        );
    }

    #[test]
    fn lex_open_brace() {
        println!("Testing open brace");
        let one = tokenize("{");
        assert!(one.is_ok());
        assert_eq!(
            one.unwrap(),
            vec![
                Token::Start,
                Token::OpenBrace(Location::new(0, 1, 1)),
                Token::End,
            ]
        );
    }
    #[test]
    fn lex_close_brace() {
        println!("Testing close brace");
        let one = tokenize("}");
        assert!(one.is_ok());
        assert_eq!(
            one.unwrap(),
            vec![
                Token::Start,
                Token::CloseBrace(Location::new(0, 1, 1)),
                Token::End,
            ]
        );
    }

    #[test]
    fn lex_open_paren() {
        println!("Testing open parenthesis");
        let one = tokenize("(");
        assert!(one.is_ok());
        assert_eq!(
            one.unwrap(),
            vec![
                Token::Start,
                Token::OpenParen(Location::new(0, 1, 1)),
                Token::End,
            ]
        );
    }
    #[test]
    fn lex_close_paren() {
        println!("Testing close parenthasis");
        let one = tokenize(")");
        assert!(one.is_ok());
        assert_eq!(
            one.unwrap(),
            vec![
                Token::Start,
                Token::CloseParen(Location::new(0, 1, 1)),
                Token::End,
            ]
        );
    }

    #[test]
    fn lex_open_square() {
        println!("Testing open square bracket");
        let one = tokenize("[");
        assert!(one.is_ok());
        assert_eq!(
            one.unwrap(),
            vec![
                Token::Start,
                Token::OpenSquare(Location::new(0, 1, 1)),
                Token::End,
            ]
        );
    }
    #[test]
    fn lex_close_square() {
        println!("Testing close square bracket");
        let one = tokenize("]");
        assert!(one.is_ok());
        assert_eq!(
            one.unwrap(),
            vec![
                Token::Start,
                Token::CloseSquare(Location::new(0, 1, 1)),
                Token::End,
            ]
        );
    }

    #[test]
    fn lex_spread() {
        println!("Testing spread");
        let one = tokenize("...");
        println!("Debug float: {:?}", one);
        assert!(one.is_ok());
        assert_eq!(
            one.unwrap(),
            vec![
                Token::Start,
                Token::Spread(Location::new(0, 1, 1)),
                Token::End,
            ]
        );
    }

    #[test]
    fn lex_int() {
        println!("Testing int");
        let one = tokenize("123456");
        assert!(one.is_ok());
        assert_eq!(
            one.unwrap(),
            vec![
                Token::Start,
                Token::Int(Location::new(0, 1, 1), 123456i64),
                Token::End,
            ]
        );
        let one = tokenize("-9876");
        assert!(one.is_ok());
        assert_eq!(
            one.unwrap(),
            vec![
                Token::Start,
                Token::Int(Location::new(0, 1, 1), -9876i64),
                Token::End,
            ]
        );
    }

    #[test]
    fn lex_float() {
        println!("Testing float");
        let one = tokenize("1.23456789");
        assert!(one.is_ok());
        assert_eq!(
            one.unwrap(),
            vec![
                Token::Start,
                Token::Float(Location::new(0, 1, 1), 1.23456789f64),
                Token::End,
            ]
        );
        let one = tokenize("-0.987654321");
        assert!(one.is_ok());
        assert_eq!(
            one.unwrap(),
            vec![
                Token::Start,
                Token::Float(Location::new(0, 1, 1), -0.987654321f64),
                Token::End,
            ]
        );
    }

    #[test]
    fn lex_strings() {
        println!("Testing strings");
        let text = tokenize(r#""text""#);
        assert!(text.is_ok());
        assert_eq!(
            text.unwrap(),
            vec![
                Token::Start,
                Token::Str(Location::new(0, 1, 1), "text"),
                Token::End,
            ]
        );
    }

    #[test]
    fn lex_block_strings() {
        println!("Testing block strings");
        let text = tokenize(
            r#""""test

text""""#,
        );
        assert!(text.is_ok());
        assert_eq!(
            text.unwrap(),
            vec![
                Token::Start,
                Token::BlockStr(Location::new(0, 1, 1), "test\n\ntext"),
                Token::End,
            ]
        );
    }

    #[test]
    fn lex_name() {
        println!("Testing names");
        let text = tokenize("name\nname_with_underscore");
        assert!(text.is_ok());
        assert_eq!(
            text.unwrap(),
            vec![
                Token::Start,
                Token::Name(Location::new(0, 1, 1), "name"),
                Token::Name(Location::new(5, 2, 1), "name_with_underscore"),
                Token::End,
            ]
        );
    }

    #[test]
    fn lex_comment() {
        println!("Test comment");
        let comments = tokenize(
            r#"# this is a comment
# And so is this
"#,
        );
        assert!(comments.is_ok());
        assert_eq!(comments.unwrap(), vec![Token::Start, Token::End,])
    }

    #[test]
    fn lex_query() {
        println!("Test query");
        let query = tokenize(
            r#"query {
  hero {
    name
  }
  droid(id: "2000") {
    name
  }
}"#,
        );
        assert!(query.is_ok());
        assert_eq!(
            query.unwrap(),
            vec![
                Token::Start,
                Token::Name(Location::new(0, 1, 1), "query"),
                Token::OpenBrace(Location::new(6, 1, 7)),
                Token::Name(Location::new(10, 2, 3), "hero"),
                Token::OpenBrace(Location::new(15, 2, 8)),
                Token::Name(Location::new(21, 3, 5), "name"),
                Token::CloseBrace(Location::new(28, 4, 3)),
                Token::Name(Location::new(32, 5, 3), "droid"),
                Token::OpenParen(Location::new(37, 5, 8)),
                Token::Name(Location::new(38, 5, 9), "id"),
                Token::Colon(Location::new(40, 5, 11)),
                Token::Str(Location::new(42, 5, 13), "2000"),
                Token::CloseParen(Location::new(48, 5, 19)),
                Token::OpenBrace(Location::new(50, 5, 21)),
                Token::Name(Location::new(56, 6, 5), "name"),
                Token::CloseBrace(Location::new(63, 7, 3)),
                Token::CloseBrace(Location::new(65, 8, 1)),
                Token::End,
            ]
        )
    }

    #[test]
    fn lex_type() {
        let t = tokenize(
            r#"type Query {
  hero(episode: Episode): Character
  droid(id: ID!): Droid
}
"#,
        );
        println!("T in lex_type: {:?}", t);
        assert!(t.is_ok());
        assert_eq!(
            t.unwrap(),
            vec![
                Token::Start,
                Token::Name(Location::new(0, 1, 1), "type"),
                Token::Name(Location::new(5, 1, 6), "Query"),
                Token::OpenBrace(Location::new(11, 1, 12)),
                Token::Name(Location::new(15, 2, 3), "hero"),
                Token::OpenParen(Location::new(19, 2, 7)),
                Token::Name(Location::new(20, 2, 8), "episode"),
                Token::Colon(Location::new(27, 2, 15)),
                Token::Name(Location::new(29, 2, 17), "Episode"),
                Token::CloseParen(Location::new(36, 2, 24)),
                Token::Colon(Location::new(37, 2, 25)),
                Token::Name(Location::new(39, 2, 27), "Character"),
                Token::Name(Location::new(51, 3, 3), "droid"),
                Token::OpenParen(Location::new(56, 3, 8)),
                Token::Name(Location::new(57, 3, 9), "id"),
                Token::Colon(Location::new(59, 3, 11)),
                Token::Name(Location::new(61, 3, 13), "ID"),
                Token::Bang(Location::new(63, 3, 15)),
                Token::CloseParen(Location::new(64, 3, 16)),
                Token::Colon(Location::new(65, 3, 17)),
                Token::Name(Location::new(67, 3, 19), "Droid"),
                Token::CloseBrace(Location::new(73, 4, 1)),
                Token::End,
            ]
        )
    }

    #[test]
    fn lex_fragment() {
        let fragment = tokenize(
            r#"query {
  hero {
    name
    ... on Human {
      height
    }
  }
}"#,
        );
        assert!(fragment.is_ok());
        assert_eq!(
            fragment.unwrap(),
            vec![
                Token::Start,
                Token::Name(Location::new(0, 1, 1), "query"),
                Token::OpenBrace(Location::new(6, 1, 7)),
                Token::Name(Location::new(10, 2, 3), "hero"),
                Token::OpenBrace(Location::new(15, 2, 8)),
                Token::Name(Location::new(21, 3, 5), "name"),
                Token::Spread(Location::new(30, 4, 5)),
                Token::Name(Location::new(34, 4, 9), "on"),
                Token::Name(Location::new(37, 4, 12), "Human"),
                Token::OpenBrace(Location::new(43, 4, 18)),
                Token::Name(Location::new(51, 5, 7), "height"),
                Token::CloseBrace(Location::new(62, 6, 5)),
                Token::CloseBrace(Location::new(66, 7, 3)),
                Token::CloseBrace(Location::new(68, 8, 1)),
                Token::End,
            ]
        );
    }

    #[test]
    fn handles_unmatched_quote() {
        let err = tokenize("\"test");
        assert!(err.is_err());
        assert_eq!(
            err.unwrap_err(),
            LexError::UnmatchedQuote(Location::new(0, 1, 2))
        );
        let err = tokenize("\"\"\"test\n\n\"\"");
        assert!(err.is_err());
        assert_eq!(
            err.unwrap_err(),
            LexError::UnmatchedQuote(Location::new(0, 1, 2))
        );
        let err = tokenize("\"\"\"test\ntest\ntest\"");
        assert!(err.is_err());
        assert_eq!(
            err.unwrap_err(),
            LexError::UnmatchedQuote(Location::new(0, 1, 2))
        );
    }

    #[test]
    fn handles_unknown_character() {
        let err = tokenize("%");
        assert!(err.is_err());
        assert_eq!(
            err.unwrap_err(),
            LexError::UnknownCharacter(Location::new(0, 1, 1))
        );
    }

    #[test]
    fn ignores_commas() {
        let query = tokenize(
            "{
  one,
  two,
  three,,,
}",
        );
        assert!(query.is_ok());
        assert_eq!(
            query.unwrap(),
            vec![
                Token::Start,
                Token::OpenBrace(Location::new(0, 1, 1)),
                Token::Name(Location::new(4, 2, 3), "one"),
                Token::Name(Location::new(10, 3, 3), "two"),
                Token::Name(Location::new(17, 4, 3), "three"),
                Token::CloseBrace(Location::new(26, 5, 1)),
                Token::End,
            ]
        );
    }

    #[test]
    fn handles_text_after_strings() {
        let strings = tokenize(
            r#"
"""
This is a generic object comment
They can be multiple lines
"""
type Obj {
  "This is the name of the object"
  name: String
}
"#,
        );
        println!("Strings: {:?}", strings);
        assert!(strings.is_ok());
        assert_eq!(
            strings.unwrap(),
            vec![
                Token::Start,
                Token::BlockStr(
                    Location::new(1, 2, 1),
                    r#"
This is a generic object comment
They can be multiple lines
"#
                ),
                Token::Name(Location::new(70, 6, 1), "type"),
                Token::Name(Location::new(75, 6, 6), "Obj"),
                Token::OpenBrace(Location::new(79, 6, 10)),
                Token::Str(Location::new(83, 7, 3), "This is the name of the object"),
                Token::Name(Location::new(108, 8, 3), "name"),
                Token::Colon(Location::new(112, 8, 7)),
                Token::Name(Location::new(114, 8, 9), "String"),
                Token::CloseBrace(Location::new(121, 9, 1)),
                Token::End,
            ]
        );
    }

    #[test]
    fn handles_multiple_block_strings() {
        let strings = tokenize(
            r#"
"""
This is a multiline string
"""
name
"""Followed by a single line"""
id
"""
And a final multiline string
"""
"#,
        );
        assert!(strings.is_ok());
        assert_eq!(
            strings.unwrap(),
            vec![
                Token::Start,
                Token::BlockStr(Location::new(1, 2, 1), "\nThis is a multiline string\n"),
                Token::Name(Location::new(36, 5, 1), "name"),
                Token::BlockStr(Location::new(41, 6, 1), "Followed by a single line"),
                Token::Name(Location::new(73, 7, 1), "id"),
                Token::BlockStr(Location::new(76, 8, 1), "\nAnd a final multiline string\n"),
                Token::End,
            ]
        )
    }
}
