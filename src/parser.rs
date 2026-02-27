use std::collections::{HashMap, HashSet};

use crate::{errors, lexer::{Token, TokenMeta}};

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Function,
    TypeDef,
    Import,
    Return,
    Struct,
    While,
    Cast,
    Else,
    Enum,
    Asm,
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
            Self::TypeDef => TokenKind::TypeDef,
            Self::Import => TokenKind::Import,
            Self::Return => TokenKind::Return,
            Self::Struct => TokenKind::Struct,
            Self::While => TokenKind::While,
            Self::Cast => TokenKind::Cast,
            Self::Else => TokenKind::Else,
            Self::Enum => TokenKind::Enum,
            Self::Asm => TokenKind::Asm,
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
        size: Option<Expression>,
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
        ret: Type,
        params: Vec<(String, Type)>, // (name, type)
        body: Statement,
    },
    TypeDef {
        name: String,
        kind: Type,
    },
    Statement(Statement), // this is pretty much only for global variable declarations and imports
}

#[derive(Debug)]
pub enum Statement {
    Import(String), // path (relative?)
    Return(Option<Expression>),
    VarDecl {
        is_static: bool,
        name: String,
        kind: Option<Type>,
        value: Option<Expression>,
    },
    If {
        condition: Expression,
        body: Box<Statement>,
        alt: Option<Box<Statement>>,
    },
    While {
        condition: Expression,
        body: Box<Statement>,
    },
    Asm(String),
    Block(Vec<Statement>),
    Expression(Expression),
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
        name: Box<Expression>,
        args: Vec<Expression>,
    },
    ArrayAccess {
        lhs: Box<Expression>,
        index: Box<Expression>,
    },
    MemberAccess {
        lhs: Box<Expression>,
        member: String,
    },
    StructInitializer {
        name: String,
        values: HashMap<String, Expression>
    },

    TypeCast {
        to: Box<Type>,
        value: Box<Expression>,
    },

    Integer(i128),
    String(String),
    Identifier(String),
    Group(Box<Expression>),
}

pub struct Parser<'a> {
    filename: &'a str,
    source_lines: Vec<String>,
    tokens: Vec<TokenMeta>,
    i: usize,
}

impl<'a> Parser<'a> {
    pub fn from(filename: &'a str, source_lines: Vec<String>, tokens: Vec<TokenMeta>) -> Self {
        Self {
            filename,
            source_lines,
            tokens,
            i: 0usize,
        }
    }

    fn previous(&mut self) -> Option<TokenMeta> {
        if self.i > 0 && self.i-1 < self.tokens.len() {
            return Some(self.tokens[self.i - 1].clone());
        }
        None
    }
    fn advance(&mut self) -> TokenMeta {
        self.i += 1;
        self.previous().unwrap()
    }
    fn current(&self) -> &TokenMeta {
        &self.tokens[self.i]
    }
    fn consume(&mut self, kind: TokenKind) -> TokenMeta {
        let curr = self.current();
        let ck = curr.get_kind();
        if ck == kind {
            return self.advance();
        }
        errors::parser::expect_mismatch(self.filename, &self.source_lines, curr.pos, kind, ck);
    }

    pub fn run(&mut self) -> Vec<Declaration> {
        let mut program: Vec<Declaration> = Vec::new();

        while self.i < self.tokens.len() {
            program.push(self.declaration());
        }

        program
    }

