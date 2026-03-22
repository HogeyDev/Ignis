use std::collections::{HashMap, HashSet};

use crate::{diagnostics::{util::{error_align_caret, print_error_header, token_width}}, lexer::{Token, TokenMeta}};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
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

    Ident,
    String,
    Integer,
    Char,

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
            Self::Continue => TokenKind::Continue,
            Self::Function => TokenKind::Function,
            Self::TypeDef => TokenKind::TypeDef,
            Self::Import => TokenKind::Import,
            Self::Return => TokenKind::Return,
            Self::Static => TokenKind::Static,
            Self::Struct => TokenKind::Struct,
            Self::Break => TokenKind::Break,
            Self::While => TokenKind::While,
            Self::Cast => TokenKind::Cast,
            Self::Else => TokenKind::Else,
            Self::Enum => TokenKind::Enum,
            Self::Spec => TokenKind::Spec,
            Self::Asm => TokenKind::Asm,
            Self::For => TokenKind::For,
            Self::Let => TokenKind::Let,
            Self::If => TokenKind::If,

            Self::Ident(_) => TokenKind::Ident,
            Self::String(_) => TokenKind::String,
            Self::Integer(_, _) => TokenKind::Integer,
            Self::Char(_) => TokenKind::Char,

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
            Self::DoubleEquals => TokenKind::EqualTo,
            Self::NotEquals => TokenKind::NotEqualTo,
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

pub type RootAST = Vec<Declaration>;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    ParseError,
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
    ParseError,
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
    ParseError,
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
    Break,
    Continue,
    Asm(String),
    Block(Vec<Statement>),
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    ParseError,
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

    Integer(i128, String), // (value, type) : 69u32 -> (69, "u32")
    String(String),
    Char(char),
    Identifier(String),
    Group(Box<Expression>),
}

pub struct Parser<'a> {
    filename: &'a str,
    source_lines: Vec<String>,

    tokens: Vec<TokenMeta>,
    i: usize,

    pub err_count: usize,
    // warn_count: usize,
}

fn concat_tokenlist(metalist: &[&[TokenKind]]) -> Vec<TokenKind> {
    metalist.iter().map(|x| x.iter().map(|x| *x).collect::<Vec<TokenKind>>()).flatten().collect()
}

macro_rules! consume {
    ( $self:expr, Ident, $message:expr, $error:tt, $( $follow:expr ),+ ) => {
        {
            let Some(Token::Ident(value)) = $self.consume(TokenKind::Ident).map(|x| x.value) else {
                $self.error($message, concat_tokenlist(&[$($follow),+]).as_slice());
                return $error::ParseError;
            };
            value
        }
    };
    ( $self:expr, String, $message:expr, $error:tt, $( $follow:expr ),+ ) => {
        {
            let Some(Token::String(value)) = $self.consume(TokenKind::String).map(|x| x.value) else {
                $self.error($message, concat_tokenlist(&[$($follow),+]).as_slice());
                return $error::ParseError;
            };
            value
        }
    };
    ( $self:expr, $kind:ident, $message:expr, $error:tt, $( $follow:expr ),+ ) => {
        {
            if $self.consume(TokenKind::$kind).is_none() {
                $self.error($message, concat_tokenlist(&[$($follow),+]).as_slice());
                return $error::ParseError;
            }
        }
    };
}

macro_rules! tlist {
    ( $( $var:ident ),* ) => {
        &[ $( TokenKind::$var ),* ]
    };
}

const DECL_FOLLOW: &[TokenKind] = tlist![
    Struct, Enum, Function,
    TypeDef, Import, Static
];
const STMT_FOLLOW: &[TokenKind] = tlist![
    Import, Return, Let, If, While,
    For, Asm, LBrace, RBrace, LogNot,
    Ampersand, At, Cast, Integer,
    String, Ident, LParen
];
const TYPE_FOLLOW: &[TokenKind] = tlist![Comma, RParen, Equals, Semi, MoreThan];
const EXPR_FOLLOW: &[TokenKind] = tlist![Comma, RParen, RBracket, Semi, RBrace];

