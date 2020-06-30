use crate::lexer::{Lexer, LexErrorKind};
use crate::token::Token;
use crate::nodes::{Document, DefinitionNode, TypeSystemDefinitionNode, TypeDefinitionNode, ObjectTypeDefinitionNode, FieldDefinitionNode, TypeNode, NamedTypeNode, ListTypeNode};
use std::iter::{Iterator, Peekable};
use std::rc::Rc;

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

    pub fn parse(&'i mut self) -> Result<Document, ParseError> {
        let definitions = self.parse_definitions()?;
        Ok(Document::new(definitions))
    }

    fn parse_definitions(&'i mut self) -> Result<Vec<DefinitionNode>, ParseError> {
        self.expect_token(Token::Start)?;
        if let Some(_) = self.expect_optional_token(&Token::End) {
            Err(ParseError::DocumentEmpty)
        } else {
            let mut nodes: Vec<DefinitionNode> = Vec::new();
            loop {
                nodes.push(self.parse_definition()?);
                if let Some(_) = self.expect_optional_token(&Token::End) {
                    break;

                }
            }
            Ok(nodes)
        }
    }

    fn parse_definition(&mut self) -> Result<DefinitionNode, ParseError> {
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

    fn parse_type(&mut self) -> Result<TypeDefinitionNode,  ParseError> {
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

    fn parse_object_type(&mut self) -> Result<ObjectTypeDefinitionNode, ParseError> {

        let name_tok = self.unwrap_next_token()?;
        if let Token::Name(_, _, _, _name) = name_tok {
            let fields = self.parse_fields()?;

            let obj = Ok(ObjectTypeDefinitionNode::new(name_tok, fields)?);
            obj
        } else {
            Err(self.parse_error(String::from("Token::Name"), name_tok))
        }
    }

    fn parse_fields(&mut self) -> Result<Vec<FieldDefinitionNode>, ParseError> {
        let mut fields: Vec<FieldDefinitionNode> = Vec::new();
        self.expect_token(Token::OpenBrace(0, 0, 0))?;
        loop {
            if let (Some(_)) = self.expect_optional_token(&Token::CloseBrace(0, 0, 0)) {
                break;
            }
            fields.push(self.parse_field()?);
        }
        Ok(fields)
    }

    fn parse_field(&mut self) -> Result<FieldDefinitionNode, ParseError> {
        let name = self.unwrap_next_token()?;
        self.expect_token(Token::Colon(0,0,0))?;
        let field_type = self.parse_field_type()?;
        FieldDefinitionNode::new(name, field_type)
    }

    fn parse_field_type(&mut self) -> Result<TypeNode, ParseError> {
        let mut field_type: TypeNode;
        if let Some(_) = self.expect_optional_token(&Token::OpenSquare(0, 0, 0)) {
            field_type = TypeNode::List(
                ListTypeNode::new(self.parse_field_type()?)
            );
            self.expect_token(Token::CloseSquare(0,0,0))?;
        } else {
            field_type = TypeNode::Named(
                NamedTypeNode::new(
                    self.expect_token(Token::Name(0,0,0,""))?
                )?
            );
        }
        if let Some(_) = self.expect_optional_token(&Token::Bang(0,0,0)) {
            field_type = TypeNode::NonNull(
                Rc::new(field_type)
            );
        }
        Ok(field_type)
    }

    fn parse_error(self: &mut Self, expected: String, received: Token) -> ParseError {
        ParseError::UnexpectedToken {
            expected,
            received: received.to_string().to_owned(),
        }
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

    fn expect_token<'a>(&'a mut self, tok: Token) -> Result<Token<'a>, ParseError> {
        if let Some(next) = self.lexer.next() {
            match next {
                Ok(actual) => {
                    if actual != tok {
                        Err(ParseError::UnexpectedToken {
                            expected: tok.to_string(),
                            received: actual.to_string().to_owned(),
                        })
                    } else {
                        Ok(actual)
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
    fn it_constructs() {
        let _ast = AST::new("test");
        assert!(true);
    }
}
