use crate::span::Span;

pub enum TokenKind {
}

pub struct Token {
    value: TokenKind,
    pos: Span,
}
