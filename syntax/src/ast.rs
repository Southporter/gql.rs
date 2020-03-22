use crate::lexer::{Lexer, LexErrorKind};
use crate::nodes::Document;
use std::iter::Iterator;

#[derive(PartialEq)]
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
        let mut lexer = Lexer::new(input).peekable();

        let document = Document::new(&mut lexer)?;
        Ok(AST {
            input,
            document,
        })
    }
    pub fn from(input: &'i str, document: Document<'i>) -> AST<'i> {
        AST {
            input,
            document,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    BadValue,
    DocumentEmpty,
    EOF,
    LexError(LexErrorKind),
    UnexpectedToken { expected: String, received: String }
}

// struct Location<'a> {
//     start: Token<'a>,
//     end: Token<'a>,
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_runs() {
        let _ast = AST::new("test");
        assert!(true);
    }
}
