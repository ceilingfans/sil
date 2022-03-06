mod token;

pub use token::*;

use logos::Logos;

pub fn lex(source: String) -> Option<Vec<Token>> {
    let mut lex = TokenKind::lexer(&source);
    let mut tokens: Vec<Token> = vec![];
    while let Some(tok) = lex.next() {
        if tok == TokenKind::ERROR {
            return None;
        }

        let token = Token::new(lex.slice().to_string(), tok);
        tokens.push(token);
    }

    Some(tokens)
}