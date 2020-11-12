//! Tokens represent parts of the string as they are parsed.
//!
//! Some simply point to the position in the input string where they reside. Others will contain a
//! value based on what was parsed. The [`Start`] and [`End`] tokens represent the beginning and
//! end of the string respectively and do not correlate to an character in the input string.
//!
//! [`Start`]: enum.Token.html#variant.Start
//! [`End`]: enum.Token.html#variant.End
//!
//!

/// Contains the information on the location of a lexer error relative to the input string.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Location {
    /// The absolute position in the string. Disregards lines and columns.
    pub absolute_position: usize,
    /// The line number
    pub line: usize,
    /// The column of the line
    pub column: usize,
}

const IGNORED_LOCATION: Location = Location {
    absolute_position: 0,
    line: 0,
    column: 0,
};

impl Location {
    /// Creates a new location based on the provided data
    pub fn new(pos: usize, line: usize, column: usize) -> Self {
        Location {
            absolute_position: pos,
            line,
            column,
        }
    }

    /// Creates a location that can be ignored. Used as a placeholder in parser code.
    pub fn ignored() -> Self {
        IGNORED_LOCATION
    }
}

/// Enumeration of the possible tokens that can be found in a GraphQL String.
#[derive(Debug, Clone)]
pub enum Token<'a> {
    // (position, line, col, value)
    /// Represents the start of the token stream
    Start,
    /// Represents the end of the token stream
    End,
    /// Represents the `!` character and it's position
    Bang(Location),
    /// Represents the `$` character and it's position
    Dollar(Location),
    /// Represents the `&` character and it's position
    Amp(Location),
    /// Represents the `...` series of characters and it's position
    Spread(Location),
    /// Represents the `:` character and it's position
    Colon(Location),
    /// Represents the `=` character and it's position
    Equals(Location),
    /// Represents the `@` character and it's position
    At(Location),
    /// Represents the `(` character and it's position
    OpenParen(Location),
    /// Represents the `)` character and it's position
    CloseParen(Location),
    /// Represents the `[` character and it's position
    OpenSquare(Location),
    /// Represents the `]` character and it's position
    CloseSquare(Location),
    /// Represents the `{` character and it's position
    OpenBrace(Location),
    /// Represents the `}` character and it's position
    CloseBrace(Location),
    /// Represents the `|` character and it's position
    Pipe(Location),
    /// Represents a series of alphanumeric and/or `_` characters. These characters are NOT
    /// surrouned in quotes.
    Name(Location, &'a str),
    /// Represents an parsed integer and it's location in the string
    Int(Location, i64),
    /// Represents an parsed float and it's location in the string
    Float(Location, f64),
    /// Represents a quoted series of characters. These characters can be any valid unicode
    /// character. It will capture all characters within a pair of double quotes
    Str(Location, &'a str),
    /// Represents a triple quoted series of characters. These characters can be any valid unicode
    /// character. It will capture all characters within a pair of triple double quotes (i.e. """A BlockStr is in here""")
    BlockStr(Location, &'a str),
    /// Represents a GraphQL Comment string.
    Comment(Location, &'a str),
}

use std::mem;

impl<'a> Token<'a> {
    /// Helper function to determine if to tokens are of the same
    /// Enum variant.
    ///
    /// ```
    /// use syntax::token::Token;
    ///
    /// assert!(Token::Start.is_same_type(&Token::Start));
    /// assert!(!Token::Start.is_same_type(&Token::End));
    /// ```
    pub fn is_same_type(&self, other: &Token) -> bool {
        return mem::discriminant(self) == mem::discriminant(other);
    }

    /// Extracts the token's location from the enum variant.
    ///
    /// ```
    /// use syntax::token::{Token, Location};
    ///
    /// let location = Location::new(42, 4, 2);
    /// let pipe = Token::Pipe(Location::new(42, 4, 2));
    /// assert_eq!(pipe.location(), location);
    /// assert_eq!(Token::Start.location(), Location {
    ///   absolute_position: 0,
    ///   line: 0,
    ///   column: 0,
    /// });
    /// ```
    pub fn location(&self) -> Location {
        match self {
            Token::Start | Token::End => Location::ignored(),
            Token::Bang(location)
            | Token::Dollar(location)
            | Token::Amp(location)
            | Token::Spread(location)
            | Token::Colon(location)
            | Token::Equals(location)
            | Token::At(location)
            | Token::Pipe(location)
            | Token::OpenParen(location)
            | Token::CloseParen(location)
            | Token::OpenSquare(location)
            | Token::CloseSquare(location)
            | Token::OpenBrace(location)
            | Token::CloseBrace(location)
            | Token::Name(location, _)
            | Token::Int(location, _)
            | Token::Float(location, _)
            | Token::Str(location, _)
            | Token::BlockStr(location, _)
            | Token::Comment(location, _) => *location,
        }
    }
}

use std::fmt;

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Token<{:?}>", self)
    }
}

use std::cmp::{Eq, PartialEq};

impl<'a> PartialEq for Token<'a> {
    fn eq(&self, other: &Token) -> bool {
        match self {
            Token::Name(_, value) => matches!(other, Token::Name(_, value2) if *value2 == *value),
            Token::Str(_, value) => matches!(other, Token::Str(_, value2) if *value2 == *value),
            Token::BlockStr(_, value) => {
                matches!(other, Token::BlockStr(_, value2) if *value2 == *value)
            }
            Token::Int(_, value) => matches!(other, Token::Int(_, value2) if value2 == value),
            Token::Float(_, value) => matches!(other, Token::Float(_, value2) if value2 == value),
            _ => mem::discriminant(self) == mem::discriminant(other),
        }
    }
}

impl<'a> Eq for Token<'a> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_type() {
        assert_eq!(Token::Start, Token::Start);
        assert_ne!(Token::End, Token::Start);

        assert_eq!(
            Token::Bang(Location::new(0, 0, 0)),
            Token::Bang(Location::new(12, 2, 3))
        );
        assert_ne!(
            Token::Amp(Location::new(0, 0, 0)),
            Token::Float(Location::new(0, 0, 0), 0.0)
        );
        assert_ne!(
            Token::Dollar(Location::new(0, 0, 0)),
            Token::OpenBrace(Location::new(0, 1, 1))
        );
        assert_ne!(
            Token::Int(Location::new(0, 0, 0), 0),
            Token::Float(Location::new(0, 0, 0), 0.0)
        );
    }

    #[test]
    fn compare_value() {
        assert_eq!(
            Token::Int(Location::new(0, 0, 0), 10),
            Token::Int(Location::new(12, 3, 14), 10)
        );
        assert_eq!(
            Token::Float(Location::new(0, 0, 0), 3.14),
            Token::Float(Location::new(3, 1, 4), 3.14)
        );
        assert_eq!(
            Token::Name(Location::new(0, 0, 0), "id"),
            Token::Name(Location::new(3, 3, 3), "id")
        );
        assert_eq!(
            Token::Str(Location::new(0, 0, 0), "Comment"),
            Token::Str(Location::new(1, 2, 1), "Comment")
        );
        assert_eq!(
            Token::BlockStr(Location::new(0, 0, 0), "Comment"),
            Token::BlockStr(Location::new(1, 2, 1), "Comment")
        );

        assert_ne!(
            Token::Int(Location::new(0, 0, 0), 10),
            Token::Int(Location::new(12, 3, 14), 11)
        );
        assert_ne!(
            Token::Float(Location::new(0, 0, 0), 3.14),
            Token::Float(Location::new(3, 1, 4), 3.14159)
        );
        assert_ne!(
            Token::Name(Location::new(0, 0, 0), "id"),
            Token::Name(Location::new(3, 3, 3), "val")
        );
        assert_ne!(
            Token::Str(Location::new(0, 0, 0), "Comment"),
            Token::Str(Location::new(1, 2, 1), "Your comment here")
        );
        assert_ne!(
            Token::BlockStr(Location::new(0, 0, 0), "Comment"),
            Token::BlockStr(Location::new(1, 2, 1), "Your comment here")
        );
    }

    #[test]
    fn get_location() {
        let loc = Location::new(42, 3, 4);
        assert_eq!(Token::Start.location(), Location::ignored());
        assert_eq!(Token::End.location(), Location::ignored());
        assert_eq!(Token::Bang(loc).location(), loc);
        assert_eq!(Token::Str(loc, "Some str value").location(), loc);
    }
}
