use crate::error::ParseResult;
use crate::nodes::*;

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
