use crate::token::Token;
use crate::error::{ParseError, ParseResult};
use std::rc::Rc;


#[derive(Debug, PartialEq)]
pub struct NameNode {
    pub value: String
}
impl NameNode {
    pub fn new(token: Token) -> Result<NameNode, ParseError> {
        match token {
            Token::Name(_, _, _, value) => Ok(
                NameNode {
                    value: value.to_owned(),
                }
            ),
            _ => Err(ParseError::UnexpectedToken {
                expected: String::from("Token<Name>"),
                received: token.to_string().to_owned() })
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct StringValueNode {
    pub value: String
}

impl StringValueNode {
    pub fn new(token: Token) -> Result<StringValueNode, ParseError> {
        match token {
            Token::Str(_, _, _, val) |
            Token::BlockStr(_, _, _, val) => Ok(
                StringValueNode {
                    value: val.to_owned(),
                }
            ),
            _ => Err(ParseError::UnexpectedToken {
                expected: String::from("Token<Str> or Token<BlockStr>"),
                received: token.to_string().to_owned()
            })
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ListTypeNode {
    pub list_type: Rc<TypeNode>
}

impl ListTypeNode {
    pub fn new(list_type: TypeNode) -> ListTypeNode {
        ListTypeNode {
            list_type: Rc::new(list_type),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TypeNode {
    Named(NamedTypeNode),
    List(ListTypeNode),
    NonNull(Rc<TypeNode>),
}

// #[derive(Debug, PartialEq)]
// pub enum ValueNode {
//     Variable(VariableValueNode),
//     Int(IntValueNode)
// }


#[derive(Debug, PartialEq)]
pub struct InputValueDefinitionNode {
    pub description: Description,
    pub name: NameNode,
    pub input_type: TypeNode,
    // pub default_value: ValueNode,
    // pub directives: Directives
}

pub type Description = Option<StringValueNode>;
pub type Arguments = Option<Vec<InputValueDefinitionNode>>;

// #[derive(Debug)]
// enum ValueNode {
//     String(StringValueNode),
// }

#[derive(Debug, PartialEq)]
pub struct NamedTypeNode {
    pub name: NameNode
}

impl NamedTypeNode {
    pub fn new(tok: Token) -> Result<NamedTypeNode, ParseError> {
        Ok(NamedTypeNode {
            name: NameNode::new(tok)?,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct FieldDefinitionNode {
    pub description: Description,
    pub name: NameNode,
    pub arguments: Arguments,
    pub field_type: TypeNode,
    // directives: Vec<DirectiveDefinitionNode>,
}

impl FieldDefinitionNode {
    pub fn new(name: Token, field_type: TypeNode, description: Description) -> Result<FieldDefinitionNode, ParseError> {
        Ok(FieldDefinitionNode {
            description,
            name: NameNode::new(name)?,
            arguments: None,
            field_type,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct EnumValueDefinitionNode {
    pub description: Description,
    pub name: NameNode,
    // directives: Option<Vec<DirectiveDefinitionNode>>
}

impl EnumValueDefinitionNode {
    pub fn new(name: Token, description: Description) -> ParseResult<EnumValueDefinitionNode> {
        Ok(EnumValueDefinitionNode {
            description,
            name: NameNode::new(name)?,
        })
    }
}

// struct Location<'a> {
//     start: usize,
//     end: usize,
//     startToken: Token<'a>,
//     endToken: Token<'a>,
//     source: &'a str
// }

// const OPERATION: &'static str = "Operation";
// pub struct OperationDefinitionNode {
//     kind: OPERATION,
//     // location: Location,
//     operation: OperationTypeNode,
//     name: Option<Token>,
//     variables: Vec<VariableDefinitionNode>,
//     directives: Vec<DirectiveDefinitionNode>,
//     selection_set: Vec<SelectionSetNode>
// }

// pub enum ExecutableDefinitionNode {
//     Operation(OperationDefinitionNode),
    // Fragment(FragmentDefinitionNode),
// }
//
//

const SCHEMA: &'static str = "SchemaDefinition";
#[derive(Debug, PartialEq)]
pub struct SchemaDefinitionNode {
    kind: &'static str,
    description: Description,
    // directives: Vec<DirectiveDefinitionNode>,
    // operations: Vec<OperationTypeDefinitionNode>,
}
impl SchemaDefinitionNode {
    pub fn new() -> SchemaDefinitionNode {
        SchemaDefinitionNode {
            kind: SCHEMA,
            description: None
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ScalarTypeDefinitionNode {
    description: Description,
    name: NameNode,
    // directives: Vec<DirectiveDefinitionNode>
}

impl ScalarTypeDefinitionNode {
    pub fn new(tok: Token) -> Result<ScalarTypeDefinitionNode, ParseError> {
        let name = NameNode::new(tok)?;
        Ok(ScalarTypeDefinitionNode {
            description: None,
            name
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct ObjectTypeDefinitionNode {
    pub description: Description,
    pub name: NameNode,
    // interfaces: Vec<NamedTypeNode>,
    // directives: Vec<DirectiveDefinitionNode>,
    pub fields: Vec<FieldDefinitionNode>
}

impl ObjectTypeDefinitionNode {
    pub fn new(tok: Token, description: Description, fields: Vec<FieldDefinitionNode>) -> Result<ObjectTypeDefinitionNode, ParseError> {
        Ok(ObjectTypeDefinitionNode {
            description,
            name: NameNode::new(tok)?,
            fields
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct EnumTypeDefinitionNode {
    pub description: Description,
    pub name: NameNode,
    // directives: Vec<DirectiveDefinitionNode>,
    pub values: Vec<EnumValueDefinitionNode>
}

impl EnumTypeDefinitionNode {
    pub fn new(tok: Token, description: Description, values: Vec<EnumValueDefinitionNode>) -> Result<EnumTypeDefinitionNode, ParseError> {
        Ok(EnumTypeDefinitionNode {
            description,
            name: NameNode::new(tok)?,
            values,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum TypeDefinitionNode {
    Scalar(ScalarTypeDefinitionNode),
    Object(ObjectTypeDefinitionNode),
    // Interface(InterfaceTypeDefinitionNode)
    // Union(UnionTypeDefinitionNode)
    Enum(EnumTypeDefinitionNode)
    // Input(InputObjectTypeDefinitionNode)
}

#[derive(Debug, PartialEq)]
pub enum TypeSystemDefinitionNode {
    Schema(SchemaDefinitionNode),
    Type(TypeDefinitionNode),
    // Directive(DirectiveDefinitionNode),
}

#[derive(Debug, PartialEq)]
pub enum DefinitionNode {
    // Executable(ExecutableDefinitionNode),
    TypeSystem(TypeSystemDefinitionNode),
    // Extension(TypeSystemExtensionNode),
}

#[derive(Debug, PartialEq)]
pub struct Document {
    pub definitions: Vec<DefinitionNode>,
}
impl Document {
    pub fn new(definitions: Vec<DefinitionNode>) -> Document {
        Document {
            definitions
        }
    }
}
