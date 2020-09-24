//! Errors representing a synactical or logical issue in the GraphQL Document
//!
//! LexError represents a lexical issue. This usually boils down to unrecognized characters, issues
//! converting into other types or unmatched paired characters.
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
//! use syntax::token::Location;
//!
//! let result = syntax::parse("type Empty {}");
//! println!("Result: {:?}", result);
//! assert!(result.is_err());
//! match result.unwrap_err() {
//!     ParseError::ObjectEmpty(location) => {
//!         assert!(true);
//!         assert_eq!(location, Location::new(5, 1, 6));
//!     }
//!     _ => assert!(false),
//! }
//! ```
//!

use crate::token::Location;
use std::fmt;

fn format_location_message(message: &'static str, location: &Location) -> String {
    format!(
        "{} line {}, column {}",
        message, location.line, location.column
    )
}

fn format_expected_value_message(
    message: &'static str,
    location: &Location,
    expected: &str,
) -> String {
    format!(
        "{}: Expected one of {}",
        format_location_message(message, location),
        expected
    )
}

fn format_expected_received_message(
    message: &'static str,
    location: &Location,
    expected: &str,
    received: &str,
) -> String {
    format!(
        "{}: Expected \"{}\", but found \"{}\"",
        format_location_message(message, location),
        expected,
        received
    )
}

/// Represents a symantic issue in the GraphQL string.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LexError {
    /// The Lexer encountered a `"` that was not paired
    UnmatchedQuote(Location),
    /// The next character is not a valid GraphQL symbol
    UnknownCharacter(Location),
    /// The following character is valid but was not expected in that order
    UnexpectedCharacter(Location),
    /// An issue occured while trying to turn the string value into some other type
    UnableToConvert(Location, &'static str),
    /// The end of the file was encountered unexpectedly
    EOF,
}

const EOF_MESSAGE: &'static str = "Parse Error: Encountered End of File unexpectedly";
const UNMATCHED_QUOTE_MESSAGE: &'static str = "Parse Error: Unmatched quote found on";
const UNKNOWN_CHARACTER_MESSAGE: &'static str = "Parse Error: Unknown character found on";
const UNEXPECTED_CHARACTER_MESSAGE: &'static str = "Parse Error: Unexpected character found on";
const UNABLE_TO_CONVERT_MESSAGE: &'static str = "Parse Error: Unable to convert value at";

const UNKNOWN_ERROR_MESSAGE: &'static str = "Unknown error while parsing";

impl LexError {
    fn get_message(&self) -> String {
        match self {
            LexError::EOF => String::from(EOF_MESSAGE),
            LexError::UnmatchedQuote(location) => {
                format_location_message(UNMATCHED_QUOTE_MESSAGE, location)
            }
            LexError::UnknownCharacter(location) => {
                format_location_message(UNKNOWN_CHARACTER_MESSAGE, location)
            }
            LexError::UnexpectedCharacter(location) => {
                format_location_message(UNEXPECTED_CHARACTER_MESSAGE, location)
            }
            LexError::UnableToConvert(location, expected) => {
                format_expected_value_message(UNABLE_TO_CONVERT_MESSAGE, location, expected)
            }
        }
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_message())
    }
}

/// A collection of syntactically bad states that a parser can get into.
#[derive(Debug, PartialEq)]
pub enum ParseError {
    /// Used when the parser is in a bad state and the issue cannot be concretly
    /// determined from the context.
    BadValue,

    /// The GraphQL String was empty
    DocumentEmpty,

    /// The argument string of a field, type, etc. was empty
    ArgumentEmpty(Location),

    /// A type was defined but had no fields
    ObjectEmpty(Location),

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
        /// The [`location`] of the unexpected token
        /// [`location`]: ../token/struct.Location.html
        location: Location,
    },

    /// Expected a keyword, but the token lexed was not that keyword.
    /// Typically the token was of the correct type, but the content
    /// was unexpected.
    UnexpectedKeyword {
        /// The keyword that is expected
        expected: String,
        /// The keyword that was recieved
        received: String,
        /// The [`location`] of the unexpected token
        /// [`location`]: ../token/struct.Location.html
        location: Location,
    },

    /// Used to convey to the developer or user that this functionality
    /// is planned, but not currently implemented.
    NotImplemented,
}

const NOT_IMPLEMENTED_MESSAGE: &'static str =
    "Parse Error: One or more operations/types specified is not implemented";
const BAD_VALUE_MESSAGE: &'static str =
    "Parse Error: Bad value received. Please check input and try again.";
const DOCUMENT_EMPTY_MESSAGE: &'static str =
    "Parse Error: Document is empty. Cannot parse an empty value";
