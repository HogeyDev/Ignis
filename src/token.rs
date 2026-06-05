use crate::span::Span;

#[derive(Debug)]
pub enum TokenKind<'a> {
    Continue,
    Function,
    TypeDef,
    Import,
    Return,
    Static,
    Struct,
    Break,
    While,
    Cast,
    Else,
    Enum,
    Spec,
    Asm,
    For,
    Let,
    If,

    Ident(&'a str),
    String(&'a str),
    Integer(&'a str, &'a str),
    Char(char),

    PrimType(&'a str),
    FuncType,

    LBrace,
    RBrace,
    LParen,
    RParen,
    LBracket,
    RBracket,
    
    Colon,
    Semi,
    Comma,

    Eq,
    PipePipe,
    AmpAmp,
    Bang,
    EqEq,
    BangEq,
    LT,
    GT,
    LTE,
    GTE,
    Pipe,
    Caret,
    Tilde,
    LShift,
    RShift,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Amp,
    At,
    Dot,

    Eof,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub value: TokenKind<'a>,
    pub pos: Span,
}
