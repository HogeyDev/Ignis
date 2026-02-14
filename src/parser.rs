use std::collections::{HashMap, HashSet};

use crate::lexer::Token;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Function,
    NoPrefix,
    TypeDef,
    Import,
    Return,
    Struct,
    While,
    Else,
    Enum,
    For,
    Let,
    If,

    Ident,
    String,
    Integer,

    PrimType,
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

    Equals,
    LogOr,
    LogAnd,
    LogNot,
    EqualTo,
    NotEqualTo,
    LessThan,
    MoreThan,
    LessThanEq,
    MoreThanEq,
    BitOr,
    BitXor,
    BitNeg,
    LShift,
    RShift,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Ampersand,
    At,

    Dot,
    Arrow,
}

impl Token {
    pub fn get_kind(&self) -> TokenKind {
        match self {
            Self::Function => TokenKind::Function,
            Self::NoPrefix => TokenKind::NoPrefix,
            Self::TypeDef => TokenKind::TypeDef,
            Self::Import => TokenKind::Import,
            Self::Return => TokenKind::Return,
            Self::Struct => TokenKind::Struct,
            Self::While => TokenKind::While,
            Self::Else => TokenKind::Else,
            Self::Enum => TokenKind::Enum,
            Self::For => TokenKind::For,
            Self::Let => TokenKind::Let,
            Self::If => TokenKind::If,

            Self::Ident(_) => TokenKind::Ident,
            Self::String(_) => TokenKind::String,
            Self::Integer(_) => TokenKind::Integer,

            Self::PrimType(_) => TokenKind::PrimType,
            Self::FuncType => TokenKind::FuncType,

            Self::LBrace => TokenKind::LBrace,
            Self::RBrace => TokenKind::RBrace,
            Self::LParen => TokenKind::LParen,
            Self::RParen => TokenKind::RParen,
            Self::LBracket => TokenKind::LBracket,
            Self::RBracket => TokenKind::RBracket,
            
            Self::Colon => TokenKind::Colon,
            Self::Semi => TokenKind::Semi,
            Self::Comma => TokenKind::Comma,

            Self::Equals => TokenKind::Equals,
            Self::LogOr => TokenKind::LogOr,
            Self::LogAnd => TokenKind::LogAnd,
            Self::LogNot => TokenKind::LogNot,
            Self::EqualTo => TokenKind::EqualTo,
            Self::NotEqualTo => TokenKind::NotEqualTo,
            Self::LessThan => TokenKind::LessThan,
            Self::MoreThan => TokenKind::MoreThan,
            Self::LessThanEq => TokenKind::LessThanEq,
            Self::MoreThanEq => TokenKind::MoreThanEq,
            Self::BitOr => TokenKind::BitOr,
            Self::BitXor => TokenKind::BitXor,
            Self::BitNeg => TokenKind::BitNeg,
            Self::LShift => TokenKind::LShift,
            Self::RShift => TokenKind::RShift,
            Self::Plus => TokenKind::Plus,
            Self::Minus => TokenKind::Minus,
            Self::Star => TokenKind::Star,
            Self::Slash => TokenKind::Slash,
            Self::Percent => TokenKind::Percent,
            Self::Ampersand => TokenKind::Ampersand,
            Self::At => TokenKind::At,

            Self::Dot => TokenKind::Dot,
            Self::Arrow => TokenKind::Arrow,
        }
    }
}

#[derive(Debug)]
pub enum Type {
    Prim(String),
    Array {
        size: Expression,
        kind: Box<Type>,
    },
    Pointer {
        kind: Box<Type>
    },
    Ident(String),
    Function {
        ret: Box<Type>,
        params: Vec<Type>,
    },
}

type Block = Vec<Statement>;

#[derive(Debug)]
pub enum Declaration {
    Struct {
        name: String,
        fields: HashMap<String, Type>
    },
    Enum {
        name: String,
        modifiers: HashSet<String>,
        variants: Vec<String>,
    },
    Function {
        name: String,
        kind: Type,
        body: Block,
    },
    TypeDef {
        name: String,
        kind: Type,
    },
}

