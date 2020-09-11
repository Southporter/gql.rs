use crate::error::{ParseError, ParseResult, ValidationError};
use crate::token::Token;
use crate::validation::{self, ValidExtensionNode, ValidNode, ValidationResult};
use std::rc::Rc;

pub trait NodeWithFields {
    fn get_fields(&self) -> &[FieldDefinitionNode] {
        &[]
    }
}

#[derive(Debug, PartialEq)]
pub struct NameNode {
    pub value: String,
}
impl NameNode {
    /// Generates a new name node from the itoken.
    /// If the token is not of type Token::Name,
    /// an error is thrown
    pub fn new(token: Token) -> ParseResult<NameNode> {
        match token {
            Token::Name(_, _, _, value) => Ok(NameNode {
                value: value.to_owned(),
            }),
            _ => Err(ParseError::UnexpectedToken {
                expected: String::from("Token<Name>"),
                received: token.to_string().to_owned(),
            }),
        }
    }

    /// Used internally for testing. No error is thrown.
    pub fn from(name: &str) -> NameNode {
        NameNode {
            value: String::from(name),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct StringValueNode {
    pub value: String,
    block: bool,
}

impl StringValueNode {
    pub fn new(token: Token) -> ParseResult<StringValueNode> {
        match token {
            Token::Str(_, _, _, val) => Ok(StringValueNode {
                value: val.to_owned(),
                block: false,
            }),
            Token::BlockStr(_, _, _, val) => Ok(StringValueNode {
                value: val.to_owned(),
                block: true,
            }),
            _ => Err(ParseError::UnexpectedToken {
                expected: String::from("Token<Str> or Token<BlockStr>"),
                received: token.to_string().to_owned(),
            }),
        }
    }

    pub fn from(content: &str, block: bool) -> StringValueNode {
        StringValueNode {
            value: String::from(content),
            block,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct NamedTypeNode {
    pub name: NameNode,
}

impl NamedTypeNode {
    /// Generates a NamedTypeNode from the token.
    /// NameNode will throw an error if the token is not
    /// of type Token::Name
    pub fn new(tok: Token) -> ParseResult<NamedTypeNode> {
        Ok(NamedTypeNode {
            name: NameNode::new(tok)?,
        })
    }

    /// Used for internal testing.
    pub fn from(name: &str) -> NamedTypeNode {
        NamedTypeNode {
            name: NameNode::from(name),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ListTypeNode {
    pub list_type: Rc<TypeNode>,
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

#[derive(Debug, PartialEq)]
pub struct VariableNode {
    pub name: NameNode,
}

impl VariableNode {
    pub fn new(tok: Token) -> ParseResult<VariableNode> {
        Ok(VariableNode {
            name: NameNode::new(tok)?,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct IntValueNode {
    pub value: i64,
}

#[derive(Debug, PartialEq)]
pub struct FloatValueNode {
    pub value: f64,
}

#[derive(Debug, PartialEq)]
pub struct BooleanValueNode {
    pub value: bool,
}

#[derive(Debug, PartialEq)]
pub struct EnumValueNode {
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct ListValueNode {
    pub values: Vec<ValueNode>,
}

#[derive(Debug, PartialEq)]
pub struct ObjectFieldNode {
    pub name: NameNode,
    pub value: ValueNode,
}

#[derive(Debug, PartialEq)]
pub struct ObjectValueNode {
    pub fields: Vec<ObjectFieldNode>,
}

#[derive(Debug, PartialEq)]
pub enum ValueNode {
    Variable(VariableNode),
    Int(IntValueNode),
    Float(FloatValueNode),
    Str(StringValueNode),
    Bool(BooleanValueNode),
    Null,
    Enum(EnumValueNode),
    List(ListValueNode),
    Object(ObjectValueNode),
}

#[derive(Debug, PartialEq)]
pub struct DirectiveNode {
    pub name: NameNode,
    pub arguments: Option<Arguments>,
}

impl DirectiveNode {
    pub fn new(name: Token, arguments: Option<Arguments>) -> ParseResult<DirectiveNode> {
        Ok(DirectiveNode {
            name: NameNode::new(name)?,
            arguments,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct InputValueDefinitionNode {
    pub description: Description,
    pub name: NameNode,
    pub input_type: TypeNode,
    pub default_value: Option<ValueNode>,
    pub directives: Option<Directives>,
}

impl InputValueDefinitionNode {
    pub fn new(
        name: Token,
        input_type: TypeNode,
        description: Description,
    ) -> ParseResult<InputValueDefinitionNode> {
        Ok(InputValueDefinitionNode {
            description,
            name: NameNode::new(name)?,
            input_type,
            default_value: None,
            directives: None,
        })
    }

    pub fn with_default_value(&mut self, default_value: Option<ValueNode>) -> &mut Self {
        self.default_value = default_value;
        self
    }

    pub fn with_directives(&mut self, directives: Option<Directives>) -> &mut Self {
        self.directives = directives;
        self
    }
}

#[derive(Debug, PartialEq)]
pub struct Argument {
    pub name: NameNode,
    pub value: ValueNode,
}

pub type Description = Option<StringValueNode>;
pub type Arguments = Vec<Argument>;
pub type ArgumentDefinitions = Vec<InputValueDefinitionNode>;
pub type Directives = Vec<DirectiveNode>;

// #[derive(Debug)]
// enum ValueNode {
//     String(StringValueNode),
// }
//

#[derive(Debug, PartialEq)]
pub struct FieldDefinitionNode {
    pub description: Description,
    pub name: NameNode,
    pub arguments: Option<ArgumentDefinitions>,
    pub field_type: TypeNode,
    // directives: Vec<DirectiveDefinitionNode>,
}

impl FieldDefinitionNode {
    pub fn new(
        name: Token,
        field_type: TypeNode,
        description: Description,
        arguments: Option<ArgumentDefinitions>,
    ) -> ParseResult<FieldDefinitionNode> {
        Ok(FieldDefinitionNode {
            description,
            name: NameNode::new(name)?,
            arguments: arguments,
            field_type,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct EnumValueDefinitionNode {
    pub description: Description,
    pub name: NameNode,
    pub directives: Option<Directives>,
}

impl EnumValueDefinitionNode {
    pub fn new(
        name: Token,
        description: Description,
        directives: Option<Directives>,
    ) -> ParseResult<EnumValueDefinitionNode> {
        Ok(EnumValueDefinitionNode {
            description,
            name: NameNode::new(name)?,
            directives,
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
            description: None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ScalarTypeDefinitionNode {
    pub description: Description,
    pub name: NameNode,
    pub directives: Option<Directives>,
}

impl ScalarTypeDefinitionNode {
    pub fn new(tok: Token, description: Description) -> ParseResult<ScalarTypeDefinitionNode> {
        let name = NameNode::new(tok)?;
        Ok(ScalarTypeDefinitionNode {
            description,
            name,
            directives: None,
        })
    }

    pub fn with_directives(&mut self, directives: Option<Directives>) -> &mut Self {
        self.directives = directives;
        self
    }
}

#[derive(Debug, PartialEq)]
pub struct ObjectTypeDefinitionNode {
    pub description: Description,
    pub name: NameNode,
    pub interfaces: Option<Vec<NamedTypeNode>>,
    pub directives: Option<Directives>,
    pub fields: Vec<FieldDefinitionNode>,
}

impl ObjectTypeDefinitionNode {
    pub fn new(
        tok: Token,
        description: Description,
        fields: Vec<FieldDefinitionNode>,
    ) -> ParseResult<Self> {
        if !fields.is_empty() {
            Ok(ObjectTypeDefinitionNode {
                description,
                name: NameNode::new(tok)?,
                interfaces: None,
                directives: None,
                fields,
            })
        } else {
            Err(ParseError::ObjectEmpty)
        }
    }

    pub fn with_interfaces(&mut self, interfaces: Option<Vec<NamedTypeNode>>) -> &mut Self {
        self.interfaces = interfaces;
        self
    }

    pub fn with_directives(&mut self, directives: Option<Directives>) -> &mut Self {
        self.directives = directives;
        self
    }

    pub fn with_fields(&mut self, fields: Vec<FieldDefinitionNode>) -> &mut Self {
        self.fields = fields;
        self
    }
}

impl NodeWithFields for ObjectTypeDefinitionNode {
    fn get_fields(&self) -> &[FieldDefinitionNode] {
        &self.fields
    }
}

#[derive(Debug, PartialEq)]
pub struct ObjectTypeExtensionNode {
    pub description: Description,
    pub name: NameNode,
    pub interfaces: Option<Vec<NamedTypeNode>>,
    pub directives: Option<Directives>,
    pub fields: Option<Vec<FieldDefinitionNode>>,
}

impl ObjectTypeExtensionNode {
    pub fn new(tok: Token, description: Description) -> ParseResult<ObjectTypeExtensionNode> {
        Ok(ObjectTypeExtensionNode {
            description,
            name: NameNode::new(tok)?,
            interfaces: None,
            directives: None,
            fields: None,
        })
    }

    pub fn with_interfaces(&mut self, interfaces: Option<Vec<NamedTypeNode>>) -> &mut Self {
        self.interfaces = interfaces;
        self
    }

    pub fn with_directives(&mut self, directives: Option<Directives>) -> &mut Self {
        self.directives = directives;
        self
    }

    pub fn with_fields(&mut self, fields: Vec<FieldDefinitionNode>) -> &mut Self {
        self.fields = Some(fields);
        self
    }
}

impl NodeWithFields for ObjectTypeExtensionNode {
    fn get_fields(&self) -> &[FieldDefinitionNode] {
        if let Some(fields) = &self.fields {
            &fields
        } else {
            &[]
        }
    }
}

impl ValidNode for ObjectTypeExtensionNode {
    fn validate(&self) -> ValidationResult {
        if !(self.directives.is_none() && self.interfaces.is_none() && self.fields.is_none()) {
            Ok(())
        } else {
            Err(ValidationError::new("Object Extension must have at least one of the following: Directive, Interface, or Field"))
        }
    }
}

impl ValidExtensionNode<ObjectTypeDefinitionNode> for ObjectTypeExtensionNode {
    fn validate_extension(&self, original: Option<&ObjectTypeDefinitionNode>) -> ValidationResult {
        if let Some(obj) = original {
            validation::validate_extension_fields_against_original(self, obj)?;
            Ok(())
        } else {
            Err(ValidationError::new(
                format!(
                    "Invalid Object Extension {0}: No type of name {0} in schema",
                    self.name.value
                )
                .as_str(),
            ))
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct InputTypeDefinitionNode {
    pub description: Description,
    pub name: NameNode,
    pub fields: Vec<InputValueDefinitionNode>,
}

impl InputTypeDefinitionNode {
    pub fn new(name_tok: Token, description: Description) -> ParseResult<InputTypeDefinitionNode> {
        Ok(InputTypeDefinitionNode {
            name: NameNode::new(name_tok)?,
            description,
            fields: Vec::new(),
        })
    }

    pub fn with_fields(&mut self, fields: Vec<InputValueDefinitionNode>) -> &mut Self {
        self.fields = fields;
        self
    }
}

#[derive(Debug, PartialEq)]
pub struct InterfaceTypeDefinitionNode {
    pub description: Description,
    pub name: NameNode,
    pub directives: Option<Directives>,
    pub fields: Vec<FieldDefinitionNode>,
}

impl InterfaceTypeDefinitionNode {
    pub fn new(tok: Token, description: Description) -> ParseResult<InterfaceTypeDefinitionNode> {
        Ok(InterfaceTypeDefinitionNode {
            name: NameNode::new(tok)?,
            description,
            directives: None,
            fields: Vec::new(),
        })
    }
    pub fn with_fields(&mut self, fields: Vec<FieldDefinitionNode>) -> &mut Self {
        self.fields = fields;
        self
    }

    pub fn with_directives(&mut self, directives: Option<Directives>) -> &mut Self {
        self.directives = directives;
        self
    }
}

#[derive(Debug, PartialEq)]
pub struct EnumTypeDefinitionNode {
    pub description: Description,
    pub name: NameNode,
    pub directives: Option<Directives>,
    pub values: Vec<EnumValueDefinitionNode>,
}

impl EnumTypeDefinitionNode {
    pub fn new(
        tok: Token,
        description: Description,
        directives: Option<Directives>,
        values: Vec<EnumValueDefinitionNode>,
    ) -> ParseResult<EnumTypeDefinitionNode> {
        Ok(EnumTypeDefinitionNode {
            description,
            name: NameNode::new(tok)?,
            directives,
            values,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct UnionTypeDefinitionNode {
    pub description: Description,
    pub name: NameNode,
    pub directives: Option<Directives>,
    pub types: Vec<NamedTypeNode>,
}

impl UnionTypeDefinitionNode {
    pub fn new(
        tok: Token,
        description: Description,
        directives: Option<Directives>,
        types: Vec<NamedTypeNode>,
    ) -> ParseResult<UnionTypeDefinitionNode> {
        Ok(UnionTypeDefinitionNode {
            description,
            name: NameNode::new(tok)?,
            directives,
            types,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum TypeDefinitionNode {
    Scalar(ScalarTypeDefinitionNode),
    Object(ObjectTypeDefinitionNode),
    Interface(InterfaceTypeDefinitionNode),
    Union(UnionTypeDefinitionNode),
    Enum(EnumTypeDefinitionNode),
    Input(InputTypeDefinitionNode),
}

#[derive(Debug, PartialEq)]
pub enum TypeSystemDefinitionNode {
    Schema(SchemaDefinitionNode),
    Type(TypeDefinitionNode),
    // Directive(DirectiveDefinitionNode),
}

#[derive(Debug, PartialEq)]
pub enum TypeSystemExtensionNode {
    Object(ObjectTypeExtensionNode),
}

#[derive(Debug, PartialEq)]
pub enum DefinitionNode {
    // Executable(ExecutableDefinitionNode),
    TypeSystem(TypeSystemDefinitionNode),
    Extension(TypeSystemExtensionNode),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_extenstion_validates() {
        let mut extension = ObjectTypeExtensionNode {
            name: NameNode::from("SomeObj"),
            description: None,
            directives: None,
            interfaces: None,
            fields: None,
        };
        assert!(extension.validate().is_err());
        extension.with_directives(Some(vec![DirectiveNode {
            arguments: None,
            name: NameNode::from("someDirective"),
        }]));
        assert!(extension.validate().is_ok());
        extension.with_directives(None);
        extension.with_interfaces(Some(vec![NamedTypeNode::from("SomeInterface")]));
        assert!(extension.validate().is_ok());

        extension.with_interfaces(None);
        extension.with_fields(vec![FieldDefinitionNode {
            arguments: None,
            description: None,
            name: NameNode::from("someField"),
            field_type: TypeNode::Named(NamedTypeNode::from("String")),
        }]);
        assert!(extension.validate().is_ok());
    }

    #[test]
    fn object_extension_validates_against_original() {
        let extension = ObjectTypeExtensionNode {
            name: NameNode::from("Obj"),
            description: None,
            directives: Some(vec![DirectiveNode {
                name: NameNode::from("depricated"),
                arguments: None,
            }]),
            interfaces: Some(vec![NamedTypeNode::from("Timestamped")]),
            fields: Some(vec![FieldDefinitionNode {
                name: NameNode::from("someField"),
                description: None,
                arguments: None,
                field_type: TypeNode::Named(NamedTypeNode::from("String")),
            }]),
        };

        println!("Validating against None");
        assert!(extension.validate_extension(None).is_err());

        let mut object = ObjectTypeDefinitionNode {
            name: NameNode::from("Obj"),
            description: None,
            directives: None,
            interfaces: None,
            fields: vec![FieldDefinitionNode {
                name: NameNode::from("initial"),
                description: None,
                arguments: None,
                field_type: TypeNode::Named(NamedTypeNode::from("Int")),
            }],
        };
        println!("Validating against object with NO overlap");
        assert!(extension.validate_extension(Some(&object)).is_ok());

        object.with_fields(vec![FieldDefinitionNode {
            name: NameNode::from("someField"),
            description: None,
            arguments: None,
            field_type: TypeNode::Named(NamedTypeNode::from("String")),
        }]);
        let res = extension.validate_extension(Some(&object));
        assert!(res.is_err());
        assert!(res.unwrap_err().message.contains("someField"));
    }
}
