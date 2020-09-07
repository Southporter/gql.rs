use crate::error::ValidationError;
use crate::nodes::NodeWithFields;

pub type ValidationResult = Result<(), ValidationError>;

/// A trait used by Document to walk the tree and
/// determine wheter or not the nodes are valid.
/// Defaults to valid.
pub trait ValidNode {
    fn validate(&self) -> ValidationResult {
        Ok(())
    }
}

fn contains_any_element<T: PartialEq>(haystack: &[T], needles: &[T]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}

pub fn validate_extension_fields_against_original<E: NodeWithFields, O: NodeWithFields>(
    extension: &E,
    original: &O,
) -> ValidationResult {
    if contains_any_element(original.get_fields(), extension.get_fields()) {
        let original_fields = original.get_fields();
        let conflicting_names: String = extension
            .get_fields()
            .iter()
            .filter_map(|needle| {
                if original_fields.contains(needle) {
                    Some(needle.name.value.as_str())
                } else {
                    None
                }
            })
            .collect::<Vec<&str>>()
            .join(", ");

        Err(ValidationError::new(
            format!(
                "Invalid Extension: Cannot redefine field(s) {}",
                conflicting_names
            )
            .as_str(),
        ))
    } else {
        Ok(())
    }
}

/// A trait used to determine if a type extension is valid.
/// This requires passing in the original declaration. The original is then
/// used to determine the validity of the extension.
pub trait ValidExtensionNode<T> {
    fn validate_extension(&self, original: Option<&T>) -> ValidationResult {
        if let Some(_) = original {
            Ok(())
        } else {
            Err(ValidationError::new("Invalid Extension: No root element"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_any_element_fn() {
        assert!(!contains_any_element(&[1], &[2]));
        assert!(contains_any_element(&[1, 2], &[2]));
        assert!(contains_any_element(&[1], &[1, 2]));
    }
}
