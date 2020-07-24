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
    use crate::nodes::*;
    use crate::token::Token;
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
  arg(arg1: Int = 42, arg2: Bool!): Bool
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
                                    name: NameNode::from("Obj"),
                                    fields: vec![
                                        FieldDefinitionNode {
                                            description: None,
                                            name: NameNode::from("name"),
                                            arguments: None,
                                            field_type: TypeNode::Named(
                                                NamedTypeNode {
                                                    name: NameNode::from("String"),
                                                }
                                            )
                                        },
                                        FieldDefinitionNode {
                                            description: None,
                                            name: NameNode::from("id"),
                                            arguments: None,
                                            field_type: TypeNode::NonNull(
                                                Rc::new(
                                                    TypeNode::Named(
                                                        NamedTypeNode {
                                                            name: NameNode::from("Int")
                                                        }
                                                    )
                                                )
                                            )
                                        },
                                        FieldDefinitionNode {
                                            description: None,
                                            name: NameNode::from("strs"),
                                            arguments: None,
                                            field_type: TypeNode::List(
                                                ListTypeNode {
                                                    list_type: Rc::new(
                                                        TypeNode::Named(
                                                            NamedTypeNode {
                                                                name: NameNode::from("String")
                                                            }
                                                        )
                                                    )
                                                }
                                            )
                                        },
                                        FieldDefinitionNode {
                                            description: None,
                                            name: NameNode::from("refIds"),
                                            arguments: None,
                                            field_type: TypeNode::NonNull(
                                                Rc::new(TypeNode::List(
                                                    ListTypeNode::new(
                                                        TypeNode::NonNull(
                                                            Rc::new(
                                                                TypeNode::Named(
                                                                    NamedTypeNode {
                                                                        name: NameNode::from("Int")
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
                                            name: NameNode::from("someIds"),
                                            arguments: None,
                                            field_type: TypeNode::NonNull(
                                                Rc::new(
                                                    TypeNode::List(
                                                        ListTypeNode::new(
                                                            TypeNode::Named(
                                                                NamedTypeNode {
                                                                    name: NameNode::from("Int")
                                                                }
                                                            )
                                                        )
                                                    )
                                                )
                                            )
                                        },
                                        FieldDefinitionNode {
                                            description: None,
                                            name: NameNode::from("arg"),
                                            arguments: Some(vec![
                                                InputValueDefinitionNode {
                                                    description: None,
                                                    name: NameNode::from("arg1"),
                                                    input_type: TypeNode::Named(NamedTypeNode { name: NameNode::from("Int") }),
                                                    default_value: Some(ValueNode ::Int(IntValueNode { value: 42 })),
                                                },
                                                InputValueDefinitionNode {
                                                    description: None,
                                                    name: NameNode::from("arg2"),
                                                    input_type: TypeNode::NonNull(Rc::new(TypeNode::Named(NamedTypeNode { name: NameNode::from("Bool") }))),
                                                    default_value: None,
                                                },
                                            ]),
                                            field_type: TypeNode::Named(
                                                NamedTypeNode {
                                                    name: NameNode::from("Bool")
                                                }
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
        assert_eq!(res.unwrap(), Document {
            definitions: vec![
                DefinitionNode::TypeSystem(
                    TypeSystemDefinitionNode::Type(
                        TypeDefinitionNode::Object(
                            ObjectTypeDefinitionNode {
                                description: Some(StringValueNode::new(Token::BlockStr(0, 0, 0, "\nThis is a generic object comment\nThey can be multiple lines\n")).unwrap()),
                                name: NameNode {
                                    value: String::from("Obj")
                                },
                                fields: vec![
                                    FieldDefinitionNode {
                                        description: Some(StringValueNode::new(Token::BlockStr(0,0,0,"This is the name of the object")).unwrap()),
                                        name: NameNode {
                                            value: String::from("name")
                                        },
                                        arguments: None,
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
                    DefinitionNode::TypeSystem(
                        TypeSystemDefinitionNode::Type(
                            TypeDefinitionNode::Enum(
                                EnumTypeDefinitionNode {
                                    description: None,
                                    name: NameNode {
                                        value: String::from("VEHICLE_TYPE")
                                    },
                                    values: vec![
                                        EnumValueDefinitionNode {
                                            description: None,
                                            name: NameNode {
                                                value: String::from("SEDAN")
                                            }
                                        },
                                        EnumValueDefinitionNode {
                                            description: None,
                                            name: NameNode {
                                                value: String::from("SUV")
                                            }
                                        },
                                        EnumValueDefinitionNode {
                                            description: None,
                                            name: NameNode {
                                                value: String::from("COMPACT")
                                            }
                                        },
                                        EnumValueDefinitionNode {
                                            description: None,
                                            name: NameNode {
                                                value: String::from("TRUCK")
                                            }
                                        },
                                        EnumValueDefinitionNode {
                                            description: None,
                                            name: NameNode {
                                                value: String::from("HYBRID")
                                            }
                                        },
                                    ]
                                }
                            )
                        )
                    )
                ]
            }
        );
    }
}