const ARGUMENT_EMPTY_MESSAGE: &'static str = "Parse Error: Argument empty on";
const OBJECT_EMPTY_MESSAGE: &'static str = "Parse Error: Object empty on";

const EXPECTED_TOKEN_MESSAGE: &'static str = "Parse Error: Unexpected token on";
const EXPECTED_KEYWORD_MESSAGE: &'static str = "Parse Error: Unexpected keyword on";

impl ParseError {
    fn get_message(&self) -> String {
        match self {
            ParseError::NotImplemented => String::from(NOT_IMPLEMENTED_MESSAGE),
            ParseError::BadValue => String::from(BAD_VALUE_MESSAGE),
            ParseError::DocumentEmpty => String::from(DOCUMENT_EMPTY_MESSAGE),
            ParseError::ArgumentEmpty(location) => {
                format_location_message(ARGUMENT_EMPTY_MESSAGE, location)
            }
            ParseError::ObjectEmpty(location) => {
                format_location_message(OBJECT_EMPTY_MESSAGE, location)
            }
            ParseError::EOF => String::from(EOF_MESSAGE),
            ParseError::LexError(lex_error) => lex_error.to_string(),
            ParseError::UnexpectedToken {
                expected,
                received,
                location,
            } => format_expected_received_message(
                EXPECTED_TOKEN_MESSAGE,
                location,
                expected,
                received,
            ),
            ParseError::UnexpectedKeyword {
                expected,
                received,
                location,
            } => format_expected_received_message(
                EXPECTED_KEYWORD_MESSAGE,
                location,
                expected,
                received,
            ),
            _ => String::from(UNKNOWN_ERROR_MESSAGE),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_message())
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    #[test]
    fn creates_message_for_not_implemented() {
        let error = ParseError::NotImplemented;
        assert_eq!(error.to_string(), NOT_IMPLEMENTED_MESSAGE);
    }

    #[test]
    fn creates_message_for_a_bad_value() {
        let error = ParseError::BadValue;
        assert_eq!(error.to_string(), BAD_VALUE_MESSAGE);
    }

    #[test]
    fn creates_message_for_an_empty_document() {
        let error = ParseError::DocumentEmpty;
        assert_eq!(error.to_string(), DOCUMENT_EMPTY_MESSAGE);
    }

    #[test]
    fn creates_message_for_an_empty_argument() {
        let error = ParseError::ArgumentEmpty(Location::new(42, 4, 2));
        assert_eq!(
            error.to_string(),
            format!("{} line {}, column {}", ARGUMENT_EMPTY_MESSAGE, 4, 2)
        );
    }

    #[test]
    fn creates_message_for_an_empty_object() {
        let error = ParseError::ObjectEmpty(Location::new(42, 4, 2));
        assert_eq!(
            error.to_string(),
            format!("{} line {}, column {}", OBJECT_EMPTY_MESSAGE, 4, 2)
        );
    }

    #[test]
    fn creates_message_for_end_of_file() {
        let error = ParseError::EOF;
        assert_eq!(error.to_string(), EOF_MESSAGE);
    }

    #[test]
    fn creates_lex_error_message() {
        let lex_error =
            LexError::UnableToConvert(Location::new(42, 4, 2), "Light Side or Dark Side");
        let error = ParseError::LexError(lex_error);
        assert_eq!(error.to_string(), lex_error.to_string());
    }

    #[test]
    fn creates_unexpected_token_message() {
        let location = Location::new(42, 4, 2);
        let expected = Token::Name(Location::new(42, 4, 2), "val");
        let received = Token::Str(location, "Content of value");
        let error = ParseError::UnexpectedToken {
            expected: expected.to_string(),
            received: received.to_string(),
            location: received.location(),
        };
        assert_eq!(
            error.to_string(),
            format!(
                "{} line {}, column {}: Expected \"{}\", but found \"{}\"",
                EXPECTED_TOKEN_MESSAGE,
                location.line,
                location.column,
                expected.to_string(),
                received.to_string()
            )
        );
    }

    #[test]
    fn creates_unexpected_keyword_message() {
        let location = Location::new(42, 4, 2);
        let received = Token::Name(location, "extends");
        let error = ParseError::UnexpectedKeyword {
            expected: String::from("implements"),
            received: String::from("extends"),
            location: received.location(),
        };
        assert_eq!(
            error.to_string(),
            format!(
                "{} line {}, column {}: Expected \"{}\", but found \"{}\"",
                EXPECTED_KEYWORD_MESSAGE, location.line, location.column, "implements", "extends"
            )
        );
    }
}
