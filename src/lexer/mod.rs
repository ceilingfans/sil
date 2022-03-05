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

    fn current_char(&self) -> char {
        self.source.as_bytes()[self.current_index] as char
    }

    fn peek_next(&self) -> Option<char> {
        if self.current_index >= self.source.len() - 1 {
            None
        } else {
            Some(self.source.as_bytes()[self.current_index + 1] as char)
        }
    }

    pub fn read_number(&mut self) -> Option<i64> {
        if !self.current_char().is_ascii_digit() {
            return None;
        }
        let mut number_literal = String::new();
        while self.peek_next().is_some() {
            let c = self.current_char();
            if c.is_ascii_digit() {
                number_literal.push(c);
                self.current_index += 1;
            } else {
                break;
            }
        }

        Some(number_literal.parse::<i64>().unwrap())
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