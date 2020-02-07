use crate::token::{Token, WhitespaceType};
use crate::extract::ExtractErrorKind;
use std::str::CharIndices;
use std::iter::Peekable;
use std::iter::Iterator;

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
                '{' => {
                    self.input.next();
                    Ok(Token::OpenBrace(self.position, self.line, self.col))
                },
                '}' => {
                    self.input.next();
                    Ok(Token::CloseBrace(self.position, self.line, self.col))
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
                }
                'a' ..= 'z' | 'A' ..= 'Z' => {
                    let init_pos = *index;
                    match self.input.position(|(_,c)| !c.is_alphanumeric()) {
                        Some(pos) => {
                            self.position += pos - init_pos;
                            // self.col += pos - init_pos;
                            Ok(Token::Name(self.position, self.line, self.col, self.raw.get(init_pos..pos).unwrap()))
                        },
                        None => Ok(Token::Name(self.position, self.line, self.col, self.raw.get(init_pos..).unwrap())),
                    }
                }
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
    fn lex_strings() {
        println!("Testing strings");
        let text = tokenize("\"text\"");
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
    fn lex_name() {
        println!("Testing name");
        let text = tokenize("name");
        assert!(text.is_ok());
        assert_eq!(text.unwrap(), vec![
                   Token::Name(
                       0,
                       0,
                       1,
                       "name"
                   )
        ]);

    }
}
