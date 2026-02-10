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
        Declaration::Struct { name: String::new(), fields: HashMap::new() }
    }
}
