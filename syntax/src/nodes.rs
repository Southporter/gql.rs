use crate::token::Token;
use crate::ast::ParseError;

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

#[derive(Debug)]
enum ValueNode {
    String(StringValueNode),
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
    description: Option<StringValueNode>,
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
    description: Option<StringValueNode>,
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
    pub description: Option<StringValueNode>,
    pub name: NameNode,
    // interfaces: Vec<NamedTypeNode>,
    // directives: Vec<DirectiveDefinitionNode>,
    // fields: Vec<FieldDefinitionNode>
}

impl ObjectTypeDefinitionNode {
    pub fn new(tok: Token) -> Result<ObjectTypeDefinitionNode, ParseError> {
        Ok(ObjectTypeDefinitionNode {
            description: None,
            name: NameNode::new(tok)?,
            // fields: vec![]
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum TypeDefinitionNode {
    Scalar(ScalarTypeDefinitionNode),
    Object(ObjectTypeDefinitionNode),
    // Interface(InterfaceTypeDefinitionNode)
    // Union(UnionTypeDefinitionNode)
    // Enum(EnumTypeDefinitionNode)
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