impl<'a> Parser<'a> {
    pub fn from(filename: &'a str, source_lines: Vec<String>, tokens: Vec<TokenMeta>) -> Self {
        Self {
            filename,
            source_lines,

            tokens,
            i: 0usize,

            err_count: 0,
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
    fn consume(&mut self, kind: TokenKind) -> Option<TokenMeta> {
        let curr = self.current();
        let curr_kind = curr.get_kind();
        if curr_kind == kind {
            return Some(self.advance());
        }
        // eprintln!("expected {kind:?}, but instead got {curr_kind:?}"); // idk if i really want this line anymore
        None
    }

    fn error(&mut self, msg: String, sync_tokens: &[TokenKind]) {
        self.err_count += 1;

        let curr = self.current();
        let (line, off) = error_align_caret(&self.source_lines[curr.pos.0], curr.pos.1);
        let tok_width = token_width(curr.to_owned());

        print_error_header(self.filename, curr.pos, msg);
        let line_num = format!("{} | ", curr.pos.1+1);
        eprintln!("{line_num}{}", line);
        eprintln!("{: >width$}{}\n", '^', std::iter::repeat_n('~', tok_width-1).collect::<String>(), width=off+line_num.len()+1);

        self.synchronize(sync_tokens);
    }
    fn synchronize(&mut self, sync_tokens: &[TokenKind]) {
        while self.i < self.tokens.len() && !sync_tokens.contains(&self.current().get_kind()) {
            self.advance();
        }
    }

    pub fn run(&mut self) -> Vec<Declaration> {
        std::iter::from_fn(|| if self.i < self.tokens.len() { Some(self.declaration()) } else { None }).collect()
        // let mut program: Vec<Declaration> = Vec::new();

        // while self.i < self.tokens.len() {
        //     program.push(self.declaration());
        // }

        // program
    }

    fn declaration(&mut self) -> Declaration {
        match &self.current().value {
            Token::Struct => self.struct_decl(),
            Token::Enum => self.enum_decl(),
            Token::Function => self.function_decl(),
            Token::TypeDef => self.typedef_decl(),
            Token::Import => Declaration::Statement(self.import_st()),
            Token::Static => Declaration::Statement(self.var_decl()),
            x => {
                self.error(
                    format!("invalid declaration: '{}'", x.get_plaintext()),
                    DECL_FOLLOW
                );
                Declaration::ParseError
            }
        }
    }
    fn struct_decl(&mut self) -> Declaration {
        self.advance();
        let name = consume!(self, Ident, "expected an identifier".into(), Declaration, DECL_FOLLOW);
        let mut fields: HashMap<String, Type> = HashMap::new();

        consume!(self, LBrace, "expected '{'".into(), Declaration, DECL_FOLLOW);
        while let Token::Ident(field) = self.current().value.to_owned() {
            self.advance();
            consume!(self, Colon, "expected ':'".into(), Declaration, DECL_FOLLOW);
            let kind = self.parse_type();
            fields.insert(field.to_owned(), kind);
            consume!(self, Semi, "expected ';'".into(), Declaration, DECL_FOLLOW);
        }
        consume!(self, RBrace, "expected '}'".into(), Declaration, DECL_FOLLOW);

        Declaration::Struct { name, fields }
    }
    fn enum_decl(&mut self) -> Declaration {
        self.advance();
        let name = consume!(self, Ident, "expected an identifier".into(), Declaration, DECL_FOLLOW);
        let mut variants: Vec<String> = Vec::new();

        let mut modifiers: HashSet<String> = HashSet::new();
        if let Token::LBracket = self.current().value {
            self.advance();
            while let Token::Ident(modifier) = self.current().value.to_owned() {
                self.advance();
                modifiers.insert(modifier);
            }
            consume!(self, RBracket, "expected ']'".into(), Declaration, &[TokenKind::LBrace], DECL_FOLLOW);
        }

        consume!(self, LBrace, "expected '{'".into(), Declaration, DECL_FOLLOW);
        while let Token::Ident(variant) = self.current().value.to_owned() {
            self.advance();
            variants.push(variant);
            if self.current().get_kind() != TokenKind::Comma { break; }
            else { self.advance(); }
        }
        consume!(self, RBrace, "expected '}'".into(), Declaration, DECL_FOLLOW);

        Declaration::Enum { name, modifiers, variants }
    }
    fn function_decl(&mut self) -> Declaration {
        self.advance();

        let name = consume!(self, Ident, "expected an identifier".into(), Declaration, DECL_FOLLOW);

        consume!(self, LParen, "expected '('".into(), Declaration, DECL_FOLLOW);
        let ret = self.parse_type();

        let mut params = Vec::new();
        while self.current().get_kind() == TokenKind::Comma {
            consume!(self, Comma, "expeced ','".into(), Declaration, &[TokenKind::RParen], DECL_FOLLOW);

            let param_name = consume!(self, Ident, "expected an identifier".into(), Declaration, &[TokenKind::RParen], DECL_FOLLOW);
            consume!(self, Colon, "expected ':'".into(), Declaration, &[TokenKind::RParen], DECL_FOLLOW);
            let param_type = self.parse_type();
            
            params.push((param_name, param_type));
        }
        consume!(self, RParen, "expected ')'".into(), Declaration, DECL_FOLLOW);

        Declaration::Function { name, ret, params, body: self.block() }
    }
    fn typedef_decl(&mut self) -> Declaration {
        self.advance();

        let curr = self.current().value.to_owned();
        let new_type = consume!(self, Ident, format!("expected an identifier, recieved {curr:?}"), Declaration, DECL_FOLLOW);
        consume!(self, Equals, "expected '='".into(), Declaration, DECL_FOLLOW);
        let value = self.parse_type();
        consume!(self, Semi, "expected ';'".into(), Declaration, DECL_FOLLOW);

        Declaration::TypeDef { name: new_type, kind: value }
    }
    fn block(&mut self) -> Statement {
        let mut statements = Vec::new();

        if self.current().get_kind() == TokenKind::LBrace {
            consume!(self, LBrace, "expected '{'".into(), Statement, STMT_FOLLOW);
            while self.current().get_kind() != TokenKind::RBrace {
                statements.push(self.statement());
            }
            consume!(self, RBrace, "expected '}'".into(), Statement, STMT_FOLLOW);
        } else {
            statements.push(self.statement());
        }

        Statement::Block(statements)
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
                consume!(self, Semi, "expected ';'".into(), Statement, STMT_FOLLOW);
                Statement::Expression(expr)
            }
        }
    }
    fn import_st(&mut self) -> Statement {
        self.advance();

        let mut path = consume!(self, Ident, "expected an identifier".into(), Statement, STMT_FOLLOW);
        while self.current().get_kind() == TokenKind::Dot {
            consume!(self, Dot, "expected '.'".into(), Statement, STMT_FOLLOW);
            let subdir = consume!(self, Ident, "expected an identifier".into(), Statement, STMT_FOLLOW);

            path.push('/');
            path.push_str(&subdir);
        }

        consume!(self, Semi, "expected ';'".into(), Statement, STMT_FOLLOW);

        Statement::Import(path)
    }
    fn return_st(&mut self) -> Statement {
        self.advance();
        if self.current().get_kind() == TokenKind::Semi {
            self.advance();
            Statement::Return(None)
        } else {
            let value = self.expression();
            consume!(self, Semi, "expected ';'".into(), Statement, STMT_FOLLOW);
            Statement::Return(Some(value))
        }
    }
    fn var_decl(&mut self) -> Statement {
        let is_static = match self.advance().value {
            Token::Let => false,
            Token::Static => true,
            x => unreachable!("Variable is neither static nor non-static. What are you?\n\t{x:?}"),
        };

        let name = consume!(self, Ident, "expected an identifier".into(), Statement, STMT_FOLLOW);

        let kind = if self.current().get_kind() == TokenKind::Colon {
            consume!(self, Colon, "expected ':'".into(), Statement, STMT_FOLLOW);
            let kind = self.parse_type();
            Some(kind)
        } else {
            None
        };

        let value = if self.current().get_kind() == TokenKind::Equals {
            consume!(self, Equals, "expected '='".into(), Statement, STMT_FOLLOW);
            let expr = self.expression();
            Some(expr)
        } else {
            None
        };

        consume!(self, Semi, "expected ';'".into(), Statement, STMT_FOLLOW);

        Statement::VarDecl { is_static, name, kind, value }
    }
    fn if_st(&mut self) -> Statement {
        self.advance();

        consume!(self, LParen, "expected '('".into(), Statement, STMT_FOLLOW);
        let condition = self.expression();
        consume!(self, RParen, "expected ')'".into(), Statement, STMT_FOLLOW);

        let body = self.block();

        let alt = if self.current().get_kind() == TokenKind::Else {
            self.advance();
            Some(Box::new(self.statement()))
        } else { None };

        Statement::If { condition, body: Box::new(body), alt }
    }
    fn while_st(&mut self) -> Statement {
        self.advance();

        consume!(self, LParen, "expected '('".into(), Statement, STMT_FOLLOW);
        let condition = self.expression();
        consume!(self, RParen, "expected ')'".into(), Statement, STMT_FOLLOW);

        let body = self.block();

        Statement::While { condition, body: Box::new(body) }
    }
    fn asm_st(&mut self) -> Statement {
        self.advance();
        
        let value = consume!(self, String, "expected a string".into(), Statement, STMT_FOLLOW);

        consume!(self, Semi, "expected ';'".into(), Statement, STMT_FOLLOW);
        Statement::Asm(value)
    }
    fn for_st(&mut self) -> Statement {
        self.advance();

        consume!(self, LParen, "expected '('".into(), Statement, STMT_FOLLOW, &[TokenKind::Semi]);
        let init = self.expression();
        consume!(self, Semi, "expected ';'".into(), Statement, STMT_FOLLOW, &[TokenKind::Semi]);

        let condition = self.expression();
        consume!(self, Semi, "expected ';'".into(), Statement, STMT_FOLLOW, &[TokenKind::Semi]);

        let updater = self.expression();
        consume!(self, Semi, "expected ';'".into(), Statement, STMT_FOLLOW);
        consume!(self, RParen, "expected ')'".into(), Statement, STMT_FOLLOW);

        let mut body = self.block();
        match body {
            Statement::Block(ref mut xs) => xs.push(Statement::Expression(updater)),
            Statement::ParseError => return Statement::ParseError,
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
                self.advance();
                let size = if self.current().get_kind() != TokenKind::RBracket {
                    Some(self.expression())
                } else { None };
                consume!(self, RBracket, "expected ']'".into(), Type, TYPE_FOLLOW);
                let child = self.parse_type();
                Type::Array { size, kind: Box::new(child) }
            }
            Token::At => {
                self.advance();
                let child = self.parse_type();
                Type::Pointer { kind: Box::new(child) }
            }
            Token::FuncType => {
                self.advance();

                consume!(self, LessThan, "expected '<'".into(), Type, TYPE_FOLLOW);
                self.consume(TokenKind::LessThan);
                let ret_type = self.parse_type();

                let mut params = Vec::new();
                while self.current().get_kind() == TokenKind::Comma {
                    self.advance();
                    params.push(self.parse_type());
                }

                consume!(self, MoreThan, "expected '>'".into(), Type, TYPE_FOLLOW);
                Type::Function { ret: Box::new(ret_type), params }
            }
            Token::PrimType(kind) => {
                self.advance();
                Type::Prim(kind.to_owned())
            }
            Token::Ident(kind) => {
                self.advance();
                Type::Ident(kind.to_owned())
            }
            x => {
                self.error(format!("invalid type: '{}'", x.get_plaintext()), TYPE_FOLLOW);
                Type::ParseError
            }
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
            consume!(self, LParen, "expected '('".into(), Expression, EXPR_FOLLOW);

            let to = self.parse_type();
            consume!(self, Comma, "expected ','".into(), Expression, EXPR_FOLLOW);

            let value = self.expression();

            consume!(self, RParen, "expected ')'".into(), Expression, EXPR_FOLLOW);
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
                    consume!(self, RParen, "expected ')'".into(), Expression, EXPR_FOLLOW);

                    lhs = Expression::FunctionCall { name: Box::new(lhs), args };
                    true
                }
                Token::LBracket => {
                    let index = self.expression();

                    lhs = Expression::ArrayAccess { lhs: Box::new(lhs), index: Box::new(index) };
                    consume!(self, RBracket, "expected ']'".into(), Expression, EXPR_FOLLOW);
                    true
                }
                Token::LBrace => {
                    let mut values = HashMap::new();
                    while self.current().get_kind() != TokenKind::RBrace {
                        let val_name = consume!(self, Ident, "expected an identifer".into(), Expression, EXPR_FOLLOW);
                        consume!(self, Colon, "expected ':'".into(), Expression, EXPR_FOLLOW);
                        let value = self.expression();
                        values.insert(val_name, value);

                        if self.current().get_kind() == TokenKind::Comma { self.advance(); }
                        else { break; }
                    }
                    consume!(self, RBrace, "expected '}'".into(), Expression, EXPR_FOLLOW);
                    let Expression::Identifier(name) = lhs else { panic!("expected a struct name before initializer"); };
                    lhs = Expression::StructInitializer { name, values };
                    true
                }
                Token::Arrow => {
                    let member = consume!(self, Ident, "expected an identifer".into(), Expression, EXPR_FOLLOW);
                    lhs = Expression::Unary { child: Box::new(lhs), op: Token::Star };
                    lhs = Expression::MemberAccess { lhs: Box::new(lhs), member };
                    true
                }
                Token::Dot => {
                    let member = consume!(self, Ident, "expected an identifer".into(), Expression, EXPR_FOLLOW);
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
            Token::Integer(x, k) => Expression::Integer(x.parse::<i128>().unwrap(), k),
            Token::String(x) => Expression::String(x),
            Token::Char(x) => Expression::Char(x),
            Token::LParen => {
                let child = self.expression();
                consume!(self, RParen, "expected ')'".into(), Expression, EXPR_FOLLOW);
                child
            }
            x => {
                self.i -= 1; // go back to fix alignment
                self.error(format!("'{}' is not an expression", x.get_plaintext()), EXPR_FOLLOW);
                Expression::ParseError
            }
        }
    }
}
