//! [![github]](https://github.com/ssedrick/gql.rs)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//!
//! <br>
//!
//! A syntax package for GraphQL parsing and manipulation tokens into a GraphQL Document.
//! This package adheres to the [GraphQL Spec](http://spec.graphql.org/June2018/).
//!
//!

#![warn(trivial_casts, trivial_numeric_casts, unstable_features)]
#![forbid(unsafe_code, missing_docs)]

#[macro_use]
extern crate lazy_static;
mod ast;
pub mod document;
pub mod error;
pub mod lexer;
mod nodes;
pub mod token;
mod validation;

use ast::AST;
use document::Document;
use error::ParseResult;

/// Parse a string into a GraphQL Document.
/// This is a potentially heavy, synchronous operation.
pub fn parse<'a>(query: &'a str) -> ParseResult<Document> {
    let mut ast = AST::new(query)?;
    let document = ast.parse()?;
    Ok(document)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ParseError;
    use crate::nodes::*;
    use crate::token::Token;
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
  arg(arg1: Int = 42, arg2: Bool!): Bool
}"#;
        let res = parse(input);
        println!("res: {:?}", res);
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            Document {
                definitions: vec![DefinitionNode::TypeSystem(TypeSystemDefinitionNode::Type(
                    TypeDefinitionNode::Object(ObjectTypeDefinitionNode {
                        description: None,
                        name: NameNode::from("Obj"),
                        interfaces: None,
                        directives: None,
                        fields: vec![
                            FieldDefinitionNode {
                                description: None,
                                name: NameNode::from("name"),
                                arguments: None,
                                field_type: TypeNode::Named(NamedTypeNode {
                                    name: NameNode::from("String"),
                                })
                            },
                            FieldDefinitionNode {
                                description: None,
                                name: NameNode::from("id"),
                                arguments: None,
                                field_type: TypeNode::NonNull(Rc::new(TypeNode::Named(
                                    NamedTypeNode {
                                        name: NameNode::from("Int")
                                    }
                                )))
                            },
                            FieldDefinitionNode {
                                description: None,
                                name: NameNode::from("strs"),
                                arguments: None,
                                field_type: TypeNode::List(ListTypeNode {
                                    list_type: Rc::new(TypeNode::Named(NamedTypeNode {
                                        name: NameNode::from("String")
                                    }))
                                })
                            },
                            FieldDefinitionNode {
                                description: None,
                                name: NameNode::from("refIds"),
                                arguments: None,
                                field_type: TypeNode::NonNull(Rc::new(TypeNode::List(
                                    ListTypeNode::new(TypeNode::NonNull(Rc::new(TypeNode::Named(
                                        NamedTypeNode {
                                            name: NameNode::from("Int")
                                        }
                                    ))))
                                )))
                            },
                            FieldDefinitionNode {
                                description: None,
                                name: NameNode::from("someIds"),
                                arguments: None,
                                field_type: TypeNode::NonNull(Rc::new(TypeNode::List(
                                    ListTypeNode::new(TypeNode::Named(NamedTypeNode {
                                        name: NameNode::from("Int")
                                    }))
                                )))
                            },
                            FieldDefinitionNode {
                                description: None,
                                name: NameNode::from("arg"),
                                arguments: Some(vec![
                                    InputValueDefinitionNode {
                                        description: None,
                                        name: NameNode::from("arg1"),
                                        input_type: TypeNode::Named(NamedTypeNode {
                                            name: NameNode::from("Int")
                                        }),
                                        default_value: Some(ValueNode::Int(IntValueNode {
                                            value: 42
                                        })),
                                        directives: None,
                                    },
                                    InputValueDefinitionNode {
                                        description: None,
                                        name: NameNode::from("arg2"),
                                        input_type: TypeNode::NonNull(Rc::new(TypeNode::Named(
                                            NamedTypeNode {
                                                name: NameNode::from("Bool")
                                            }
                                        ))),
                                        default_value: None,
                                        directives: None,
                                    },
                                ]),
                                field_type: TypeNode::Named(NamedTypeNode {
                                    name: NameNode::from("Bool")
                                })
                            },
                        ],
                    })
                ))]
            }
        )
    }

    #[test]
    fn parses_documentation() {
        println!("parsing documentation");
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
        assert_eq!(
            res.unwrap(),
            Document {
                definitions: vec![DefinitionNode::TypeSystem(TypeSystemDefinitionNode::Type(
                    TypeDefinitionNode::Object(ObjectTypeDefinitionNode {
                        description: Some(
                            StringValueNode::new(Token::BlockStr(
                                0,
                                0,
                                0,
                                "\nThis is a generic object comment\nThey can be multiple lines\n"
                            ))
                            .unwrap()
                        ),
                        name: NameNode {
                            value: String::from("Obj")
                        },
                        interfaces: None,
                        directives: None,
                        fields: vec![FieldDefinitionNode {
                            description: Some(
                                StringValueNode::new(Token::BlockStr(
                                    0,
                                    0,
                                    0,
                                    "This is the name of the object"
                                ))
                                .unwrap()
                            ),
                            name: NameNode {
                                value: String::from("name")
                            },
                            arguments: None,
                            field_type: TypeNode::Named(NamedTypeNode {
                                name: NameNode {
                                    value: String::from("String")
                                }
                            })
                        },],
                    })
                ))]
            }
        );
    }

    #[test]
    fn it_handles_enums() {
        println!("parsing enums");
        let res = parse(
            r#"enum VEHICLE_TYPE {
  SEDAN
  SUV
  COMPACT
  TRUCK
  HYBRID
}
"#,
        );
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            Document {
                definitions: vec![DefinitionNode::TypeSystem(TypeSystemDefinitionNode::Type(
                    TypeDefinitionNode::Enum(EnumTypeDefinitionNode {
                        description: None,
                        name: NameNode {
                            value: String::from("VEHICLE_TYPE")
                        },
                        directives: None,
                        values: vec![
                            EnumValueDefinitionNode {
                                description: None,
                                name: NameNode {
                                    value: String::from("SEDAN")
                                },
                                directives: None,
                            },
                            EnumValueDefinitionNode {
                                description: None,
                                name: NameNode {
                                    value: String::from("SUV")
                                },
                                directives: None,
                            },
                            EnumValueDefinitionNode {
                                description: None,
                                name: NameNode {
                                    value: String::from("COMPACT")
                                },
                                directives: None,
                            },
                            EnumValueDefinitionNode {
                                description: None,
                                name: NameNode {
                                    value: String::from("TRUCK")
                                },
                                directives: None,
                            },
                            EnumValueDefinitionNode {
                                description: None,
                                name: NameNode {
                                    value: String::from("HYBRID")
                                },
                                directives: None,
                            },
                        ]
                    })
                ))]
            }
        );
    }

    #[test]
    fn parses_union() {
        let res = parse(
            r#"union SearchResult = Photo | Person
union Pic =
  | Gif
  | Jpeg
  | Png
  | Svg
"#,
        );
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            Document {
                definitions: vec![
                    DefinitionNode::TypeSystem(TypeSystemDefinitionNode::Type(
                        TypeDefinitionNode::Union(UnionTypeDefinitionNode {
                            description: None,
                            name: NameNode::from("SearchResult"),
                            directives: None,
                            types: vec![
                                NamedTypeNode::from("Photo"),
                                NamedTypeNode::from("Person"),
                            ]
                        })
                    )),
                    DefinitionNode::TypeSystem(TypeSystemDefinitionNode::Type(
                        TypeDefinitionNode::Union(UnionTypeDefinitionNode {
                            description: None,
                            name: NameNode::from("Pic"),
                            directives: None,
                            types: vec![
                                NamedTypeNode::from("Gif"),
                                NamedTypeNode::from("Jpeg"),
                                NamedTypeNode::from("Png"),
                                NamedTypeNode::from("Svg"),
                            ]
                        })
                    )),
                ]
            }
        );
    }

    #[test]
    fn parses_object_with_interface() {
        println!("Parsing object with interface");
        let res = parse(r#"type Obj implements Named & Sort & Filter { id: ID }"#);
        println!("res: {:?}", res);
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            Document {
                definitions: vec![DefinitionNode::TypeSystem(TypeSystemDefinitionNode::Type(
                    TypeDefinitionNode::Object(ObjectTypeDefinitionNode {
                        description: None,
                        name: NameNode::from("Obj"),
                        interfaces: Some(vec![
                            NamedTypeNode::from("Named"),
                            NamedTypeNode::from("Sort"),
                            NamedTypeNode::from("Filter"),
                        ]),
                        directives: None,
                        fields: vec![FieldDefinitionNode {
                            description: None,
                            arguments: None,
                            name: NameNode::from("id"),
                            field_type: TypeNode::Named(NamedTypeNode::from("ID")),
                        }],
                    })
                ))]
            }
        );
    }

    #[test]
    fn parses_object_with_directives() {
        println!("Parsing object with directives");
        let res = parse(r#"type Obj @depricated @old(allow: false) { id: ID }"#);
        println!("res: {:?}", res);
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            Document {
                definitions: vec![DefinitionNode::TypeSystem(TypeSystemDefinitionNode::Type(
                    TypeDefinitionNode::Object(ObjectTypeDefinitionNode {
                        description: None,
                        name: NameNode::from("Obj"),
                        interfaces: None,
                        directives: Some(vec![
                            DirectiveNode {
                                name: NameNode::from("depricated"),
                                arguments: None
                            },
                            DirectiveNode {
                                name: NameNode::from("old"),
                                arguments: Some(vec![Argument {
                                    name: NameNode::from("allow"),
                                    value: ValueNode::Bool(BooleanValueNode { value: false })
                                }])
                            },
                        ]),
                        fields: vec![FieldDefinitionNode {
                            description: None,
                            arguments: None,
                            name: NameNode::from("id"),
                            field_type: TypeNode::Named(NamedTypeNode::from("ID")),
                        }],
                    })
                ))]
            }
        );
    }

    #[test]
    fn parse_interfaces() {
        let res = parse(
            r#"interface Empty {}
interface Named {
  name: String
}
interface Void @depricated {
  void: Boolean!
}
"#,
        );
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            Document {
                definitions: vec![
                    DefinitionNode::TypeSystem(TypeSystemDefinitionNode::Type(
                        TypeDefinitionNode::Interface(InterfaceTypeDefinitionNode {
                            name: NameNode::from("Empty"),
                            description: None,
                            directives: None,
                            fields: Vec::new(),
                        })
                    )),
                    DefinitionNode::TypeSystem(TypeSystemDefinitionNode::Type(
                        TypeDefinitionNode::Interface(InterfaceTypeDefinitionNode {
                            name: NameNode::from("Named"),
                            description: None,
                            directives: None,
                            fields: vec![FieldDefinitionNode {
                                description: None,
                                name: NameNode::from("name"),
                                arguments: None,
                                field_type: TypeNode::Named(NamedTypeNode::from("String"))
                            }],
                        })
                    )),
                    DefinitionNode::TypeSystem(TypeSystemDefinitionNode::Type(
                        TypeDefinitionNode::Interface(InterfaceTypeDefinitionNode {
                            name: NameNode::from("Void"),
                            description: None,
                            directives: Some(vec![DirectiveNode {
                                name: NameNode::from("depricated"),
                                arguments: None
                            }]),
                            fields: vec![FieldDefinitionNode {
                                description: None,
                                name: NameNode::from("void"),
                                arguments: None,
                                field_type: TypeNode::NonNull(Rc::new(TypeNode::Named(
                                    NamedTypeNode::from("Boolean")
                                )))
                            }],
                        })
                    )),
                ]
            }
        )
    }

    #[test]
    fn parses_input_type() {
        let res = parse(
            r#"
input Point {
  x: Float
  y: Float
}
"#,
        );
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            Document {
                definitions: vec![DefinitionNode::TypeSystem(TypeSystemDefinitionNode::Type(
                    TypeDefinitionNode::Input(InputTypeDefinitionNode {
                        description: None,
                        name: NameNode::from("Point"),
                        fields: vec![
                            InputValueDefinitionNode {
                                description: None,
                                name: NameNode::from("x"),
                                input_type: TypeNode::Named(NamedTypeNode::from("Float")),
                                default_value: None,
                                directives: None
                            },
                            InputValueDefinitionNode {
                                description: None,
                                name: NameNode::from("y"),
                                input_type: TypeNode::Named(NamedTypeNode::from("Float")),
                                default_value: None,
                                directives: None
                            },
                        ],
                    })
                ))]
            }
        )
    }

    #[test]
    fn parses_scalars() {
        let res = parse(
            r#"scalar Date
"""Time is represented by a string"""
scalar Time @format(pattern: "HH:mm:ss")"#,
        );
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            Document {
                definitions: vec![
                    DefinitionNode::TypeSystem(TypeSystemDefinitionNode::Type(
                        TypeDefinitionNode::Scalar(ScalarTypeDefinitionNode {
                            description: None,
                            name: NameNode::from("Date"),
                            directives: None,
                        })
                    )),
                    DefinitionNode::TypeSystem(TypeSystemDefinitionNode::Type(
                        TypeDefinitionNode::Scalar(ScalarTypeDefinitionNode {
                            description: Some(StringValueNode::from(
                                "Time is represented by a string",
                                true
                            )),
                            name: NameNode::from("Time"),
                            directives: Some(vec![DirectiveNode {
                                name: NameNode::from("format"),
                                arguments: Some(vec![Argument {
                                    name: NameNode::from("pattern"),
                                    value: ValueNode::Str(StringValueNode::from("HH:mm:ss", false))
                                }])
                            }]),
                        })
                    )),
                ]
            }
        )
    }

    #[test]
    fn parses_object_extension() {
        let res = parse(
            r#"extend type Obj implements Timestamped @addedDirective { createdOn: DateTime, updatedOn: DateTime }
            extend type Admin implements Sudo & Root
            extend type User @accessLevel
            "#,
        );
        println!("res: {:?}", res);
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            Document {
                definitions: vec![
                    DefinitionNode::Extension(TypeSystemExtensionNode::Object(
                        ObjectTypeExtensionNode {
                            description: None,
                            name: NameNode::from("Obj"),
                            interfaces: Some(vec![NamedTypeNode::from("Timestamped")]),
                            directives: Some(vec![DirectiveNode {
                                name: NameNode::from("addedDirective"),
                                arguments: None,
                            }]),
                            fields: Some(vec![
                                FieldDefinitionNode {
                                    arguments: None,
                                    description: None,
                                    name: NameNode::from("createdOn"),
                                    field_type: TypeNode::Named(NamedTypeNode::from("DateTime")),
                                },
                                FieldDefinitionNode {
                                    arguments: None,
                                    description: None,
                                    name: NameNode::from("updatedOn"),
                                    field_type: TypeNode::Named(NamedTypeNode::from("DateTime")),
                                },
                            ]),
                        }
                    )),
                    DefinitionNode::Extension(TypeSystemExtensionNode::Object(
                        ObjectTypeExtensionNode {
                            description: None,
                            name: NameNode::from("Admin"),
                            interfaces: Some(vec![
                                NamedTypeNode::from("Sudo"),
                                NamedTypeNode::from("Root")
                            ]),
                            directives: None,
                            fields: None,
                        }
                    )),
                    DefinitionNode::Extension(TypeSystemExtensionNode::Object(
                        ObjectTypeExtensionNode {
                            description: None,
                            name: NameNode::from("User"),
                            interfaces: None,
                            directives: Some(vec![DirectiveNode {
                                name: NameNode::from("accessLevel"),
                                arguments: None
                            }]),
                            fields: None,
                        }
                    ))
                ],
            }
        );
    }
}
