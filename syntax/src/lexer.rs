use crate::token::{Token, WhitespaceType};
use std::str::CharIndices;
use std::iter::Peekable;
use std::iter::Iterator;
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum LexErrorKind {
    UnmatchQuote { line: usize, col: usize },
    UnknownCharacter { line: usize, col: usize },
    UnhandledCase,
    UnexpectedCharacter { line: usize, col: usize },
    UnableToConvert { line: usize, col: usize },
    EOF { line: usize, col: usize }
    // Custom(&'static str)
}

#[derive(Debug)]
pub struct Lexer<'a> {
    raw: &'a str,
    input: Peekable<CharIndices<'a>>,
    initialized: bool,
    ended: bool,
    pub position: usize,
    pub line: usize,
    pub col: usize,
}

impl<'a> Lexer<'a> {
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

    fn advance(&mut self) {
        self.input.next();
        self.position += 1;
        self.col += 1;
    }

    fn advance_n(&mut self, n: usize) {
        self.position += n;
        let new_pos = self.position - 1;
        self.col += n;
        self.input.position(|(i, _)| { println!("Advance N: {} of {}", i, n); i == new_pos });
    }
    fn advance_to(&mut self, pos: usize) {
        self.position = pos;
        self.col = pos;
        self.input.position(|(i, _)| i == pos - 1);
    }
    // TODO Move the body of the match arms into methods here
}

use std::fmt;
impl<'a> fmt::Display for Lexer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lexer<position: {}, line: {}, col: {}>", self.position, self.line, self.col)
    }
}


impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, LexErrorKind>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.initialized {
            println!("Uninizialized");
            self.initialized = true;
            Some(Ok(Token::Start))
        } else if let Some((index, next)) = self.input.peek() {
            println!("Next: {}, index: {}", next, index);
            let tok = match next {
                '!' => {
                    let tok = Ok(Token::Bang(self.position, self.line, self.col));
                    self.advance();
                    tok
                },
                '$' => {
                    let tok = Ok(Token::Dollar(self.position, self.line, self.col));
                    self.advance();
                    tok
                },
                '&' => {
                    let tok = Ok(Token::Amp(self.position, self.line, self.col));
                    self.advance();
                    tok
                },
                '|' => {
                    let tok = Ok(Token::Pipe(self.position, self.line, self.col));
                    self.advance();
                    tok
                },
                '@' => {
                    let tok = Ok(Token::At(self.position, self.line, self.col));
                    self.advance();
                    tok
                },
                ':' => {
                    let tok = Ok(Token::Colon(self.position, self.line, self.col));
                    self.advance();
                    tok
                },
                '{' => {
                    let tok = Ok(Token::OpenBrace(self.position, self.line, self.col));
                    self.advance();
                    tok
                },
                '}' => {
                    let tok = Ok(Token::CloseBrace(self.position, self.line, self.col));
                    self.advance();
                    tok
                },
                '(' => {
                    let tok = Ok(Token::OpenParen(self.position, self.line, self.col));
                    self.advance();
                    tok
                },
                ')' => {
                    let tok = Ok(Token::CloseParen(self.position, self.line, self.col));
                    self.advance();
                    tok
                },
                '[' => {
                    let tok = Ok(Token::OpenSquare(self.position, self.line, self.col));
                    self.advance();
                    tok
                },
                ']' => {
                    let tok = Ok(Token::CloseSquare(self.position, self.line, self.col));
                    self.advance();
                    tok
                },
                ' ' => {
                    let tok = Ok(Token::Whitespace(self.position, self.line, self.col, WhitespaceType::Space));
                    self.advance();
                    tok
                },
                '\t' => {
                    let tok = Ok(Token::Whitespace(self.position, self.line, self.col, WhitespaceType::Tab));
                    self.advance();
                    tok
                },
                '\n' => {
                    let cur_pos  = self.position;
                    let cur_line = self.line;
                    let cur_col  = self.col;
                    self.line += 1;
                    self.col = 1;
                    self.position += 1;
                    self.input.next();
                    Ok(Token::Whitespace(cur_pos, cur_line, cur_col, WhitespaceType::Newline))
                },
                '"' => {
                    lazy_static! {
                        static ref BLOCK_START: Regex = Regex::new(r#"""""#).unwrap();
                        static ref BLOCK: Regex = Regex::new(r#""""((?:\\.|[^"\\])*)""""#).unwrap();
                        static ref SINGLE: Regex = Regex::new(r#""((?:\\.|[^"\\])*)""#).unwrap();
                    }
                    if BLOCK_START.is_match_at(self.raw, *index) {
                        let init_pos = *index;
                        let mut locations = BLOCK.capture_locations();
                        match BLOCK.captures_read_at(&mut locations, self.raw, init_pos) {
                            Some(_) => {
                                match locations.get(1) {
                                    Some((start_off, end_off)) => {
                                        match self.input.position(|(i, _)| i == init_pos + end_off + 3) {
                                            Some(pos) => self.position = pos,
                                            None => ()
                                        }
                                        Ok(Token::BlockStr(init_pos, self.line, self.col, self.raw.get(start_off..end_off).unwrap()))
                                    },
                                    None => Err(LexErrorKind::UnmatchQuote { line: self.line, col: self.col + 1 }),
                                }
                            },
                            None => Err(LexErrorKind::UnmatchQuote { line: self.line, col: self.col + 1 }),
                        }
                    } else {
                        let init_pos = *index;
                        let mut locations = SINGLE.capture_locations();
                        match SINGLE.captures_read_at(&mut locations, self.raw, init_pos) {
                            Some(_) => {
                                match locations.get(1) {
                                    Some((start_off, end_off)) => {
                                        println!("Single: init: {}, end: {}", init_pos, end_off);
                                        let cur_col = self.col;
                                        match self.input.position(|(i, _)| i == end_off) {
                                            Some(pos) => {
                                                self.position += pos + 1;
                                                self.col += pos + 1;
                                            },
                                            None => ()
                                        }
                                        Ok(Token::Str(init_pos, self.line, cur_col, self.raw.get(start_off..end_off).unwrap()))
                                    },
                                    None => Err(LexErrorKind::UnmatchQuote { line: self.line, col: self.col + 1 }),
                                }
                            },
                            None => Err(LexErrorKind::UnmatchQuote { line: self.line, col: self.col + 1 }),
                        }
                    }
                }
                // TODO Make this multilingual
                'a' ..= 'z' | 'A' ..= 'Z' => {
                    let init_pos = *index;
                    let mut end_pos = 0;
                    while let Some((_, c)) = self.input.peek() {
                        if c.is_alphanumeric() {
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
                    Ok(Token::Name(init_pos, self.line, init_col, self.raw.get(init_pos..end_pos).unwrap()))
                },
                // TODO Make this handle scientific notation
                '1' ..= '9' | '-' => {
                    lazy_static! {
                        static ref FLOAT: Regex = Regex::new(r#"-?[0-9]+\.[0-9]+"#).unwrap();
                        static ref INT: Regex = Regex::new(r#"-?[0-9]+"#).unwrap();
                    }
                    println!("found number");
                    if FLOAT.is_match_at(self.raw, *index) {
                        let init_pos = *index;
                        println!("found float at {}", init_pos);
                        let mut locations = FLOAT.capture_locations();
                        match FLOAT.captures_read_at(&mut locations, self.raw, init_pos) {
                            Some(_) => {
                                println!("float locations: {:?}, {:?}", locations, locations.get(1));
                                match locations.get(0) {
                                    Some((start, end)) => {
                                        println!("Start and end of float: {} - {}", start, end);
                                        let cur_col = self.col;
                                        let substr = self.raw.get(start..end).unwrap();
                                        println!("Float as string: {}", substr);
                                        match substr.parse::<f64>() {
                                            Ok(f) => {
                                                self.advance_to(end);
                                                Ok(Token::Float(init_pos, self.line, cur_col, f))
                                            },
                                            Err(_) => Err(LexErrorKind::UnableToConvert { line: self.line, col: self.col }),
                                        }
                                    },
                                    None => Err(LexErrorKind::UnableToConvert { line: self.line, col: self.col }),
                                }
                            },
                            None => Err(LexErrorKind::UnexpectedCharacter { line: self.line, col: self.col })
                        }
                    } else if INT.is_match_at(self.raw, *index) {
                        let init_pos = *index;
                        println!("found int at {}", init_pos);
                        let mut locations = INT.capture_locations();
                        match INT.captures_read_at(&mut locations, self.raw, init_pos) {
                            Some(_) => {
                                println!("found int at {}", init_pos);
                                match locations.get(0) {
                                    Some((start, end)) => {
                                        println!("Start and end of int: {} - {}", start, end);
                                        let substr = self.raw.get(start..end).unwrap();
                                        println!("Integer as string: {}", substr);
                                        match substr.parse::<i64>() {
                                            Ok(i) => {
                                                let tok = Token::Int(self.position, self.line, self.col, i);
                                                self.advance_to(end);
                                                Ok(tok)
                                            },
                                            Err(_) => Err(LexErrorKind::UnableToConvert { line: self.line, col: self.col })
                                        }
                                    },
                                    None => Err(LexErrorKind::UnknownCharacter { line: self.line, col: self.col }),
                                }
                            },
                            None => Err(LexErrorKind::UnexpectedCharacter { line: self.line, col: self.col }),
                        }
                    } else {
                        Err(LexErrorKind::UnableToConvert { line: self.line, col: self.col })
                    }
                },
                '.' => {
                    lazy_static! {
                        static ref SPREAD: Regex = Regex::new("...").unwrap();
                    }
                    if SPREAD.is_match_at(self.raw, *index) {
                        let cur_col = self.col;
                        let cur_pos = self.position;
                        self.advance_n(3);
                        println!("pos: {}..{}", cur_pos, self.position);
                        println!("pos: {}..{}", cur_col, self.col);
                        println!("next: {:?}", self.input.peek());
                        Ok(Token::Spread(cur_pos, self.line, cur_col))
                    } else {
                        Err(LexErrorKind::UnexpectedCharacter { line: self.line, col: self.col })
                    }
                }
                _ => Err(LexErrorKind::UnknownCharacter { line: self.line, col: self.col }),
            };
            Some(tok)
        } else {
            println!("Found a None in the string: {}", self.ended);
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
pub fn tokenize<'a>(input: &'a str) -> Result<Vec<Token<'a>>, LexErrorKind> {
    let state = Lexer::new(input);
    let results: Result<Vec<Token>, LexErrorKind> = state.collect();
    results
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    #[test]
    fn lex_empty() {
        let empty = tokenize("");
        assert!(empty.is_ok());
        assert_eq!(empty.unwrap(), vec![
            Token::Start,
            Token::End,
        ]);
    }

    #[test]
    fn lex_bang() {
        println!("Testing bang");
        let one = tokenize("!");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
            Token::Start,
            Token::Bang(0, 1, 1),
            Token::End,
        ]);
    }

    #[test]
    fn lex_dollar() {
        println!("Testing dollar");
        let one = tokenize("$");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
            Token::Start,
            Token::Dollar(0, 1, 1),
            Token::End,
        ]);
    }

    #[test]
    fn lex_ampersand() {
        println!("Testing ampersand");
        let one = tokenize("&");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
            Token::Start,
            Token::Amp(0, 1, 1),
            Token::End,
        ]);
    }

    #[test]
    fn lex_at_sign() {
        println!("Testing at sign");
        let one = tokenize("@");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
            Token::Start,
            Token::At(0, 1, 1),
            Token::End,
        ]);
    }

    #[test]
    fn lex_pipe() {
        println!("Testing pipe");
        let one = tokenize("|");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
            Token::Start,
            Token::Pipe(0, 1, 1),
            Token::End,
        ]);
    }

    #[test]
    fn lex_colon() {
        println!("Testing colon");
        let one = tokenize(":");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
            Token::Start,
            Token::Colon(0, 1, 1),
            Token::End,
        ]);
    }

    #[test]
    fn lex_open_brace() {
        println!("Testing open brace");
        let one = tokenize("{");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
            Token::Start,
            Token::OpenBrace(0, 1, 1),
            Token::End,
        ]);
    }
    #[test]
    fn lex_close_brace() {
        println!("Testing close brace");
        let one = tokenize("}");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
            Token::Start,
            Token::CloseBrace(0, 1, 1),
            Token::End,
        ]);
    }

    #[test]
    fn lex_open_paren() {
        println!("Testing open parenthesis");
        let one = tokenize("(");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
            Token::Start,
            Token::OpenParen(0, 1, 1),
            Token::End,
        ]);
    }
    #[test]
    fn lex_close_paren() {
        println!("Testing close parenthasis");
        let one = tokenize(")");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
            Token::Start,
            Token::CloseParen(0, 1, 1),
            Token::End,
        ]);
    }

    #[test]
    fn lex_open_square() {
        println!("Testing open square bracket");
        let one = tokenize("[");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
            Token::Start,
            Token::OpenSquare(0, 1, 1),
            Token::End,
        ]);
    }
    #[test]
    fn lex_close_square() {
        println!("Testing close square bracket");
        let one = tokenize("]");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
            Token::Start,
            Token::CloseSquare(0, 1, 1),
            Token::End,
        ]);
    }

    #[test]
    fn lex_spread() {
        println!("Testing spread");
        let one = tokenize("...");
        println!("Debug float: {:?}", one);
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
            Token::Start,
            Token::Spread(0, 1, 1),
            Token::End,
        ]);
    }

    #[test]
    fn lex_int() {
        println!("Testing int");
        let one = tokenize("123456");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
            Token::Start,
            Token::Int(0, 1, 1, 123456i64),
            Token::End,
        ]);
        let one = tokenize("-9876");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
            Token::Start,
            Token::Int(0, 1, 1, -9876i64),
            Token::End,
        ]);
    }

    #[test]
    fn lex_float() {
        println!("Testing float");
        let one = tokenize("1.23456789");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
            Token::Start,
            Token::Float(0, 1, 1, 1.23456789f64),
            Token::End,
        ]);
        let one = tokenize("-0.987654321");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
            Token::Start,
            Token::Float(0, 1, 1, -0.987654321f64),
            Token::End,
        ]);
    }

    #[test]
    fn lex_strings() {
        println!("Testing strings");
        let text = tokenize(r#""text""#);
        assert!(text.is_ok());
        assert_eq!(text.unwrap(), vec![
            Token::Start,
            Token::Str(0, 1, 1, "text"),
            Token::End,
        ]);
    }

    #[test]
    fn lex_block_strings() {
        println!("Testing block strings");
        let text = tokenize(r#""""test

text""""#);
        assert!(text.is_ok());
        assert_eq!(text.unwrap(), vec![
            Token::Start,
            Token::BlockStr(0, 1, 1, "test\n\ntext"),
            Token::End,
        ]);
    }

    #[test]
    fn lex_name() {
        println!("Testing name");
        let text = tokenize("name");
        assert!(text.is_ok());
        assert_eq!(text.unwrap(), vec![
            Token::Start,
            Token::Name(0, 1, 1, "name"),
            Token::End,
        ]);
    }

    #[test]
    fn lex_query() {
        println!("Test query");
        let query = tokenize(r#"query {
  hero {
    name
  }
  droid(id: "2000") {
    name
  }
}"#);
        assert!(query.is_ok());
        assert_eq!(query.unwrap(), vec![
            Token::Start,
            Token::Name(0, 1, 1, "query"),
            Token::Whitespace(5, 1, 6, WhitespaceType::Space),
            Token::OpenBrace(6, 1, 7),
            Token::Whitespace(7, 1, 8, WhitespaceType::Newline),
            Token::Whitespace(8, 2, 1, WhitespaceType::Space),
            Token::Whitespace(9, 2, 2, WhitespaceType::Space),
            Token::Name(10, 2, 3, "hero"),
            Token::Whitespace(14, 2, 7, WhitespaceType::Space),
            Token::OpenBrace(15, 2, 8),
            Token::Whitespace(16, 2, 9, WhitespaceType::Newline),
            Token::Whitespace(17, 3, 1, WhitespaceType::Space),
            Token::Whitespace(18, 3, 2, WhitespaceType::Space),
            Token::Whitespace(19, 3, 3, WhitespaceType::Space),
            Token::Whitespace(20, 3, 4, WhitespaceType::Space),
            Token::Name(21, 3, 5, "name"),
            Token::Whitespace(25, 3, 9, WhitespaceType::Newline),
            Token::Whitespace(26, 4, 1, WhitespaceType::Space),
            Token::Whitespace(27, 4, 2, WhitespaceType::Space),
            Token::CloseBrace(28, 4, 3),
            Token::Whitespace(29, 4, 4, WhitespaceType::Newline),
            Token::Whitespace(30, 5, 1, WhitespaceType::Space),
            Token::Whitespace(31, 5, 2, WhitespaceType::Space),
            Token::Name(32, 5, 3, "droid"),
            Token::OpenParen(37, 5, 8),
            Token::Name(38, 5, 9, "id"),
            Token::Colon(40, 5, 11),
            Token::Whitespace(41, 5, 12, WhitespaceType::Space),
            Token::Str(42, 5, 13, "2000"),
            Token::CloseParen(48, 5, 19),
            Token::Whitespace(49, 5, 20, WhitespaceType::Space),
            Token::OpenBrace(50, 5, 21),
            Token::Whitespace(51, 5, 22, WhitespaceType::Newline),
            Token::Whitespace(52, 6, 1, WhitespaceType::Space),
            Token::Whitespace(53, 6, 2, WhitespaceType::Space),
            Token::Whitespace(54, 6, 3, WhitespaceType::Space),
            Token::Whitespace(55, 6, 4, WhitespaceType::Space),
            Token::Name(56, 6, 5, "name"),
            Token::Whitespace(60, 6, 9, WhitespaceType::Newline),
            Token::Whitespace(61, 7, 1, WhitespaceType::Space),
            Token::Whitespace(62, 7, 2, WhitespaceType::Space),
            Token::CloseBrace(63, 7, 3),
            Token::Whitespace(64, 7, 4, WhitespaceType::Newline),
            Token::CloseBrace(65, 8, 1),
            Token::End,
        ])
    }

    #[test]
    fn lex_type() {
        let t = tokenize(r#"type Query {
  hero(episode: Episode): Character
  droid(id: ID!): Droid
}
"#);
        assert!(t.is_ok());
        assert_eq!(t.unwrap(), vec![
            Token::Start,
            Token::Name(0, 1, 1, "type"),
            Token::Whitespace(4, 1, 5, WhitespaceType::Space),
            Token::Name(5, 1, 6, "Query"),
            Token::Whitespace(10, 1, 11, WhitespaceType::Space),
            Token::OpenBrace(11, 1, 12),
            Token::Whitespace(12, 1, 13, WhitespaceType::Newline),
            Token::Whitespace(13, 2, 1, WhitespaceType::Space),
            Token::Whitespace(14, 2, 2, WhitespaceType::Space),
            Token::Name(15, 2, 3, "hero"),
            Token::OpenParen(19, 2, 7),
            Token::Name(20, 2, 8, "episode"),
            Token::Colon(27, 2, 15),
            Token::Whitespace(28, 2, 16, WhitespaceType::Space),
            Token::Name(29, 2, 17, "Episode"),
            Token::CloseParen(36, 2, 24),
            Token::Colon(37, 2, 25),
            Token::Whitespace(38, 2, 26, WhitespaceType::Space),
            Token::Name(39, 2, 27, "Character"),
            Token::Whitespace(48, 2, 36, WhitespaceType::Newline),
            Token::Whitespace(49, 3, 1, WhitespaceType::Space),
            Token::Whitespace(50, 3, 2, WhitespaceType::Space),
            Token::Name(51, 3, 3, "droid"),
            Token::OpenParen(56, 3, 8),
            Token::Name(57, 3, 9, "id"),
            Token::Colon(59, 3, 11),
            Token::Whitespace(60, 3, 12, WhitespaceType::Space),
            Token::Name(61, 3, 13, "ID"),
            Token::Bang(63, 3, 15),
            Token::CloseParen(64, 3, 16),
            Token::Colon(65, 3, 17),
            Token::Whitespace(66, 3, 18, WhitespaceType::Space),
            Token::Name(67, 3, 19, "Droid"),
            Token::Whitespace(72, 3, 24, WhitespaceType::Newline),
            Token::CloseBrace(73, 4, 1),
            Token::Whitespace(74, 4, 2, WhitespaceType::Newline),
            Token::End,
        ])
    }

    #[test]
    fn lex_fragment() {
        let fragment = tokenize(r#"query {
  hero {
    name
    ... on Human {
      height
    }
  }
}"#);
        assert!(fragment.is_ok());
        assert_eq!(fragment.unwrap(), vec![
            Token::Start,
            Token::Name(0, 1, 1, "query"),
            Token::Whitespace(5, 1, 6, WhitespaceType::Space),
            Token::OpenBrace(6, 1, 7),
            Token::Whitespace(7, 1, 8, WhitespaceType::Newline),
            Token::Whitespace(8, 2, 1, WhitespaceType::Space),
            Token::Whitespace(9, 2, 2, WhitespaceType::Space),
            Token::Name(10, 2, 3, "hero"),
            Token::Whitespace(14, 2, 7, WhitespaceType::Space),
            Token::OpenBrace(15, 2, 8),
            Token::Whitespace(16, 2, 9, WhitespaceType::Newline),
            Token::Whitespace(17, 3, 1, WhitespaceType::Space),
            Token::Whitespace(18, 3, 2, WhitespaceType::Space),
            Token::Whitespace(19, 3, 3, WhitespaceType::Space),
            Token::Whitespace(20, 3, 4, WhitespaceType::Space),
            Token::Name(21, 3, 5, "name"),
            Token::Whitespace(25, 3, 9, WhitespaceType::Newline),
            Token::Whitespace(26, 4, 1, WhitespaceType::Space),
            Token::Whitespace(27, 4, 2, WhitespaceType::Space),
            Token::Whitespace(28, 4, 3, WhitespaceType::Space),
            Token::Whitespace(29, 4, 4, WhitespaceType::Space),
            Token::Spread(30, 4, 5),
            Token::Whitespace(33, 4, 8, WhitespaceType::Space),
            Token::Name(34, 4, 9, "on"),
            Token::Whitespace(36, 4, 11, WhitespaceType::Space),
            Token::Name(37, 4, 12, "Human"),
            Token::Whitespace(42, 4, 17, WhitespaceType::Space),
            Token::OpenBrace(43, 4, 18),
            Token::Whitespace(44, 4, 19, WhitespaceType::Newline),
            Token::Whitespace(45, 5, 1, WhitespaceType::Space),
            Token::Whitespace(46, 5, 2, WhitespaceType::Space),
            Token::Whitespace(47, 5, 3, WhitespaceType::Space),
            Token::Whitespace(48, 5, 4, WhitespaceType::Space),
            Token::Whitespace(49, 5, 5, WhitespaceType::Space),
            Token::Whitespace(50, 5, 6, WhitespaceType::Space),
            Token::Name(51, 5, 7, "height"),
            Token::Whitespace(57, 5, 13, WhitespaceType::Newline),
            Token::Whitespace(58, 6, 1, WhitespaceType::Space),
            Token::Whitespace(59, 6, 2, WhitespaceType::Space),
            Token::Whitespace(60, 6, 3, WhitespaceType::Space),
            Token::Whitespace(61, 6, 4, WhitespaceType::Space),
            Token::CloseBrace(62, 6, 5),
            Token::Whitespace(63, 6, 6, WhitespaceType::Newline),
            Token::Whitespace(64, 7, 1, WhitespaceType::Space),
            Token::Whitespace(65, 7, 2, WhitespaceType::Space),
            Token::CloseBrace(66, 7, 3),
            Token::Whitespace(67, 7, 4, WhitespaceType::Newline),
            Token::CloseBrace(68, 8, 1),
            Token::End,
        ]);
    }

    #[test]
    fn handles_unmatched_quote() {
        let err = tokenize("\"test");
        assert!(err.is_err());
        assert_eq!(err.unwrap_err(), LexErrorKind::UnmatchQuote { line: 1, col: 2 });
        let err = tokenize("\"\"\"test\n\n\"\"");
        assert!(err.is_err());
        assert_eq!(err.unwrap_err(), LexErrorKind::UnmatchQuote { line: 1, col: 2 });
        let err = tokenize("\"\"\"test\ntest\ntest\"");
        assert!(err.is_err());
        assert_eq!(err.unwrap_err(), LexErrorKind::UnmatchQuote { line: 1, col: 2 });
    }

    #[test]
    fn handles_unknown_character() {
        let err = tokenize("%");
        assert!(err.is_err());
        assert_eq!(err.unwrap_err(), LexErrorKind::UnknownCharacter { line: 1, col: 1 });
    }
}
