use std::iter::Peekable;
use std::str::Char;
use crate::token::{Token, TokenKind};
use crate::span::Span;

pub enum TokenError {
    UnknownCharacter,
}

pub struct Lexer<'a> {
    source: &'a str,
    chars: Peekable<Chars<'a>>,

    start: usize,
    index: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            start: 0,
            index: 0,
            source: content,
            chars: content.
        }
    }

    fn peek(&self, off: isize) -> Option<char> {
        let pos = TryInto::<usize>::try_into(self.index as isize + off);
        match pos {
            Ok(x) => self.chars.peek(pos),
            Err(_) => None,
        }
    }

    fn curr(&self) -> char {
        self.chars.peek(pos).copied()
    }

    pub fn tokens(&mut self) -> Vec<Token<'a>> {
        let mut tokens = Vec::new();

        while let Ok(token) = self.next_token() {
            tokens.push(token);
        }

        tokens
    }

    fn next_token(&mut self) -> Result<Token<'a>, TokenError> {
        self.skip_whitespace();

        match self.peek(0) {
        }
    }
}
