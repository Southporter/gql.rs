use crate::lexer::LexErrorKind;
use crate::token::Token;
use crate::ast::ParseError;

#[derive(Debug)]
struct NameNode<'a> {
    value: &'a str
}
impl<'a> NameNode<'a> {
    pub fn new(token: Token) -> Result<NameNode, ParseError> {
        match token {
            Token::Name(_, _, _, value) => Ok(
                NameNode {
                    value
                }
            ),
            _ => Err(ParseError::UnexpectedToken { expected: "Token<Name>", received: token.to_string().to_owned() })
        }
    }
}

const STRING: &'static str = "StringValue";
#[derive(Debug)]
struct StringValueNode<'a> {
    kind: &'static str,
    value: &'a str
}

impl<'a> StringValueNode<'a> {
    pub fn new(token: Token) -> Result<StringValueNode, ParseError> {
        match token {
            Token::Str(_, _, _, value) |
            Token::BlockStr(_, _, _, value) => Ok(
                StringValueNode {
                    kind: STRING,
                    value,
                }
            ),
            _ => Err(ParseError::UnexpectedToken {
                expected: "Token<Str> or Token<BlockStr>",
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
#[derive(Debug)]
struct SchemaDefinitionNode<'a> {
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

#[derive(Debug)]
struct ScalarTypeDefinitionNode<'a> {
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

#[derive(Debug)]
struct ObjectTypeDefinitionNode<'a> {
    description: Option<StringValueNode<'a>>,
    name: NameNode<'a>,
    // interfaces: Vec<NamedTypeNode>,
    // directives: Vec<DirectiveDefinitionNode>,
    // fields: Vec<FieldDefinitionNode<'a>>
}

impl<'a> ObjectTypeDefinitionNode<'a> {
    pub fn new(tok: Token) -> Result<ObjectTypeDefinitionNode, ParseError> {
        let name = NameNode::new(tok)?;
        Ok(ObjectTypeDefinitionNode {
            description: None,
            name,
        })
    }
}



#[derive(Debug)]
enum TypeDefinitionNode<'a> {
    Scalar(ScalarTypeDefinitionNode<'a>),
    Object(ObjectTypeDefinitionNode<'a>),
    // Interface(InterfaceTypeDefinitionNode)
    // Union(UnionTypeDefinitionNode)
    // Enum(EnumTypeDefinitionNode)
    // Input(InputObjectTypeDefinitionNode)
}

#[derive(Debug)]
enum TypeSystemDefinitionNode<'a> {
    Schema(SchemaDefinitionNode<'a>),
    Type(TypeDefinitionNode<'a>),
    // Directive(DirectiveDefinitionNode),
}

#[derive(Debug)]
enum DefinitionNode<'a> {
    // Executable(ExecutableDefinitionNode),
    TypeSystem(TypeSystemDefinitionNode<'a>),
    // Extension(TypeSystemExtensionNode),
}

fn parse_definitions(_iter: Lex) -> Result<Vec<DefinitionNode>, ParseError> {
    Err(ParseError::DocumentEmpty)
}

type Lex<'i> = Box<dyn Iterator<Item = Result<Token<'i>, LexErrorKind>> + 'i>;

#[derive(Debug)]
pub struct Document<'a> {
    definitions: Vec<DefinitionNode<'a>>,
}
impl<'a> Document<'a> {
    pub fn new(iter: Lex<'a>) -> Result<Document<'a>, ParseError> {
        let definitions = parse_definitions(iter)?;
        Ok(Document {
            definitions
        })
    }
}
