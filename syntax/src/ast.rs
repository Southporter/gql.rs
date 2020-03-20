use crate::lexer::{Lexer, LexErrorKind};
use crate::token::Token;
use crate::nodes::Document;
use std::iter::Iterator;

pub struct AST<'i>
{
    input: &'i str,
    document: Document<'i>,
}

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
impl<'i> Display for AST<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "AST")
    }
}
impl<'i> Debug for AST<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "AST")
    }
}

impl<'i> AST<'i> {
    pub fn new(input: &'i str) -> Result<AST<'i>, ParseError> {
        let lexer = Lexer::new(input).filter(|x| match x {
            Ok(z) => match z {
                Token::Whitespace(_, _, _, _) => false,
                _ => true,
            },
            Err(_) => true,
        });

        let document = Document::new(Box::new(lexer))?;
        Ok(AST {
            input,
            document,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    BadValue,
    DocumentEmpty,
    LexError(LexErrorKind),
    UnexpectedToken { expected: &'static str, received: String }
}

struct Location<'a> {
    start: Token<'a>,
    end: Token<'a>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_runs() {
        let _ast = AST::new("test");
        assert!(true);
    }
}
