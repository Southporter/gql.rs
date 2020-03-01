use crate::token::{Token, WhitespaceType};
use crate::extract::ExtractErrorKind;
use std::str::CharIndices;
use std::iter::Peekable;
use std::iter::Iterator;
use regex::Regex;

#[derive(Debug)]
pub struct Lexer<'a> {
    raw: &'a str,
    input: Peekable<CharIndices<'a>>,
    pub position: usize,
    pub line: usize,
    pub col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            raw: input,
            input: input.char_indices().peekable(),
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
}

use std::fmt;
impl<'a> fmt::Display for Lexer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lexer<position: {}, line: {}, col: {}>", self.position, self.line, self.col)
    }
}


impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, ExtractErrorKind>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((index, next)) = self.input.peek() {
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
                                    None => Err(ExtractErrorKind::UnmatchQuote { line: self.line, col: self.col + 1 }),
                                }
                            },
                            None => Err(ExtractErrorKind::UnmatchQuote { line: self.line, col: self.col + 1 }),
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
                                    None => Err(ExtractErrorKind::UnmatchQuote { line: self.line, col: self.col + 1 }),
                                }
                            },
                            None => Err(ExtractErrorKind::UnmatchQuote { line: self.line, col: self.col + 1 }),
                        }
                    }
                }
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
                '1' ..= '9' => {
                    // Handle integers and floats here
                    Err(ExtractErrorKind::UnhandledCase)
                },
                '.' => {
                    Err(ExtractErrorKind::UnhandledCase)
                }
                _ => Err(ExtractErrorKind::UnknownCharacter { line: self.line, col: self.col }),
            };
            Some(tok)
        } else {
            None
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
pub fn tokenize<'a>(input: &'a str) -> Result<Vec<Token<'a>>, ExtractErrorKind> {
    let state = Lexer::new(input);
    let results: Result<Vec<Token>, ExtractErrorKind> = state.collect();
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
        assert!(empty.unwrap().is_empty());
    }

    #[test]
    fn lex_bang() {
        println!("Testing bang");
        let one = tokenize("!");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
            Token::Bang(0, 1, 1)
        ]);
    }

    #[test]
    fn lex_dollar() {
        println!("Testing dollar");
        let one = tokenize("$");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
            Token::Dollar(0, 1, 1)
        ]);
    }

    #[test]
    fn lex_ampersand() {
        println!("Testing ampersand");
        let one = tokenize("&");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
                   Token::Amp(
                       0,
                       1,
                       1,
                   )
        ]);
    }

    #[test]
    fn lex_at_sign() {
        println!("Testing at sign");
        let one = tokenize("@");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
                   Token::At(
                       0,
                       1,
                       1,
                   )
        ]);
    }

    #[test]
    fn lex_pipe() {
        println!("Testing pipe");
        let one = tokenize("|");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
                   Token::Pipe(
                       0,
                       1,
                       1,
                   )
        ]);
    }

    #[test]
    fn lex_colon() {
        println!("Testing colon");
        let one = tokenize(":");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
                   Token::Colon(
                       0,
                       1,
                       1,
                   )
        ]);
    }

    #[test]
    fn lex_open_brace() {
        println!("Testing open brace");
        let one = tokenize("{");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
                   Token::OpenBrace(
                       0,
                       1,
                       1,
                   )
        ]);
    }
    #[test]
    fn lex_close_brace() {
        println!("Testing close brace");
        let one = tokenize("}");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
                   Token::CloseBrace(
                       0,
                       1,
                       1,
                   )
        ]);
    }

    #[test]
    fn lex_open_paren() {
        println!("Testing open parenthesis");
        let one = tokenize("(");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
                   Token::OpenParen(
                       0,
                       1,
                       1,
                   )
        ]);
    }
    #[test]
    fn lex_close_paren() {
        println!("Testing close parenthasis");
        let one = tokenize(")");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
                   Token::CloseParen(
                       0,
                       1,
                       1,
                   )
        ]);
    }

    #[test]
    fn lex_open_square() {
        println!("Testing open square bracket");
        let one = tokenize("[");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
                   Token::OpenSquare(
                       0,
                       1,
                       1,
                   )
        ]);
    }
    #[test]
    fn lex_close_square() {
        println!("Testing close square bracket");
        let one = tokenize("]");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
                   Token::CloseSquare(
                       0,
                       1,
                       1,
                   )
        ]);
    }

    #[test]
    fn lex_spread() {
        println!("Testing spread");
        let one = tokenize("...");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
            Token::Spread(0, 1, 1)
        ]);
    }

    #[test]
    fn lex_int() {
        println!("Testing spread");
        let one = tokenize("123456");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
            Token::Int(0, 1, 1, 123456i64)
        ]);
    }

    #[test]
    fn lex_float() {
        println!("Testing float");
        let one = tokenize("1.23456789");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
            Token::Float(0, 1, 1, 1.234556789f64)
        ]);
    }

    #[test]
    fn lex_strings() {
        println!("Testing strings");
        let text = tokenize(r#""text""#);
        assert!(text.is_ok());
        assert_eq!(text.unwrap(), vec![
                   Token::Str(
                       0,
                       1,
                       1,
                       "text"
                   )
        ]);
    }

    #[test]
    fn lex_block_strings() {
        println!("Testing block strings");
        let text = tokenize(r#""""test

text""""#);
        assert!(text.is_ok());
        assert_eq!(text.unwrap(), vec![
                   Token::BlockStr(
                       0,
                       1,
                       1,
                       "test\n\ntext"
                   )
        ]);
    }

    #[test]
    fn lex_name() {
        println!("Testing name");
        let text = tokenize("name");
        assert!(text.is_ok());
        assert_eq!(text.unwrap(), vec![
                   Token::Name(
                       0,
                       1,
                       1,
                       "name"
                   )
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
            Token::Name(0, 1, 1, "query"),
            Token::Whitespace(5, 1, 6, WhitespaceType::Space),
            Token::OpenBrace(6, 1, 7),
            Token::Whitespace(7, 1, 8, WhitespaceType::Newline),
            Token::Whitespace(8, 2, 1, WhitespaceType::Space),
            Token::Whitespace(9, 2, 2, WhitespaceType::Space),
            Token::Name(10, 2, 3, "hero"),
            Token::Whitespace(11, 2, 7, WhitespaceType::Space),
            Token::OpenBrace(12, 2, 8),
            Token::Whitespace(13, 2, 9, WhitespaceType::Newline),
            Token::Whitespace(14, 3, 1, WhitespaceType::Space),
            Token::Whitespace(15, 3, 2, WhitespaceType::Space),
            Token::Whitespace(16, 3, 3, WhitespaceType::Space),
            Token::Whitespace(17, 3, 4, WhitespaceType::Space),
            Token::Name(18, 3, 5, "name"),
            Token::Whitespace(22, 3, 9, WhitespaceType::Newline),
            Token::Whitespace(23, 4, 1, WhitespaceType::Space),
            Token::Whitespace(24, 4, 2, WhitespaceType::Space),
            Token::Whitespace(25, 4, 3, WhitespaceType::Space),
            Token::Whitespace(26, 4, 4, WhitespaceType::Space),
            Token::Spread(27, 4, 5),
            Token::Whitespace(30, 4, 8, WhitespaceType::Space),
            Token::Name(31, 4, 9, "on"),
            Token::Whitespace(33, 4, 11, WhitespaceType::Space),
            Token::Name(34, 4, 12, "Human"),
            Token::Whitespace(39, 4, 17, WhitespaceType::Space),
            Token::OpenBrace(40, 4, 18),
            Token::Whitespace(41, 4, 19, WhitespaceType::Newline),
            Token::Whitespace(42, 5, 1, WhitespaceType::Space),
            Token::Whitespace(43, 5, 2, WhitespaceType::Space),
            Token::Whitespace(44, 5, 3, WhitespaceType::Space),
            Token::Whitespace(45, 5, 4, WhitespaceType::Space),
            Token::Whitespace(46, 5, 5, WhitespaceType::Space),
            Token::Whitespace(47, 5, 6, WhitespaceType::Space),
            Token::Name(48, 5, 7, "height"),
            Token::Whitespace(54, 5, 13, WhitespaceType::Newline),
            Token::Whitespace(55, 6, 1, WhitespaceType::Space),
            Token::Whitespace(56, 6, 2, WhitespaceType::Space),
            Token::Whitespace(57, 6, 3, WhitespaceType::Space),
            Token::Whitespace(58, 6, 4, WhitespaceType::Space),
            Token::CloseBrace(59, 6, 5),
            Token::Whitespace(60, 6, 6, WhitespaceType::Newline),
            Token::Whitespace(61, 7, 1, WhitespaceType::Space),
            Token::Whitespace(62, 7, 2, WhitespaceType::Space),
            Token::CloseBrace(63, 7, 3),
        ]);
    }

    #[test]
    fn handles_unmatched_quote() {
        let err = tokenize("\"test");
        assert!(err.is_err());
        assert_eq!(err.unwrap_err(), ExtractErrorKind::UnmatchQuote { line: 1, col: 2 });
        let err = tokenize("\"\"\"test\n\n\"\"");
        assert!(err.is_err());
        assert_eq!(err.unwrap_err(), ExtractErrorKind::UnmatchQuote { line: 1, col: 2 });
        let err = tokenize("\"\"\"test\ntest\ntest\"");
        assert!(err.is_err());
        assert_eq!(err.unwrap_err(), ExtractErrorKind::UnmatchQuote { line: 1, col: 2 });
    }

    #[test]
    fn handles_unknown_character() {
        let err = tokenize("%");
        assert!(err.is_err());
        assert_eq!(err.unwrap_err(), ExtractErrorKind::UnknownCharacter { line: 1, col: 1 });
    }
}