    fn declaration(&mut self) -> Declaration {
        match &self.current().value {
            Token::Struct => self.struct_decl(),
            Token::Enum => self.enum_decl(),
            Token::Function => self.function_decl(),
            Token::TypeDef => self.typedef_decl(),
            Token::Import => Declaration::Statement(self.import_st()),
            Token::Ident(name) if name == "static" => Declaration::Statement(self.var_decl()),
            _ => unreachable!(),
        }
    }
    fn struct_decl(&mut self) -> Declaration {
        self.consume(TokenKind::Struct);
        let Token::Ident(name) = self.consume(TokenKind::Ident).value else { unreachable!(); };
        let mut fields: HashMap<String, Type> = HashMap::new();

        self.consume(TokenKind::LBrace);
        while let Token::Ident(field) = self.current().value.to_owned() {
            self.i += 1;
            self.consume(TokenKind::Colon);
            let kind = self.parse_type();
            fields.insert(field.to_owned(), kind);
            self.consume(TokenKind::Semi);
        }
        self.consume(TokenKind::RBrace);

        Declaration::Struct { name, fields }
    }
    fn enum_decl(&mut self) -> Declaration {
        self.consume(TokenKind::Enum);
        let Token::Ident(name) = self.consume(TokenKind::Ident).value else { unreachable!(); };
        let mut variants: Vec<String> = Vec::new();

        let mut modifiers: HashSet<String> = HashSet::new();
        if TokenKind::LBracket == self.current().get_kind() {
            self.i += 1;
            while let Token::Ident(modifier) = self.current().value.to_owned() {
                self.i += 1;
                modifiers.insert(modifier);
            }
        }
        self.consume(TokenKind::RBracket);

        self.consume(TokenKind::LBrace);
        while let Token::Ident(variant) = self.current().value.to_owned() {
            self.i += 1;
            variants.push(variant);
            if self.current().get_kind() != TokenKind::Comma { break; }
            else { self.i += 1; }
        }
        self.consume(TokenKind::RBrace);

        Declaration::Enum { name, modifiers, variants }
    }
    fn function_decl(&mut self) -> Declaration {
        self.consume(TokenKind::Function);

        let Token::Ident(name) = self.consume(TokenKind::Ident).value else { unreachable!(); };

        self.consume(TokenKind::LParen);
        let ret = self.parse_type();

        let mut params = Vec::new();
        while self.current().get_kind() == TokenKind::Comma {
            self.consume(TokenKind::Comma);

            let Token::Ident(param_name) = self.consume(TokenKind::Ident).value else { unreachable!(); };
            self.consume(TokenKind::Colon);
            let param_type = self.parse_type();
            
            params.push((param_name, param_type));
        }
        self.consume(TokenKind::RParen);

        Declaration::Function { name, ret, params, body: self.block() }
    }
    fn typedef_decl(&mut self) -> Declaration { todo!(); }
    fn block(&mut self) -> Statement {
        let mut statement = Vec::new();

        self.consume(TokenKind::LBrace);
        while self.current().get_kind() != TokenKind::RBrace {
            statement.push(self.statement());
        }
        self.consume(TokenKind::RBrace);

        Statement::Block(statement)
    }
    
    fn statement(&mut self) -> Statement {
        match self.current().value {
            Token::Import => self.import_st(),
            Token::Return => self.return_st(),
            Token::Let => self.var_decl(),
            Token::If => self.if_st(),
            Token::While => self.while_st(),
            Token::Asm => self.asm_st(),
            Token::For => self.for_st(),
            Token::LBrace => self.block(),
            _ => {
                let expr = self.expression();
                self.consume(TokenKind::Semi);
                Statement::Expression(expr)
            }
        }
    }
    fn import_st(&mut self) -> Statement {
        self.consume(TokenKind::Import);

        let Token::Ident(mut path) = self.consume(TokenKind::Ident).value else { unreachable!(); };
        while self.current().get_kind() == TokenKind::Dot {
            self.consume(TokenKind::Dot);
            let Token::Ident(subdir) = self.consume(TokenKind::Ident).value else { unreachable!(); };

            path.push('/');
            path.push_str(&subdir);
        }

        self.consume(TokenKind::Semi);

        Statement::Import(path)
    }
    fn return_st(&mut self) -> Statement {
        self.consume(TokenKind::Return);
        if self.current().get_kind() == TokenKind::Semi {
            self.advance();
            Statement::Return(None)
        } else {
            let value = self.expression();
            self.consume(TokenKind::Semi);
            Statement::Return(Some(value))
        }
    }
    fn var_decl(&mut self) -> Statement {
        let is_static = match self.advance().value {
            Token::Let => false,
            Token::Ident(name) if name == "static" => true,
            x => panic!("Variable is neither static nor non-static. What are you?\n\t{x:?}"),
        };

        let Token::Ident(name) = self.consume(TokenKind::Ident).value else { unreachable!(); };

        let kind = if self.current().get_kind() == TokenKind::Colon {
            self.consume(TokenKind::Colon);
            let kind = self.parse_type();
            Some(kind)
        } else {
            None
        };

        let value = if self.current().get_kind() == TokenKind::Equals {
            self.consume(TokenKind::Equals);
            let expr = self.expression();
            Some(expr)
        } else {
            None
        };

        self.consume(TokenKind::Semi);

        Statement::VarDecl { is_static, name, kind, value }
    }
    fn if_st(&mut self) -> Statement {
        self.consume(TokenKind::If);

        self.consume(TokenKind::LParen);
        let condition = self.expression();
        self.consume(TokenKind::RParen);

        let body = self.block();

        let alt = if self.current().get_kind() == TokenKind::Else {
            self.consume(TokenKind::Else);
            Some(Box::new(self.statement()))
        } else { None };

        Statement::If { condition, body: Box::new(body), alt }
    }
    fn while_st(&mut self) -> Statement {
        self.consume(TokenKind::While);

        self.consume(TokenKind::LParen);
        let condition = self.expression();
        self.consume(TokenKind::RParen);

        let body = self.block();

        Statement::While { condition, body: Box::new(body) }
    }
    fn asm_st(&mut self) -> Statement {
        self.consume(TokenKind::Asm);
        
        let Token::String(value) = self.consume(TokenKind::String).value else { unreachable!(); };

        self.consume(TokenKind::Semi);
        Statement::Asm(value)
    }
    fn for_st(&mut self) -> Statement {
        self.consume(TokenKind::For);

        self.consume(TokenKind::LParen);
        let init = self.expression();
        self.consume(TokenKind::Semi);

        let condition = self.expression();
        self.consume(TokenKind::Semi);

        let updater = self.expression();
        self.consume(TokenKind::Semi);
        self.consume(TokenKind::RParen);

        let mut body = self.block();
        match body {
            Statement::Block(ref mut xs) => xs.push(Statement::Expression(updater)),
            _ => unreachable!(),
        }

        Statement::Block(vec![
            Statement::Expression(init),
            Statement::While { condition, body: Box::new(body) },
        ])
    }

