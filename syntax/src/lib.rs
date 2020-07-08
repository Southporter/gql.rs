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
    // use crate::nodes::{Document, DefinitionNode, TypeSystemDefinitionNode, TypeDefinitionNode, ObjectTypeDefinitionNode, NameNode, FieldDefinitionNode, TypeNode, NamedTypeNode, ListTypeNode, StringValueNode};
    use crate::nodes::*;
    use crate::error::ParseError;
    use std::rc::Rc;

    #[test]
    fn it_handles_empty_document() {
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

    #[test]
<<<<<<< HEAD
    fn parses_documentation() {
        println!("parsing an object");
        let input = r#"
"""
This is a generic object comment
They can be multiple lines
"""
type Obj {
  """This is the name of the object"""
  name: String
}"#;
        let res = parse(input);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Document {
            definitions: vec![
                DefinitionNode::TypeSystem(
                    TypeSystemDefinitionNode::Type(
                        TypeDefinitionNode::Object(
                            ObjectTypeDefinitionNode {
                                description: Some(StringValueNode {
                                    value: String::from("\nThis is a generic object comment\nThey can be multiple lines\n")
                                }),
                                name: NameNode {
                                    value: String::from("Obj")
                                },
                                fields: vec![
                                    FieldDefinitionNode {
                                        description: Some(StringValueNode {
                                            value: String::from("This is the name of the object")
                                        }),
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
                                ],
                            }
                        )
                    )
                )
            ]
        });
    }

    #[test]
    fn it_handles_enums() {
        println!("parsing enums");
        let res = parse(r#"enum VEHICLE_TYPE {
  SEDAN
  SUV
  COMPACT
  TRUCK
  HYBRID
}
"#);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(),
            Document {
                definitions: vec![
                ]
            }
        );
    }
}
