#[macro_use] extern crate lazy_static;
pub mod token;
pub mod lexer;
pub mod ast;
mod nodes;

use ast::{AST, ParseError};
use nodes::{Document};
// use lexer::LexErrorKind;
// use token::Token;

pub fn parse<'a>(query: &'a str) -> Result<Document, ParseError>
{
    let mut ast = AST::new(query)?;
    let document = ast.parse()?;
    Ok(document)
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
    fn parses_object() {
        println!("parsing an object");
        let input = r#"type Obj{
}"#;
        let res = parse(input);
        println!("res: {:?}", res);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(),
            Document {
                definitions: vec![
                    DefinitionNode::TypeSystem(
                        TypeSystemDefinitionNode::Type(
                            TypeDefinitionNode::Object(
                                ObjectTypeDefinitionNode {
                                    description: None,
                                    name: NameNode {
                                        value: input.get(5..8).unwrap().to_owned()
                                    },
                                    // fields: vec![],
                                }
                            )
                        )
                    )
                ]
            }
        )
    }
}
