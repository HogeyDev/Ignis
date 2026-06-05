use std::iter::Peekable;
use std::str::Chars;
use crate::token::{Token, TokenKind};
use crate::span::Span;

#[derive(Debug, Clone, Copy)]
pub enum TokenError {
    UnknownCharacter(char),
    UnexpectedEof,
}

impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}
impl std::error::Error for TokenError {}

pub struct Lexer<'a> {
    source: &'a str,
    chars: Peekable<Chars<'a>>,

    index: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            index: 0,
            source: content,
            chars: content.chars().peekable(),
        }
    }

    fn curr(&mut self) -> Result<&char, TokenError> {
        match self.chars.peek() {
            Some(res) => Ok(res),
            None => Err(TokenError::UnexpectedEof),
        }
    }
    fn adv(&mut self) -> Result<char, TokenError> {
        self.index += 1;
        match self.chars.next() {
            Some(res) => Ok(res),
            None => Err(TokenError::UnexpectedEof),
        }
    }
    fn emit(&self, tk: TokenKind<'a>, s: usize) -> Token<'a> {
        Token {
            value: tk,
            pos: Span::new(self.index-s, self.index),
        }
    }
    fn emit_adv(&mut self, tk: TokenKind<'a>, s: usize) -> Result<Token<'a>, TokenError> {
        let start = self.index;
        for _ in 0..s { self.adv()?; }
        Ok(Token {
            value: tk,
            pos: Span::new(start, self.index),
        })
    }

    pub fn tokens(&mut self) -> Result<Vec<Token<'a>>, TokenError> {
        let mut tokens = Vec::new();

        while let token = self.next_token()? && !matches!(token, Token { value: TokenKind::Eof, .. }) {
            eprintln!("{token:?}");
            tokens.push(token);
        }

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token<'a>, TokenError> {
        loop {
            self.skip_whitespace()?;

            let curr = match self.chars.peek() {
                Some(c) => *c,
                None => return Ok(self.emit(TokenKind::Eof, 0)),
            };

            match curr {
                '/' => {
                    self.adv()?;
                    match self.chars.peek() {
                        Some('/') => {
                            while let Some(c) = self.chars.peek() {
                                if *c == '\n' { break; }
                                self.adv()?;
                            }
                        }
                        _ => {
                            return Ok(Token {
                                value: TokenKind::Slash,
                                pos: Span::new(self.index - 1, self.index),
                            });
                        }
                    }
                }
                s if s.is_alphabetic() => { return self.identifier(); }
                '\"' => { return self.string(); }
                n if n.is_numeric() => { return self.number(); }
                '\'' => { return self.character(); }
                '{' => { return self.emit_adv(TokenKind::LBrace, 1); }
                '}' => { return self.emit_adv(TokenKind::RBrace, 1); }
                '(' => { return self.emit_adv(TokenKind::LParen, 1); }
                ')' => { return self.emit_adv(TokenKind::RParen, 1); }
                '[' => { return self.emit_adv(TokenKind::LBracket, 1); }
                ']' => { return self.emit_adv(TokenKind::RBracket, 1); }

                ':' => { return self.emit_adv(TokenKind::Colon, 1); }
                ';' => { return self.emit_adv(TokenKind::Semi, 1); }
                ',' => { return self.emit_adv(TokenKind::Comma, 1); }

                '=' => { return self.emit_adv(TokenKind::Eq, 1); }
                '|' => { return self.emit_adv(TokenKind::Pipe, 1); }
                x => { return Err(TokenError::UnknownCharacter(x)); }
            }
        }
    }

    fn identifier(&mut self) -> Result<Token<'a>, TokenError> {
        let start = self.index;

        loop {
            match self.chars.peek() {
                Some(c) if c.is_alphanumeric() || *c == '_' => {
                    self.adv()?;
                }
                _ => break,
            }
        }
        let lit_value = &self.source[start..self.index];
        let value = match lit_value {
            "continue" => TokenKind::Continue,
            "function" => TokenKind::Function,
            "typeDef" => TokenKind::TypeDef,
            "import" => TokenKind::Import,
            "return" => TokenKind::Return,
            "static" => TokenKind::Static,
            "struct" => TokenKind::Struct,
            "break" => TokenKind::Break,
            "while" => TokenKind::While,
            "cast" => TokenKind::Cast,
            "else" => TokenKind::Else,
            "enum" => TokenKind::Enum,
            "spec" => TokenKind::Spec,
            "asm" => TokenKind::Asm,
            "for" => TokenKind::For,
            "let" => TokenKind::Let,
            "if" => TokenKind::If,
            "Func" => TokenKind::FuncType,
            pt @ ( "void" | "char" | "usize" | "isize" | "u64" | "i64" |
                   "u32" | "i32" | "u16" | "i16" | "u8" | "i8" ) => TokenKind::PrimType(pt),
            x => TokenKind::Ident(x),
        };
        Ok(Token { value, pos: Span::new(start, self.index) })
    }

    fn skip_whitespace(&mut self) -> Result<(), TokenError> {
        while let Some(c) = self.chars.peek() {
            if !c.is_whitespace() { break; }
            self.adv()?;
        }
        Ok(())
    }
    fn skip_comments(&mut self) -> Result<(), TokenError> {
        Ok(())
    }

    fn string(&mut self) -> Result<Token<'a>, TokenError> {
        Ok(Token { value: TokenKind::Eof, pos: Span::new(0,0) })
    }
    fn character(&mut self) -> Result<Token<'a>, TokenError> {
        Ok(Token { value: TokenKind::Eof, pos: Span::new(0,0) })
    }
    fn number(&mut self) -> Result<Token<'a>, TokenError> {
        Ok(Token { value: TokenKind::Eof, pos: Span::new(0,0) })
    }
}
