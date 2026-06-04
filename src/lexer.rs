use crate::token::Token;

pub struct Lexer {
    index: usize,
    content: String,
}

impl Lexer {
    pub fn new(content: String) -> Self {
        Self {
            index: 0,
            content,
        }
    }

    pub fn tokens(&mut self) -> Vec<Token> {
        Vec::new()
    }
}
