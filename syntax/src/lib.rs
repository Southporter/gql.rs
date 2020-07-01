#[macro_use] extern crate lazy_static;
pub mod token;
pub mod lexer;
pub mod ast;
pub mod error;
mod nodes;

use ast::AST;
use nodes::Document;
use error::ParseResult;

pub fn parse<'a>(query: &'a str) -> ParseResult<Document>
{
    let mut ast = AST::new(query)?;
    let document = ast.parse()?;
    Ok(document)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::{Document, DefinitionNode, TypeSystemDefinitionNode, TypeDefinitionNode, ObjectTypeDefinitionNode, NameNode, FieldDefinitionNode, TypeNode, NamedTypeNode, ListTypeNode};
    use crate::error::ParseError;
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
                                        value: String::from("Obj")
                                    },
                                    fields: vec![
                                        FieldDefinitionNode {
                                            description: None,
                                            name: NameNode {
                                                value: String::from("name")
                                            },
                                            field_type: TypeNode::Named(
                                                            NamedTypeNode {
                                                                name: NameNode {
                                                                    value: String::from("String")
                                                                }
                                                            }
                                                        )

                                        },
                                        FieldDefinitionNode {
                                            description: None,
                                            name: NameNode {
                                                value: String::from("id")
                                            },
                                            field_type: TypeNode::NonNull(
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
