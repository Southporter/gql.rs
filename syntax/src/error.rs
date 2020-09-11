//! Errors representing a synactical or logical issue in the GraphQL Document
//!
//! ParseError represents a syntactical issue. This includes things like
//! an empty type, an unknown token, or an empty body are included in a
//! ParseError
//!
//! ValidationError is a logical issue with the Document. This includes issues
//! like an extension including duplicate field, redefining a type, etc.
//!
//! Use these types to catch and appropriately inform the user of parsing issues:
//!
//! # Example
//!
//! A [`ParseError`] can be used as follows:
//!
//! [`ParseError`]: ../enum.ParseError.html
//!
//! ```
//! use syntax;
//! use syntax::error::ParseError;
//!
//! let result = syntax::parse("type Empty {}");
//! println!("Result: {:?}", result);
//! assert!(result.is_err());
//! match result.unwrap_err() {
//!     ParseError::ObjectEmpty => assert!(true),
//!     _ => assert!(false),
//! }
//! ```
//!

use crate::lexer::LexError;

/// A collection of syntactically bad states that a parser can get into.
#[derive(Debug, PartialEq)]
pub enum ParseError {
    /// Used when the parser is in a bad state and the issue cannot be concretly
    /// determined from the context.
    BadValue,

    /// The GraphQL String was empty
    DocumentEmpty,

    /// The argument string of a field, type, etc. was empty
    ArgumentEmpty,

    /// A type was defined but had no fields
    ObjectEmpty,

    /// Encountered the end of the GraphQL string unexpectedly
    EOF,

    /// There was an error lexing the next token
    LexError(LexError),

    /// The last token lexed was not the token that is defined
    /// in the GraphQL spec
    UnexpectedToken {
        /// The token that was expected
        expected: String,
        /// The token received from the string
        received: String,
    },

    /// Expected a keyword, but the token lexed was not that keyword.
    /// Typically the token was of the correct type, but the content
    /// was unexpected.
    UnexpectedKeyword {
        /// The keyword that is expected
        expected: String,
        /// The keyword that was recieved
        received: String,
    },

    /// Used to convey to the developer or user that this functionality
    /// is planned, but not currently implemented.
    NotImplemented,
}

/// The return type of `parse`.
pub type ParseResult<T> = Result<T, ParseError>;

/// [`ValidationError`]: ../struct.ValidationError.html
///
/// A representation of a logical issue in the GraphQL Document.
///
/// # Example
///
/// ```
/// use syntax::parse;
/// use syntax::document::Document;
/// ```
#[derive(Debug)]
pub struct ValidationError {
    /// A description of the logical error encountered while validating
    /// the GraphQL Document.
    pub message: String,
}

impl ValidationError {
    /// Returns a ValidationError with a message of the issue.
    ///
    /// Used internally. Not intended for external use.
    pub fn new(message: &str) -> ValidationError {
        ValidationError {
            message: String::from(message),
        }
    }
}
