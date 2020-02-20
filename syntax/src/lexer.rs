use crate::token::{Token, WhitespaceType};
use crate::extract::ExtractErrorKind;
use std::str::CharIndices;
use std::iter::Peekable;
use std::iter::Iterator;
use regex::Regex;

pub struct Lexer<'a> {
    raw: &'a str,
    input: Peekable<CharIndices<'a>>,
    pub position: usize,
    pub line: u32,
    pub col: u32,
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
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, ExtractErrorKind>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((index, next)) = self.input.peek() {
            let tok = match next {
                '!' => {
                    self.input.next();
                    Ok(Token::Bang(self.position, self.line, self.col))
                },
                '$' => {
                    self.input.next();
                    Ok(Token::Dollar(self.position, self.line, self.col))
                },
                '&' => {
                    self.input.next();
                    Ok(Token::Amp(self.position, self.line, self.col))
                },
                '|' => {
                    self.input.next();
                    Ok(Token::Pipe(self.position, self.line, self.col))
                },
                '@' => {
                    self.input.next();
                    Ok(Token::At(self.position, self.line, self.col))
                },
                ':' => {
                    self.input.next();
                    Ok(Token::Colon(self.position, self.line, self.col))
                },
                '{' => {
                    self.input.next();
                    Ok(Token::OpenBrace(self.position, self.line, self.col))
                },
                '}' => {
                    self.input.next();
                    Ok(Token::CloseBrace(self.position, self.line, self.col))
                },
                '(' => {
                    self.input.next();
                    Ok(Token::OpenParen(self.position, self.line, self.col))
                },
                ')' => {
                    self.input.next();
                    Ok(Token::CloseParen(self.position, self.line, self.col))
                },
                '[' => {
                    self.input.next();
                    Ok(Token::OpenSquare(self.position, self.line, self.col))
                },
                ']' => {
                    self.input.next();
                    Ok(Token::CloseSquare(self.position, self.line, self.col))
                },
                ' ' => {
                    self.input.next();
                    Ok(Token::Whitespace(self.position, self.line, self.col, WhitespaceType::Space))
                },
                '\t' => {
                    self.input.next();
                    Ok(Token::Whitespace(self.position, self.line, self.col, WhitespaceType::Tab))
                },
                '\n' => {
                    let cur_pos  = self.position;
                    let cur_line = self.line;
                    let cur_col  = self.col;
                    self.line += 1;
                    self.col = 0;
                    self.input.next();
                    Ok(Token::Whitespace(cur_pos, cur_line, cur_col, WhitespaceType::Newline))
                },
                '"' => {
                    lazy_static! {
                        static ref BLOCK: Regex = Regex::new("\"\"\"").unwrap();
                        static ref SINGLE: Regex = Regex::new(r#""(?P<content>(?:\\.|[^"\\])*)""#).unwrap();
                    }
                    if BLOCK.is_match_at(self.raw, *index) {
                        println!("We found a block string!");
                        // let skipped = self.input.skip(3);
                        Err(ExtractErrorKind::Custom("BlockStr not implemented"))
                    } else {
                        println!("We found a regular string!");
                        let init_pos = *index;
                        let mut locations = SINGLE.capture_locations();
                        match SINGLE.captures_read_at(&mut locations, self.raw, init_pos) {
                            Some(m) => {
                                match locations.get(1) {
                                    Some((start_off, end_off)) => {
                                        let new_pos = self.input.position(|(i, _)| i == init_pos + end_off + 1);
                                        println!("start: {}, end: {}, new: {:?}", start_off, end_off, new_pos);
                                        Ok(Token::Str(init_pos, self.line, self.col, m.as_str().get(start_off..end_off).unwrap()))
                                    },
                                    None => Err(ExtractErrorKind::UnmatchQuote { line: self.line, col: self.col + 1 }),
                                }
                            },
                            None => Err(ExtractErrorKind::UnmatchQuote { line: self.line, col: self.col + 1 }),
                        }
                    }
                }
                '1' ..= '9' => {
                    // Handle integers and floats here
                    Err(ExtractErrorKind::UnhandledCase)
                },
                'a' ..= 'z' | 'A' ..= 'Z' => {
                    let init_pos = *index;
                    match self.input.position(|(_,c)| !c.is_alphanumeric()) {
                        Some(pos) => {
                            println!("pos: {}, init_pos: {}", pos, init_pos);
                            self.position += pos;
                            // self.col += pos - init_pos;
                            Ok(Token::Name(self.position, self.line, self.col, self.raw.get(init_pos..pos).unwrap()))
                        },
                        None => Ok(Token::Name(self.position, self.line, self.col, self.raw.get(init_pos..).unwrap())),
                    }
                },
                _ => Err(ExtractErrorKind::UnknownCharacter { line: self.line, col: self.col }),
            };
            self.position += 1;
            self.col += 1;
            Some(tok)
        } else {
            None
        }
    }
}

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
                   Token::Bang(
                       0,
                       1,
                       1,
                   )
        ]);
    }

    #[test]
    fn lex_dollar() {
        println!("Testing dollar");
        let one = tokenize("$");
        assert!(one.is_ok());
        assert_eq!(one.unwrap(), vec![
                   Token::Dollar(
                       0,
                       1,
                       1,
                   )
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
    fn lex_strings() {
        println!("Testing strings");
        let text = tokenize(r#""text""#);
        println!("text: {:?}", text);
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
    #[ignore]
    fn lex_block_strings() {
        println!("Testing block strings");
        let text = tokenize("\"\"\"test\n\ntext\"\"\"");
        assert!(text.is_ok());
        assert_eq!(text.unwrap(), vec![
                   Token::BlockStr(
                       0,
                       1,
                       1,
                       "text\n\ntext"
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
    fn handles_unmatched_quote() {
        let err = tokenize("\"test");
        assert!(err.is_err());
        assert_eq!(err.unwrap_err(), ExtractErrorKind::UnmatchQuote { line: 1, col: 2 });
        // let err = tokenize("\"\"\"test\n\n\"\"");
        // assert!(err.is_err());
        // assert_eq!(err.unwrap_err(), ExtractErrorKind::UnmatchQuote { line: 1, col: 2 });
        // let err = tokenize("\"\"\"test\ntest\ntest\"");
        // assert!(err.is_err());
        // assert_eq!(err.unwrap_err(), ExtractErrorKind::UnmatchQuote { line: 1, col: 2 });
    }

    #[test]
    fn handles_unknown_character() {
        let err = tokenize("%");
        assert!(err.is_err());
        assert_eq!(err.unwrap_err(), ExtractErrorKind::UnknownCharacter { line: 1, col: 1 });
    }
}
