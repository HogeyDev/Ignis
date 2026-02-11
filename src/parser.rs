use std::collections::HashMap;

use crate::lexer::Token;

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
        modifiers: Vec<String>,
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
    fn consume(&mut self, kind: Token) -> Token {
        if std::mem::discriminant(&self.tokens[self.i]) == std::mem::discriminant(&kind) {
            return self.advance();
        }
        panic!("Token types don't match: {kind:?}, {:?}", self.tokens[self.i]);
    }

    pub fn run(&mut self) -> Vec<Declaration> {
        let mut program: Vec<Declaration> = Vec::new();

        while self.i < self.tokens.len() {
            program.push(self.declaration());
        }

        program
    }

    fn declaration(&mut self) -> Declaration {
        match self.tokens[self.i] {
            Token::Struct => self.struct_decl(),
            Token::Enum => self.enum_decl(),
            Token::Function => self.function_decl(),
            Token::TypeDef => self.typedef_decl(),
            _ => unreachable!(),
        }
    }
    fn struct_decl(&mut self) -> Declaration {
        self.consume(Token::Struct);
        let Token::Ident(name) = self.consume(Token::Ident("".to_owned())) else { unreachable!(); };
        let mut fields: HashMap<String, Type> = HashMap::new();

        self.consume(Token::LBrace);
        while let Token::Ident(field) = self.tokens[self.i].to_owned() {
            self.i += 1;
            let kind = self.kind();
            fields.insert(field.to_owned(), kind);
        }
        self.consume(Token::RBrace);

        Declaration::Struct { name, fields }
    }
    fn enum_decl(&mut self) -> Declaration {
        self.consume(Token::Enum);
        let Token::Ident(name) = self.consume(Token::Ident("".to_owned())) else { unreachable!(); };
        let mut variants: Vec<String> = Vec::new();

        if let Token::LBracket = self.tokens[self.i].to_owned() {

        }

        self.consume(Token::LBrace);
        while let Token::Ident(var) = self.tokens[self.i].to_owned() {
            self.i += 1;
            variants.push(var.to_owned());
        }
        self.consume(Token::RBrace);

        Declaration::Enum { name, variants }
    }
    fn function_decl(&mut self) -> Declaration {}
    fn typedef_decl(&mut self) -> Declaration {}
    fn kind(&mut self) -> Type {}
}
