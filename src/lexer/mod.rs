pub mod token;

use token::Token;

pub struct Lexer {
    pub tokens: Vec<Token>,
    source: String,
    ignored: Vec<char>,
    keywords: Vec<String>,
    current_token: Token,
    current_index: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source,
            ..Default::default()
        }
    }
}

impl Default for Lexer {
    fn default() -> Self {
        let kw = vec!["let"];
        let kw_string: Vec<String> = kw.iter().map(ToString::to_string).collect();

        Lexer {
            tokens: vec![],
            source: String::new(),
            ignored: vec![' ', '\t', '\r'],
            keywords: kw_string,
            current_token: Default::default(),
            current_index: 0,
        }
    }
}