#[macro_use] extern crate lazy_static;
pub mod token;
pub mod lexer;
pub mod ast;
mod nodes;

use ast::{AST, ParseError};
// use lexer::LexErrorKind;
// use token::Token;

pub fn parse<'a>(query: &'a str) -> Result<AST, ParseError>
{
    let ast = AST::new(query);
    ast
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::{Document, DefinitionNode, TypeSystemDefinitionNode, TypeDefinitionNode, ObjectTypeDefinitionNode, NameNode};

    #[test]
    fn it_handles_lexing_error() {
        println!("parsing error");
        let res = parse("");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), ParseError::DocumentEmpty);
    }

    #[test]
    #[ignore]
    fn parses_object() {
        println!("parsing an object");
        let input = r#"type Obj{
  Var1: String
  Var2: Number
}"#;
        let res = parse(input);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), AST::from(
            input,
            Document {
                definitions: vec![
                    DefinitionNode::TypeSystem(
                        TypeSystemDefinitionNode::Type(
                            TypeDefinitionNode::Object(
                                ObjectTypeDefinitionNode {
                                    description: None,
                                    name: NameNode {
                                        value: input.get(5..7).unwrap()
                                    }
                                }
                            )
                        )
                    )
                ]
            }
        ))
    }
}
