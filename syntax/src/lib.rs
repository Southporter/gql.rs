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
    use crate::nodes::{Document, DefinitionNode, TypeSystemDefinitionNode, TypeDefinitionNode, ObjectTypeDefinitionNode, NameNode, FieldDefinitionNode, TypeNode, NamedTypeNode, ListTypeNode};
    use std::rc::Rc;

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
        let input = r#"type Obj {
  name: String
  id:   Int!
  strs: [String]
  refIds: [Int!]!
  someIds: [Int]!
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
                                    fields: vec![
                                        FieldDefinitionNode {
                                            description: None,
                                            name: NameNode {
                                                value: input.get(13..17).unwrap().to_owned()
                                            },
                                            field_type: TypeNode::Named(
                                                            NamedTypeNode {
                                                                name: NameNode {
                                                                    value: input.get(19..25).unwrap().to_owned()
                                                                }
                                                            }
                                                        )

                                        },
                                        FieldDefinitionNode {
                                            description: None,
                                            name: NameNode {
                                                value: input.get(28..30).unwrap().to_owned()
                                            },
                                            field_type: TypeNode::NonNull(
                                                            Rc::new(
                                                                TypeNode::Named(
                                                                    NamedTypeNode {
                                                                        name: NameNode {
                                                                            value: input.get(34..37).unwrap().to_owned()
                                                                        }
                                                                    }
                                                                )
                                                            )
                                                        )

                                        },
                                        FieldDefinitionNode {
                                            description: None,
                                            name: NameNode {
                                                value: String::from("strs")
                                            },
                                            field_type: TypeNode::List(
                                                ListTypeNode {
                                                    list_type: Rc::new(
                                                        TypeNode::Named(
                                                            NamedTypeNode {
                                                                name: NameNode {
                                                                    value: String::from("String")
                                                                }
                                                            }
                                                        )
                                                    )
                                                }
                                            )
                                        },
                                        FieldDefinitionNode {
                                            description: None,
                                            name: NameNode {
                                                value: String::from("refIds")
                                            },
                                            field_type: TypeNode::NonNull(
                                                Rc::new(TypeNode::List(
                                                    ListTypeNode::new(
                                                        TypeNode::NonNull(
                                                            Rc::new(
                                                                TypeNode::Named(
                                                                    NamedTypeNode {
                                                                        name: NameNode {
                                                                            value: String::from("Int")
                                                                        }
                                                                    }
                                                                )
                                                            )
                                                        )
                                                    )
                                                ))
                                            )
                                        },
                                        FieldDefinitionNode {
                                            description: None,
                                            name: NameNode {
                                                value: String::from("someIds")
                                            },
                                            field_type: TypeNode::NonNull(
                                                Rc::new(
                                                    TypeNode::List(
                                                        ListTypeNode::new(
                                                            TypeNode::Named(
                                                                NamedTypeNode {
                                                                    name: NameNode {
                                                                        value: String::from("Int")
                                                                    }
                                                                }
                                                            )
                                                        )
                                                    )
                                                )
                                            )
                                        },
                                    ],
                                }
                            )
                        )
                    )
                ]
            }
        )
    }
}
