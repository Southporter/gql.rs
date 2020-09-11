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
