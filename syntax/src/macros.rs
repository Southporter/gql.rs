//! Macros for use with GraphQL syntax

/// gql  will take a document string and turn it into
/// a [`Document`].
///
/// [`Document`]: ../document/struct.Document.html
///
/// # Examples
/// ```
/// use syntax::gql;
///
/// let doc = gql!(r#"{
///   hero {
///     name
///     friends {
///       name
///     }
///   }
/// }"#);
/// assert!(doc.is_ok());
/// assert!(doc.unwrap().definitions.len() == 1);
/// ```
#[macro_export]
macro_rules! gql {
    ($input:expr) => {{
        $crate::parse($input)
    }};
}

#[cfg(test)]
mod tests {
    use crate::document::Document;
    use crate::nodes::*;

    #[test]
    fn it_parses() {
        let doc = gql!(
            r#"{
            user {
                name
            }
        }"#
        );
        assert!(doc.is_ok());
        assert_eq!(
            doc.unwrap(),
            Document {
                definitions: vec![DefinitionNode::Executable(
                    ExecutableDefinitionNode::Operation(OperationTypeNode::Query(
                        QueryDefinitionNode {
                            name: None,
                            variables: None,
                            selections: vec![Selection::Field(FieldNode {
                                name: NameNode::from("user"),
                                alias: None,
                                arguments: None,
                                directives: None,
                                selections: Some(vec![Selection::Field(FieldNode::from("name")),])
                            })]
                        }
                    ))
                )]
            }
        )
    }
}
