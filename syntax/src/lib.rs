#[macro_use] extern crate lazy_static;
pub mod token;
pub mod lexer;
pub mod ast;
mod nodes;

use ast::{AST, ParseError};
// use lexer::LexErrorKind;
// use token::Token;

pub fn parse<'a>(query: &'a str) -> Result<AST, ParseError>
{
    let ast = AST::new(query);
    ast
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_handles_lexing_error() {
        println!("parsing error");
        let res = parse("");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), ParseError::DocumentEmpty);
    }

    #[test]
    fn parses_object() {
        println!("parsing an object");

    }
}