    fn parse_type(&mut self) -> Type {
        match self.current().value.to_owned() {
            Token::LBracket => {
                self.i += 1;
                let size = if self.current().get_kind() != TokenKind::RBracket {
                    Some(self.expression())
                } else { None };
                self.consume(TokenKind::RBracket);
                let child = self.parse_type();
                Type::Array { size, kind: Box::new(child) }
            }
            Token::At => {
                self.i += 1;
                let child = self.parse_type();
                Type::Pointer { kind: Box::new(child) }
            }
            Token::FuncType => {
                self.i += 1;

                self.consume(TokenKind::LessThan);
                let ret_type = self.parse_type();

                let mut params = Vec::new();
                while self.current().get_kind() == TokenKind::Comma {
                    self.i += 1;
                    params.push(self.parse_type());
                }

                self.consume(TokenKind::MoreThan);
                Type::Function { ret: Box::new(ret_type), params }
            }
            Token::PrimType(kind) => {
                self.i += 1;
                Type::Prim(kind.to_owned())
            }
            Token::Ident(kind) => {
                self.i += 1;
                Type::Ident(kind.to_owned())
            }
            x => { panic!("Not a valid type: `{x:?}`"); }
        }
    }

    fn left_rec(&mut self, symbols: &[TokenKind], child: fn(&mut Parser<'a>) -> Expression) -> Expression {
        let mut lhs = child(self);

        while symbols.contains(&self.current().get_kind()) {
            let op = self.advance();
            let rhs = child(self);
            lhs = Expression::Binary { lhs: Box::new(lhs), rhs: Box::new(rhs), op: op.value }
        }

        lhs
    }
    fn right_rec(&mut self,
        symbols: &[TokenKind],
        parent: fn(&mut Parser<'a>) -> Expression,
        child: fn(&mut Parser<'a>) -> Expression,
    ) -> Expression {
        let lhs = child(self);

        if symbols.contains(&self.current().get_kind()) {
            let op = self.advance();
            let rhs = parent(self);
            Expression::Binary { lhs: Box::new(lhs), rhs: Box::new(rhs), op: op.value }
        } else { lhs }
    }
    fn expression(&mut self) -> Expression { self.assignment() }
    fn assignment(&mut self) -> Expression { self.right_rec(&[TokenKind::Equals], Self::assignment, Self::logical_or) }
    fn logical_or(&mut self) -> Expression { self.left_rec(&[TokenKind::LogOr], Self::logical_and) }
    fn logical_and(&mut self) -> Expression { self.left_rec(&[TokenKind::LogAnd], Self::equality) }
    fn equality(&mut self) -> Expression {
        let mut lhs = self.relation();

        if [TokenKind::EqualTo,
            TokenKind::NotEqualTo,
        ].contains(&self.current().get_kind()) {
            let op = self.advance();
            let rhs = self.relation();
            lhs = Expression::Binary { lhs: Box::new(lhs), rhs: Box::new(rhs), op: op.value };
        }

        lhs
    }
    fn relation(&mut self) -> Expression {
        let mut lhs = self.bitwise_or();

        if [TokenKind::LessThan,
            TokenKind::MoreThan,
            TokenKind::MoreThanEq,
            TokenKind::LessThanEq
        ].contains(&self.current().get_kind()) {
            let op = self.advance();
            let rhs = self.bitwise_or();
            lhs = Expression::Binary { lhs: Box::new(lhs), rhs: Box::new(rhs), op: op.value };
        }

        lhs
    }
    fn bitwise_or(&mut self) -> Expression { self.left_rec(&[TokenKind::BitOr], Self::bitwise_xor) }
    fn bitwise_xor(&mut self) -> Expression { self.left_rec(&[TokenKind::BitXor], Self::bitwise_and) }
    fn bitwise_and(&mut self) -> Expression { self.left_rec(&[TokenKind::Ampersand], Self::shift) }
    fn shift(&mut self) -> Expression { self.left_rec(&[TokenKind::LShift, TokenKind::RShift], Self::addition) }
    fn addition(&mut self) -> Expression { self.left_rec(&[TokenKind::Plus, TokenKind::Minus], Self::multiplication) }
    fn multiplication(&mut self) -> Expression { self.left_rec(&[TokenKind::Star, TokenKind::Slash, TokenKind::Percent], Self::unary)}
    fn unary(&mut self) -> Expression {
        if [TokenKind::LogNot,
            TokenKind::Minus,
            TokenKind::BitNeg
        ].contains(&self.current().get_kind()) {
            let op = self.advance();
            Expression::Unary { child: Box::new(self.unary()), op: op.value }
        } else { self.reference() }
    }
    fn reference(&mut self) -> Expression {
        if self.current().get_kind() == TokenKind::Ampersand {
            let op = self.advance();
            Expression::Unary { child: Box::new(self.access()), op: op.value }
        } else { self.access() }
    }
    fn access(&mut self) -> Expression {
        if self.current().get_kind() == TokenKind::Cast {
            self.advance();
            self.consume(TokenKind::LParen);

            let to = self.parse_type();
            self.consume(TokenKind::Comma);

            let value = self.expression();

            self.consume(TokenKind::RParen);
            Expression::TypeCast { to: Box::new(to), value: Box::new(value) }
        } else if self.current().get_kind() == TokenKind::At {
            let op = self.advance();
            Expression::Unary { child: Box::new(self.primary()), op: op.value }
        } else {
            let mut lhs = self.primary();

            while match self.advance().value {
                Token::LParen => {
                    let mut args = Vec::new();
                    while self.current().get_kind() != TokenKind::RParen {
                        let arg = self.expression();
                        args.push(arg);

                        if self.current().get_kind() != TokenKind::Comma { break; }
                        else { self.advance(); }
                    }
                    self.consume(TokenKind::RParen);

                    lhs = Expression::FunctionCall { name: Box::new(lhs), args };
                    true
                }
                Token::LBracket => {
                    let index = self.expression();

                    lhs = Expression::ArrayAccess { lhs: Box::new(lhs), index: Box::new(index) };
                    self.consume(TokenKind::RBracket);
                    true
                }
                Token::LBrace => {
                    let mut values = HashMap::new();
                    while self.current().get_kind() != TokenKind::RBrace {
                        let Token::Ident(val_name) = self.consume(TokenKind::Ident).value else { unreachable!(); };
                        self.consume(TokenKind::Colon);
                        let value = self.expression();
                        values.insert(val_name, value);

                        if self.current().get_kind() == TokenKind::Comma { self.advance(); }
                        else { break; }
                    }
                    self.consume(TokenKind::RBrace);
                    let Expression::Identifier(name) = lhs else { panic!("expected a struct name before initializer"); };
                    lhs = Expression::StructInitializer { name, values };
                    true
                }
                Token::Arrow => {
                    let Token::Ident(member) = self.consume(TokenKind::Ident).value else { unreachable!(); };
                    lhs = Expression::Unary { child: Box::new(lhs), op: Token::Star };
                    lhs = Expression::MemberAccess { lhs: Box::new(lhs), member };
                    true
                }
                Token::Dot => {
                    let Token::Ident(member) = self.consume(TokenKind::Ident).value else { unreachable!(); };
                    lhs = Expression::MemberAccess { lhs: Box::new(lhs), member };
                    true
                }
                _ => {
                    self.i -= 1;
                    false
                }
            } { /* "ughh she never pays any attention to me" uh huh for sure bud, maybe if you werent so useless here i would actually use you... did you ever consider that?!?*/ }

            lhs
        }
    }

    fn primary(&mut self) -> Expression {
        match self.advance().value {
            Token::Ident(x) => Expression::Identifier(x),
            Token::Integer(x) => Expression::Integer(x.parse::<i128>().unwrap()),
            Token::String(x) => Expression::String(x),
            Token::LParen => {
                let child = self.expression();
                self.consume(TokenKind::RParen);
                child
            }
            x => panic!("Unknown primary {x:?}"),
        }
    }
}
