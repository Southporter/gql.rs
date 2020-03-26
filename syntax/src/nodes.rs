use crate::token::Token;
use crate::ast::ParseError;

#[derive(Debug, PartialEq)]
pub struct NameNode<'a> {
    pub value: &'a str
}
impl<'a> NameNode<'a> {
    pub fn new(token: Token) -> Result<NameNode, ParseError> {
        match token {
            Token::Name(_, _, _, value) => Ok(
                NameNode {
                    value
                }
            ),
            _ => Err(ParseError::UnexpectedToken {
                expected: String::from("Token<Name>"),
                received: token.to_string().to_owned() })
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct StringValueNode<'a> {
    pub value: &'a str
}

impl<'a> StringValueNode<'a> {
    pub fn new(token: Token) -> Result<StringValueNode, ParseError> {
        match token {
            Token::Str(_, _, _, value) |
            Token::BlockStr(_, _, _, value) => Ok(
                StringValueNode {
                    value,
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
enum ValueNode<'a> {
    String(StringValueNode<'a>),
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
pub struct SchemaDefinitionNode<'a> {
    kind: &'static str,
    description: Option<StringValueNode<'a>>,
    // directives: Vec<DirectiveDefinitionNode>,
    // operations: Vec<OperationTypeDefinitionNode>,
}
impl<'a> SchemaDefinitionNode<'a> {
    pub fn new() -> SchemaDefinitionNode<'a> {
        SchemaDefinitionNode {
            kind: SCHEMA,
            description: None
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ScalarTypeDefinitionNode<'a> {
    description: Option<StringValueNode<'a>>,
    name: NameNode<'a>,
    // directives: Vec<DirectiveDefinitionNode>
}

impl<'a> ScalarTypeDefinitionNode<'a> {
    pub fn new(tok: Token) -> Result<ScalarTypeDefinitionNode, ParseError> {
        let name = NameNode::new(tok)?;
        Ok(ScalarTypeDefinitionNode {
            description: None,
            name
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct ObjectTypeDefinitionNode<'a> {
    pub description: Option<StringValueNode<'a>>,
    pub name: NameNode<'a>,
    // interfaces: Vec<NamedTypeNode>,
    // directives: Vec<DirectiveDefinitionNode>,
    // fields: Vec<FieldDefinitionNode<'a>>
}

impl<'a> ObjectTypeDefinitionNode<'a> {
    pub fn new(tok: Token<'a>) -> Result<ObjectTypeDefinitionNode<'a>, ParseError> {
        Ok(ObjectTypeDefinitionNode {
            description: None,
            name: NameNode::new(tok)?
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum TypeDefinitionNode<'a> {
    Scalar(ScalarTypeDefinitionNode<'a>),
    Object(ObjectTypeDefinitionNode<'a>),
    // Interface(InterfaceTypeDefinitionNode)
    // Union(UnionTypeDefinitionNode)
    // Enum(EnumTypeDefinitionNode)
    // Input(InputObjectTypeDefinitionNode)
}

#[derive(Debug, PartialEq)]
pub enum TypeSystemDefinitionNode<'a> {
    Schema(SchemaDefinitionNode<'a>),
    Type(TypeDefinitionNode<'a>),
    // Directive(DirectiveDefinitionNode),
}

#[derive(Debug, PartialEq)]
pub enum DefinitionNode<'a> {
    // Executable(ExecutableDefinitionNode),
    TypeSystem(TypeSystemDefinitionNode<'a>),
    // Extension(TypeSystemExtensionNode),
}

#[derive(Debug, PartialEq)]
pub struct Document<'a> {
    pub definitions: Vec<DefinitionNode<'a>>,
}
impl<'a> Document<'a> {
    pub fn new(definitions: Vec<DefinitionNode>) -> Document {
        Document {
            definitions
        }
    }
}