#[derive(Debug)]
pub enum Statement {
    Import(String), // path (relative?)
    Return(Expression),
    VarDecl {
        name: String,
        kind: Type,
        value: Option<Expression>,
    },
    If {
        condition: Expression,
        body: Block,
        alt: Option<Box<Statement>>,
    },
    While {
        condition: Expression,
        body: Block,
    },
    Expression(Expression),
}

#[derive(Debug)]
pub enum Primary {
    Integer(i128),
    String(String),
    Identifier(String),
    Group(Box<Expression>),
}

#[derive(Debug)]
pub enum Expression {
    Unary {
        child: Box<Expression>,
        op: Token,
    },
    Binary {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
        op: Token,
    },
    FunctionCall {
        name: Primary,
        args: Vec<Expression>,
    },
    ArrayAccess {
        lhs: Primary,
        index: Box<Expression>,
    },
    MemberAccess {
        lhs: Primary,
        member: String,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    i: usize,
}

impl Parser {
    pub fn from(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            i: 0usize,
        }
    }

    fn previous(&mut self) -> Option<Token> {
        if self.i > 0 && self.i < self.tokens.len() {
            return Some(self.tokens[self.i - 1].clone());
        }
        None
    }
    fn advance(&mut self) -> Token {
        self.i += 1;
        self.previous().unwrap()
    }
    fn current(&self) -> &Token {
        &self.tokens[self.i]
    }
    fn consume(&mut self, kind: TokenKind) -> Token {
        if self.current().get_kind() == kind {
            return self.advance();
        }
        panic!("Token types don't match: {kind:?}, {:?}", self.current());
    }

    pub fn run(&mut self) -> Vec<Declaration> {
        let mut program: Vec<Declaration> = Vec::new();

        while self.i < self.tokens.len() {
            program.push(self.declaration());
        }

        program
    }

    fn declaration(&mut self) -> Declaration {
        match self.current() {
            Token::Struct => self.struct_decl(),
            Token::Enum => self.enum_decl(),
            Token::Function => self.function_decl(),
            Token::TypeDef => self.typedef_decl(),
            _ => unreachable!(),
        }
    }
    fn struct_decl(&mut self) -> Declaration {
        self.consume(TokenKind::Struct);
        let Token::Ident(name) = self.consume(TokenKind::Ident) else { unreachable!(); };
        let mut fields: HashMap<String, Type> = HashMap::new();

        self.consume(TokenKind::LBrace);
        while let Token::Ident(field) = self.current().to_owned() {
            self.i += 1;
            let kind = self.parse_type();
            fields.insert(field.to_owned(), kind);
        }
        self.consume(TokenKind::RBrace);

        Declaration::Struct { name, fields }
    }
    fn enum_decl(&mut self) -> Declaration {
        self.consume(TokenKind::Enum);
        let Token::Ident(name) = self.consume(TokenKind::Ident) else { unreachable!(); };
        let mut variants: Vec<String> = Vec::new();

        let mut modifiers: HashSet<String> = HashSet::new();
        if TokenKind::LBracket == self.current().get_kind() {
            self.i += 1;
            while let Token::Ident(name) = self.consume(TokenKind::Ident) {
                modifiers.insert(name);
            }
        }
        self.consume(TokenKind::RBracket);

        self.consume(TokenKind::LBrace);
        while let Token::Ident(var) = self.current().to_owned() {
            self.i += 1;
            variants.push(var.to_owned());
        }
        self.consume(TokenKind::RBrace);

        Declaration::Enum { name, modifiers, variants }
    }
    fn function_decl(&mut self) -> Declaration {
        self.consume(TokenKind::Function);
    }
    fn typedef_decl(&mut self) -> Declaration {}
    fn parse_type(&mut self) -> Type {}
}
