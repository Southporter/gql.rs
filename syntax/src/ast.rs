use crate::lexer::{Lexer, LexErrorKind};
use crate::token::Token;
use crate::nodes::{Document, DefinitionNode, TypeSystemDefinitionNode, TypeDefinitionNode, ObjectTypeDefinitionNode};
use std::iter::{Iterator, Peekable};

pub struct AST<'i>
{
    input: &'i str,
    lexer: Peekable<Lexer<'i>>,
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
        let lexer = Lexer::new(input).peekable();
        Ok(AST {
            input,
            lexer,
        })
    }

    pub fn parse(&'i mut self) -> Result<Document<'i>, ParseError> {
        let definitions = self.parse_definitions()?;
        Ok(Document::new(definitions))
    }

    fn parse_definitions(&'i mut self) -> Result<Vec<DefinitionNode<'i>>, ParseError> {
        self.expect_token(Token::Start)?;
        let mut nodes: Vec<DefinitionNode> = Vec::new();
        loop {
            nodes.push(self.parse_definition()?);
            if let Some(_) = self.expect_optional_token(&Token::End) {
                break;

            }
        }
        Ok(nodes)

        // self.many(Token::Start, |ast| ast.parse_definition(), Token::End)
    }

    fn parse_definition(&mut self) -> Result<DefinitionNode<'i>, ParseError> {
        let tok = self.unwrap_peeked_token()?;
        if let Token::Name(_, _, _, val) = tok {
            match *val {
                "type" => Ok(DefinitionNode::TypeSystem(
                    TypeSystemDefinitionNode::Type(
                        self.parse_type()?
                    )
                )),
                _ => Err(ParseError::BadValue),
            }
        } else {
            Err(ParseError::UnexpectedToken {
                expected: String::from("Token<Name>"),
                received: tok.to_string().to_owned(),

            })
        }
    }

    fn parse_type(&mut self) -> Result<TypeDefinitionNode<'i>,  ParseError> {
        let tok = self.unwrap_next_token()?;
        if let Token::Name(_, _, _, val) = tok {
            match val {
                "type" => Ok(
                    TypeDefinitionNode::Object(
                        self.parse_object_type()?
                    )
                ),
                _ => Err(ParseError::BadValue),
            }
        } else {
            Err(ParseError::UnexpectedToken {
                expected: String::from("Token::Name"),
                received: tok.to_string().to_owned(),
            })
        }
    }

    fn parse_object_type(&mut self) -> Result<ObjectTypeDefinitionNode<'i>, ParseError> {

        let tok = self.unwrap_next_token()?;
        Ok(ObjectTypeDefinitionNode::new(tok)?)
    }


    // fn many<T, P>(&'i mut self, start: Token<'i>, parser: P, end: Token<'i>) -> Result<Vec<T>, ParseError>
    // where P: Fn(&'i mut AST<'i>) -> Result<T, ParseError>
    // {
    //     self.expect_token(start)?;
    //     let mut nodes: Vec<T> = Vec::new();
    //     loop {
    //         let node = parser(self)?;
    //         if let Some(_) = self.expect_optional_token(&end) {
    //             nodes.push(node);
    //             break;
    //         }
    //     }
    //     Ok(nodes)
    // }

    fn expect_token(&mut self, tok: Token) -> Result<(), ParseError> {
        if let Some(next) = self.lexer.next() {
            match next {
                Ok(actual) => {
                    if actual != tok {
                        Err(ParseError::UnexpectedToken {
                            expected: tok.to_string(),
                            received: actual.to_string().to_owned(),
                        })
                    } else {
                        Ok(())
                    }
                },
                Err(e) => Err(ParseError::LexError(e)),
            }
        } else {
            Err(ParseError::EOF)
        }
    }
    fn expect_optional_token<'a>(&'a mut self, tok: &Token<'a>) -> Option<Token<'a>> {
        if let Some(next) = self.lexer.peek() {
            match next {
                Ok(actual) => {
                    if *actual == *tok {
                        Some(self.lexer.next().unwrap().unwrap())
                    } else {
                        None
                    }
                },
                Err(_) => None
            }
        } else {
            None
        }
    }

    fn unwrap_peeked_token(&mut self) -> Result<&Token<'i>, ParseError> {
        match self.lexer.peek() {
            Some(res) => {
                match res {
                    Ok(tok) => {
                        Ok(tok)
                    },
                    Err(lex_error) => Err(ParseError::LexError(*lex_error)),
                }
            },
            None => Err(ParseError::EOF),
        }
    }

    fn unwrap_next_token(&mut self) -> Result<Token<'i>, ParseError> {
        match self.lexer.next() {
            Some(res) => {
                match res {
                    Ok(tok) => {
                        Ok(tok)
                    },
                    Err(lex_error) => Err(ParseError::LexError(lex_error)),
                }
            },
            None => Err(ParseError::EOF),
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
