// use crate::token::Token;

#[derive(Debug, Clone)]
pub enum ExtractErrorKind {
    UnknownCharacter { line: u32, col: u32 }
}

// fn kind_from_char(content: &char, line: u32, col: u32) -> Result<TokenKind,  ExtractErrorKind> {
//     match content {
//         '!' => Ok(TokenKind::Bang),
//         '{' => Ok(TokenKind::OpenBrace),
//         '}' => Ok(TokenKind::CloseBrace),
//         '\n' | ' ' | '\t' => Ok(TokenKind::Whitespace),
//         _ => Err(ExtractErrorKind::UnknownCharacter { line, col })
//     }
// }

// pub fn one<T>(content: &char, position: usize, line: u32, col: u32) -> Result<Token<T>, ExtractErrorKind> {
//     Ok(Token::new(
//         kind_from_char(content, line, col)?,
//         position,
//         position + 1,
//         line,
//         col,
//         content,
//     ))
// }

