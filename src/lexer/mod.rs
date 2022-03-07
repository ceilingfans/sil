mod token;

pub use token::*;

use logos::Logos;

pub fn lex(source: String) -> Option<Vec<Token>> {
    let mut lex = TokenKind::lexer(&source);
    let mut tokens: Vec<Token> = vec![];
    while let Some(tok) = lex.next() {
        match tok {
            TokenKind::Error => return None,
            TokenKind::String => {
                let slice = lex.slice();
                let literal: String = slice[1..slice.len() - 1].parse().ok()?;
                tokens.push(Token::new(literal, tok));
            },
            _ => {
                tokens.push(Token::new(lex.slice().to_string(), tok));
            }
        }
    }

    Some(tokens)
}