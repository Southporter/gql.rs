//! A parsed GraphQL [`Document`].
//!
//! [`Document`]: ../struct.Document.html
use crate::nodes::DefinitionNode;

/// The Document is the root of a GraphQL schema and/or query. It contains a list of GraphQL
/// definitions. These can be anything from types, enums, unions, etc. to a query.
///
/// This struct will also provide validation methods and other ways to manipulate the GraphQL
/// syntax tree.
#[derive(Debug, PartialEq)]
pub struct Document {
    /// A list of GraphQL definitions
    pub definitions: Vec<DefinitionNode>,
}

impl Document {
    /// Create a new document with the provided definitions
    pub fn new(definitions: Vec<DefinitionNode>) -> Document {
        Document { definitions }
    }
}

use std::fmt;
impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Document<{} definitions>", self.definitions.len())
    }
}

use crate::nodes::*;
use std::default::Default;
impl Default for Document {
    fn default() -> Self {
        Self {
            definitions: vec![
                DefinitionNode::TypeSystem(TypeSystemDefinitionNode::Type(
                    TypeDefinitionNode::Scalar(ScalarTypeDefinitionNode {
                        name: NameNode::from("Int"),
                        description: Some(StringValueNode::from(
                            "A signed, 32-bit, non-fractional number",
                            false,
                        )),
                        directives: None,
                    }),
                )),
                DefinitionNode::TypeSystem(TypeSystemDefinitionNode::Type(
                    TypeDefinitionNode::Scalar(ScalarTypeDefinitionNode {
                        name: NameNode::from("Short"),
                        description: Some(StringValueNode::from(
                            "A signed, 8-bit, non-fractional number",
                            false,
                        )),
                        directives: None,
                    }),
                )),
                DefinitionNode::TypeSystem(TypeSystemDefinitionNode::Type(
                    TypeDefinitionNode::Scalar(ScalarTypeDefinitionNode {
                        name: NameNode::from("Long"),
                        description: Some(StringValueNode::from(
                            "A signed, 64-bit, non-fractional number",
                            false,
                        )),
                        directives: None,
                    }),
                )),
                DefinitionNode::TypeSystem(TypeSystemDefinitionNode::Type(
                    TypeDefinitionNode::Scalar(ScalarTypeDefinitionNode {
                        name: NameNode::from("TinyString"),
                        description: Some(StringValueNode::from(
                            "A string value with a maximum of 255 bytes",
                            false,
                        )),
                        directives: None,
                    }),
                )),
                DefinitionNode::TypeSystem(TypeSystemDefinitionNode::Type(
                    TypeDefinitionNode::Scalar(ScalarTypeDefinitionNode {
                        name: NameNode::from("String"),
                        description: Some(StringValueNode::from(
                            "A string value with a maximum of 65,535 bytes",
                            false,
                        )),
                        directives: None,
                    }),
                )),
                DefinitionNode::TypeSystem(TypeSystemDefinitionNode::Type(
                    TypeDefinitionNode::Scalar(ScalarTypeDefinitionNode {
                        name: NameNode::from("MediumString"),
                        description: Some(StringValueNode::from(
                            "A string value with a maximum of 16,777,215 bytes",
                            false,
                        )),
                        directives: None,
                    }),
                )),
                DefinitionNode::TypeSystem(TypeSystemDefinitionNode::Type(
                    TypeDefinitionNode::Scalar(ScalarTypeDefinitionNode {
                        name: NameNode::from("LongString"),
                        description: Some(StringValueNode::from(
                            "A string value with a maximum of 4,294,967,295 bytes (4GB)",
                            false,
                        )),
                        directives: None,
                    }),
                )),
            ],
        }
    }
}
