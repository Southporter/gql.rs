#[macro_use] extern crate lazy_static;
pub mod token;
mod lexer;
mod extract;

pub struct AST {
    _value: String,
}

impl AST {
    pub fn new(value: String) -> AST {
        AST {
            _value: value,
        }
    }
}

pub fn parse(query: &str) -> AST {
    let _tokens = lexer::tokenize(query);
    AST::new(String::from("Unimplemented"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
