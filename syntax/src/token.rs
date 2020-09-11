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

/// Enumeration of the possible tokens that can be found in a GraphQL String.
#[derive(Debug, Clone)]
pub enum Token<'a> {
    // (position, line, col, value)
    /// Represents the start of the token stream
    Start,
    /// Represents the end of the token stream
    End,
    /// Represents the `!` character and it's position
    Bang(usize, usize, usize),
    /// Represents the `$` character and it's position
    Dollar(usize, usize, usize),
    /// Represents the `&` character and it's position
    Amp(usize, usize, usize),
    /// Represents the `...` series of characters and it's position
    Spread(usize, usize, usize),
    /// Represents the `:` character and it's position
    Colon(usize, usize, usize),
    /// Represents the `=` character and it's position
    Equals(usize, usize, usize),
    /// Represents the `@` character and it's position
    At(usize, usize, usize),
    /// Represents the `(` character and it's position
    OpenParen(usize, usize, usize),
    /// Represents the `)` character and it's position
    CloseParen(usize, usize, usize),
    /// Represents the `[` character and it's position
    OpenSquare(usize, usize, usize),
    /// Represents the `]` character and it's position
    CloseSquare(usize, usize, usize),
    /// Represents the `{` character and it's position
    OpenBrace(usize, usize, usize),
    /// Represents the `}` character and it's position
    CloseBrace(usize, usize, usize),
    /// Represents the `|` character and it's position
    Pipe(usize, usize, usize),
    /// Represents a series of alphanumeric and/or `_` characters. These characters are NOT
    /// surrouned in quotes.
    Name(usize, usize, usize, &'a str),
    /// Represents an parsed integer and it's location in the string
    Int(usize, usize, usize, i64),
    /// Represents an parsed float and it's location in the string
    Float(usize, usize, usize, f64),
    /// Represents a quoted series of characters. These characters can be any valid unicode
    /// character. It will capture all characters within a pair of double quotes
    Str(usize, usize, usize, &'a str),
    /// Represents a triple quoted series of characters. These characters can be any valid unicode
    /// character. It will capture all characters within a pair of triple double quotes (i.e. """A BlockStr is in here""")
    BlockStr(usize, usize, usize, &'a str),
    /// Represents a GraphQL Comment string.
    Comment(usize, usize, usize, &'a str),
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
            Token::Name(_, _, _, value) => {
                matches!(other, Token::Name(_, _, _, value2) if *value2 == *value)
            }
            Token::Str(_, _, _, value) => {
                matches!(other, Token::Str(_, _, _, value2) if *value2 == *value)
            }
            Token::BlockStr(_, _, _, value) => {
                matches!(other, Token::BlockStr(_, _, _, value2) if *value2 == *value)
            }
            Token::Int(_, _, _, value) => {
                matches!(other, Token::Int(_, _, _, value2) if value2 == value)
            }
            Token::Float(_, _, _, value) => {
                matches!(other, Token::Float(_, _, _, value2) if value2 == value)
            }
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
        assert!(Token::Start == Token::Start);
        assert!(Token::End != Token::Start);
        assert!(Token::Bang(0, 0, 0) == Token::Bang(12, 2, 3));
        assert!(Token::Amp(0, 0, 0) != Token::Float(0, 0, 0, 0.0));
        assert!(Token::Dollar(0, 0, 0) != Token::OpenBrace(0, 1, 1));
        assert!(Token::Int(0, 0, 0, 0) != Token::Float(0, 0, 0, 0.0));
    }

    #[test]
    fn compare_value() {
        assert!(Token::Int(0, 0, 0, 10) == Token::Int(12, 3, 14, 10));
        assert!(Token::Float(0, 0, 0, 3.14) == Token::Float(3, 1, 4, 3.14));
        assert!(Token::Name(0, 0, 0, "id") == Token::Name(3, 3, 3, "id"));
        assert!(Token::Str(0, 0, 0, "Comment") == Token::Str(1, 2, 1, "Comment"));
        assert!(Token::BlockStr(0, 0, 0, "Comment") == Token::BlockStr(1, 2, 1, "Comment"));

        assert!(Token::Int(0, 0, 0, 10) != Token::Int(12, 3, 14, 11));
        assert!(Token::Float(0, 0, 0, 3.14) != Token::Float(3, 1, 4, 3.14159));
        assert!(Token::Name(0, 0, 0, "id") != Token::Name(3, 3, 3, "val"));
        assert!(Token::Str(0, 0, 0, "Comment") != Token::Str(1, 2, 1, "Your comment here"));
        assert!(
            Token::BlockStr(0, 0, 0, "Comment") != Token::BlockStr(1, 2, 1, "Your comment here")
        );
    }
}
