use std::{cell::RefCell, collections::{HashMap, HashSet}, rc::Rc};

use crate::{config::Configuration, diagnostics::util::{error_align_caret, print_error_header, token_width}, import::resolve_path, lexer::{Lexer, Token, TokenMeta}};

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
    Defer,
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
            Self::Defer => TokenKind::Defer,
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

trait Ast: Sized {
    fn error_variant() -> Self;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    ParseError,
    Prim(String),
    Array {
        size: Option<Rc<Expression>>,
        kind: Rc<Type>,
    },
    Pointer {
        kind: Rc<Type>,
    },
    Ident(String),
    Function {
        ret: Rc<Type>,
        params: Vec<Rc<Type>>,
    },
}
impl Ast for Type {
    fn error_variant() -> Self {
        Type::ParseError
    }
}

#[derive(Debug, Clone)]
pub enum Declaration {
    ParseError,
    Struct {
        name: String,
        fields: HashMap<String, Rc<Type>>
    },
    Enum {
        name: String,
        modifiers: HashSet<String>,
        variants: Vec<String>,
    },
    Function {
        name: String,
        ret: Rc<Type>,
        params: Vec<(String, Rc<Type>)>, // (name, type)
        body: Rc<Statement>,
    },
    TypeDef {
        name: String,
        kind: Rc<Type>,
    },
    Statement(Rc<Statement>), // this is pretty much only for global variable declarations and imports
}
impl Ast for Declaration {
    fn error_variant() -> Self {
        Declaration::ParseError
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    ParseError,
    Import(String), // path (relative?)
    Return(Option<Rc<Expression>>),
    VarDecl {
        is_static: bool,
        name: String,
        kind: Option<Rc<Type>>,
        value: Option<Rc<Expression>>,
    },
    If {
        condition: Rc<Expression>,
        body: Rc<Statement>,
        alt: Option<Rc<Statement>>,
    },
    While {
        condition: Rc<Expression>,
        body: Rc<Statement>,
    },
    Break,
    Continue,
    Asm(String),
    Block(Vec<Rc<Statement>>),
    Expression(Rc<Expression>),
}
impl Ast for Statement {
    fn error_variant() -> Self {
        Statement::ParseError
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    ParseError,
    Unary {
        child: Rc<Expression>,
        op: Token,
    },
    Binary {
        lhs: Rc<Expression>,
        rhs: Rc<Expression>,
        op: Token,
    },
    FunctionCall {
        name: Rc<Expression>,
        args: Vec<Rc<Expression>>,
    },
    ArrayAccess {
        lhs: Rc<Expression>,
        index: Rc<Expression>,
    },
    MemberAccess {
        lhs: Rc<Expression>,
        member: String,
    },
    StructInitializer {
        name: String,
        values: HashMap<String, Rc<Expression>>
    },

    TypeCast {
        to: Rc<Type>,
        value: Rc<Expression>,
    },

    Integer(i128, String), // (value, type) : 69u32 -> (69, "u32")
    String(String),
    Char(char),
    Identifier(String),
    Group(Rc<Expression>),
}
impl Ast for Expression {
    fn error_variant() -> Self {
        Expression::ParseError
    }
}

impl Type {
    pub fn is_integer(&self) -> bool {
        matches!(self, Type::Prim(p) if matches!(p.as_str(), "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" | "usize" | "isize"))
    }

    }

// #[derive(Clone)]
pub struct Parser<'a> {
    config: &'a Configuration,

    filename: &'a str,
    source_lines: HashMap<usize, Vec<String>>, // file id -> lines

    tokens: Vec<TokenMeta>,
    i: usize,

    pub err_count: usize,

    pub inc_files: &'a mut Vec<String>,
}

fn concat_tokenlist(metalist: &[&[TokenKind]]) -> Vec<TokenKind> {
    metalist.iter().map(|x| x.iter().map(|x| *x).collect::<Vec<TokenKind>>()).flatten().collect()
}

macro_rules! consume {
    ( $self:expr, Ident, $message:expr, $error:tt, $( $follow:expr ),+ ) => {
        {
            let Some(Token::Ident(value)) = $self.consume(TokenKind::Ident).map(|x| x.value) else {
                $self.error($message, concat_tokenlist(&[$($follow),+]).as_slice());
                return Parser::add_error::<$error>();
            };
            value
        }
    };
    ( $self:expr, String, $message:expr, $error:tt, $( $follow:expr ),+ ) => {
        {
            let Some(Token::String(value)) = $self.consume(TokenKind::String).map(|x| x.value) else {
                $self.error($message, concat_tokenlist(&[$($follow),+]).as_slice());
                return Parser::add_error::<$error>();
            };
            value
        }
    };
    ( $self:expr, $kind:ident, $message:expr, $error:tt, $( $follow:expr ),+ ) => {
        {
            if $self.consume(TokenKind::$kind).is_none() {
                $self.error($message, concat_tokenlist(&[$($follow),+]).as_slice());
                return Parser::add_error::<$error>();
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
    pub fn from(config: &'a Configuration, filename: &'a str, source_lines: Vec<String>, tokens: Vec<TokenMeta>, inc_files: &'a mut Vec<String>) -> Self {
        inc_files.push(filename.to_owned());
        Self {
            config,
            filename,
            source_lines: {
                let mut slh = HashMap::new();
                slh.insert(0, source_lines);
                slh
            },

            tokens,
            i: 0usize,

            err_count: 0,

            inc_files,
        }
    }

    fn include_file(&mut self, path: String) -> (bool, usize) {
        match self.inc_files.iter().position(|p| p.clone() == path) {
            Some(i) => (false, i),
            None => {
                self.inc_files.push(path);
                (true, self.inc_files.len()-1)
            }
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

    fn error_no_sync(&mut self, msg: String) {
        self.err_count += 1;

        let curr = self.current();
        let (line, off) = error_align_caret(&self.source_lines.get(&curr.file_id).unwrap()[curr.pos.0], curr.pos.1);
        let tok_width = token_width(curr.to_owned());

        print_error_header(&self.inc_files[curr.file_id], curr.pos, msg);
        let line_num = format!("{} | ", curr.pos.0+1);
        eprintln!("{line_num}{}", line);
        eprintln!("{: >width$}{}\n", '^', std::iter::repeat_n('~', tok_width-1).collect::<String>(), width=off+line_num.len()+1);
    }
    fn error(&mut self, msg: String, sync_tokens: &[TokenKind]) {
        self.error_no_sync(msg);
        self.synchronize(sync_tokens);
    }
    fn synchronize(&mut self, sync_tokens: &[TokenKind]) {
        while self.i < self.tokens.len() && !sync_tokens.contains(&self.current().get_kind()) {
            self.advance();
        }
    }

    pub fn add_error<T: Ast>() -> Rc<T> {
        Rc::new(T::error_variant())
    }

    pub fn run(&mut self) -> Vec<Rc<Declaration>> {
        std::iter::from_fn(|| if self.i < self.tokens.len() { Some(self.declaration()) } else { None }).collect()
        // let mut program: Vec<Declaration> = Vec::new();

        // while self.i < self.tokens.len() {
        //     program.push(self.declaration());
        // }

        // program
    }

    fn declaration(&mut self) -> Rc<Declaration> {
        match &self.current().value {
            Token::Struct => self.struct_decl(),
            Token::Enum => self.enum_decl(),
            Token::Function => self.function_decl(),
            Token::TypeDef => self.typedef_decl(),
            Token::Import => {
                self.import_st();
                self.declaration()
            }
            Token::Static => {
                let var = self.var_decl();
                Rc::new(Declaration::Statement(var))
            }
            x => {
                self.error(
                    format!("invalid declaration: '{}'", x.get_plaintext()),
                    DECL_FOLLOW
                );
                Rc::new(Declaration::ParseError)
            }
        }
    }
    fn struct_decl(&mut self) -> Rc<Declaration> {
        self.advance();
        let name = consume!(self, Ident, "expected an identifier".into(), Declaration, DECL_FOLLOW);
        let mut fields: HashMap<String, Rc<Type>> = HashMap::new();

        consume!(self, LBrace, "expected '{'".into(), Declaration, DECL_FOLLOW);
        while let Token::Ident(field) = self.current().value.to_owned() {
            self.advance();
            consume!(self, Colon, "expected ':'".into(), Declaration, DECL_FOLLOW);
            let kind = self.parse_type();
            fields.insert(field.to_owned(), kind);
            consume!(self, Semi, "expected ';'".into(), Declaration, DECL_FOLLOW);
        }
        consume!(self, RBrace, "expected '}'".into(), Declaration, DECL_FOLLOW);

        Rc::new(Declaration::Struct { name, fields })
    }
    fn enum_decl(&mut self) -> Rc<Declaration> {
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

        Rc::new(Declaration::Enum { name, modifiers, variants })
    }
    fn function_decl(&mut self) -> Rc<Declaration> {
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

        let body = self.block();
        Rc::new(Declaration::Function { name, ret, params, body })
    }
    fn typedef_decl(&mut self) -> Rc<Declaration> {
        self.advance();

        let curr = self.current().value.to_owned();
        let new_type = consume!(self, Ident, format!("expected an identifier, recieved {curr:?}"), Declaration, DECL_FOLLOW);
        consume!(self, Equals, "expected '='".into(), Declaration, DECL_FOLLOW);
        let value = self.parse_type();
        consume!(self, Semi, "expected ';'".into(), Declaration, DECL_FOLLOW);

       Rc::new(Declaration::TypeDef { name: new_type, kind: value })
    }
    fn block_stmts(&mut self) -> Option<Vec<Rc<Statement>>> {
        let mut statements = Vec::new();

        if self.current().get_kind() == TokenKind::LBrace {
            if self.consume(TokenKind::LBrace).is_none() {
                self.error("expected '{'".into(), concat_tokenlist(&[STMT_FOLLOW]).as_slice());
                return None;
            }
            while self.current().get_kind() != TokenKind::RBrace {
                statements.push(self.statement());
            }
            if self.consume(TokenKind::RBrace).is_none() {
                self.error("expected '}'".into(), concat_tokenlist(&[STMT_FOLLOW]).as_slice());
                return None;
            }
        } else {
            statements.push(self.statement());
        }

        Some(statements)
    }
    fn block(&mut self) -> Rc<Statement> {
        let statements = self.block_stmts();
        match statements {
            None => Rc::new(Statement::ParseError),
            Some(stmts) => Rc::new(Statement::Block(stmts)),

        }
    }
    
    fn statement(&mut self) -> Rc<Statement> {
        match self.current().value {
            Token::Import => {
                self.import_st();
                self.statement()
            }
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
                Rc::new(Statement::Expression(expr))
            }
        }
    }
    fn import_st(&mut self) -> Rc<Statement> {
        // self.advance();
        // let mut len = 1; // "import"

        // let Some(Token::Ident(mut path)) = self.consume(TokenKind::Ident).map(|x| x.value) else {
        //     self.error("expected an identifier".into(), STMT_FOLLOW);
        //     return (self.add_error::<Statement>(), len);
        // };
        // len += 1;
        // while self.current().get_kind() == TokenKind::Dot {
        //     if self.consume(TokenKind::Dot).map(|x| x.value).is_none() {
        //         self.error("expected '.'".into(), STMT_FOLLOW);
        //         return (self.add_error::<Statement>(), len);
        //     };
        //     len += 1;
        //     let Some(Token::Ident(subdir)) = self.consume(TokenKind::Ident).map(|x| x.value) else {
        //         self.error("expected an identifier".into(), STMT_FOLLOW);
        //         return (self.add_error::<Statement>(), len);
        //     };
        //     len += 1;

        //     path.push('/');
        //     path.push_str(&subdir);
        // }

        // if self.consume(TokenKind::Semi).map(|x| x.value).is_none() {
        //     self.error("expected ';'".into(), STMT_FOLLOW);
        //     return (self.add_error::<Statement>(), len);
        // };
        // len += 1;

        // (self.add_stmt(Statement::Import(path)), len)

        self.advance();
        let mut len = 1;

        let mut path = consume!(self, Ident, "expected an identifier".into(), Statement, STMT_FOLLOW);
        len += 1;
        while self.current().get_kind() == TokenKind::Dot {
            consume!(self, Dot, "expected '.'".into(), Statement, STMT_FOLLOW);
            len += 1;
            let subdir = consume!(self, Ident, "expected an identifier".into(), Statement, STMT_FOLLOW);
            len += 1;

            path.push('/');
            path.push_str(&subdir);
        }

        consume!(self, Semi, "expected ';'".into(), Statement, STMT_FOLLOW);
        len += 1;

        self.tokens.drain((self.i-len)..self.i);
        self.i -= len;

        path.push_str(".is");
        if let Some(full_path) = resolve_path(self.config, path.clone()) {
            let key = std::fs::canonicalize(&full_path) // basically just to make sure `../` gets flattened
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or(full_path.clone());

            let (unique, id) = self.include_file(key);
            if unique {
                if let Ok(file_contents) = std::fs::read_to_string(&full_path) {
                    let lines: Vec<String> = file_contents.lines().map(|x| x.to_owned()).collect();
                    let mut lexer = Lexer::from(&full_path, id, &lines, &file_contents);
                    let tokens = lexer.run();
                    self.source_lines.insert(id, lines);

                    self.tokens.splice(self.i..self.i, tokens);
                } else {
                    self.error_no_sync(format!("could not open imported file"));
                }
            }
        } else {
            self.i -= 2;
            self.error_no_sync("could not find file".to_owned());
            self.i += 2;
        }

        Rc::new(Statement::Import(path))
    }
    fn return_st(&mut self) -> Rc<Statement> {
        self.advance();
        if self.current().get_kind() == TokenKind::Semi {
            self.advance();
            Rc::new(Statement::Return(None))
        } else {
            let value = self.expression();
            consume!(self, Semi, "expected ';'".into(), Statement, STMT_FOLLOW);
            Rc::new(Statement::Return(Some(value)))
        }
    }
    fn var_decl(&mut self) -> Rc<Statement> {
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

        Rc::new(Statement::VarDecl { is_static, name, kind, value })
    }
    fn if_st(&mut self) -> Rc<Statement> {
        self.advance();

        consume!(self, LParen, "expected '('".into(), Statement, STMT_FOLLOW);
        let condition = self.expression();
        consume!(self, RParen, "expected ')'".into(), Statement, STMT_FOLLOW);

        let body = self.block();

        let alt = if self.current().get_kind() == TokenKind::Else {
            self.advance();
            Some(self.statement())
        } else { None };

        Rc::new(Statement::If { condition, body: body, alt })
    }
    fn while_st(&mut self) -> Rc<Statement> {
        self.advance();

        consume!(self, LParen, "expected '('".into(), Statement, STMT_FOLLOW);
        let condition = self.expression();
        consume!(self, RParen, "expected ')'".into(), Statement, STMT_FOLLOW);

        let body = self.block();
        Rc::new(Statement::While { condition, body })
    }
    fn asm_st(&mut self) -> Rc<Statement> {
        self.advance();
        
        let value = consume!(self, String, "expected a string".into(), Statement, STMT_FOLLOW);

        consume!(self, Semi, "expected ';'".into(), Statement, STMT_FOLLOW);
        Rc::new(Statement::Asm(value))
    }
    fn for_st(&mut self) -> Rc<Statement> {
        self.advance();

        consume!(self, LParen, "expected '('".into(), Statement, STMT_FOLLOW, &[TokenKind::Semi]);
        let init = self.expression();
        consume!(self, Semi, "expected ';'".into(), Statement, STMT_FOLLOW, &[TokenKind::Semi]);

        let condition = self.expression();
        consume!(self, Semi, "expected ';'".into(), Statement, STMT_FOLLOW, &[TokenKind::Semi]);

        let updater = self.expression();
        consume!(self, Semi, "expected ';'".into(), Statement, STMT_FOLLOW);
        consume!(self, RParen, "expected ')'".into(), Statement, STMT_FOLLOW);

        let body_stmts = self.block_stmts();
        if body_stmts.is_none() {
            return Rc::new(Statement::ParseError);
        }
        let mut body_stmts = body_stmts.unwrap();

        let updater_stmt = Rc::new(Statement::Expression(updater));
        body_stmts.push(updater_stmt);
        let body = Rc::new(Statement::Block(body_stmts));

        let init_stmt = Rc::new(Statement::Expression(init));
        let while_stmt = Rc::new(Statement::While { condition, body });
        Rc::new(Statement::Block(vec![ init_stmt, while_stmt ]))
    }

    fn parse_type(&mut self) -> Rc<Type> {
        match self.current().value.to_owned() {
            Token::LBracket => {
                self.advance();
                let size = if self.current().get_kind() != TokenKind::RBracket {
                    Some(self.expression())
                } else { None };
                consume!(self, RBracket, "expected ']'".into(), Type, TYPE_FOLLOW);
                let kind = self.parse_type();
                Rc::new(Type::Array { size, kind })
            }
            Token::At => {
                self.advance();
                let kind = self.parse_type();
                Rc::new(Type::Pointer { kind })
            }
            Token::FuncType => {
                self.advance();

                consume!(self, LessThan, "expected '<'".into(), Type, TYPE_FOLLOW);
                self.consume(TokenKind::LessThan);
                let ret = self.parse_type();

                let mut params = Vec::new();
                while self.current().get_kind() == TokenKind::Comma {
                    self.advance();
                    params.push(self.parse_type());
                }

                consume!(self, MoreThan, "expected '>'".into(), Type, TYPE_FOLLOW);
                Rc::new(Type::Function { ret, params })
            }
            Token::PrimType(kind) => {
                self.advance();
                Rc::new(Type::Prim(kind.to_owned()))
            }
            Token::Ident(kind) => {
                self.advance();
                Rc::new(Type::Ident(kind.to_owned()))
            }
            x => {
                self.error(format!("invalid type: '{}'", x.get_plaintext()), TYPE_FOLLOW);
                Parser::add_error::<Type>()
            }
        }
    }

    fn left_rec(&mut self, symbols: &[TokenKind], child: fn(&mut Parser<'a>) -> Rc<Expression>) -> Rc<Expression> {
        let mut lhs = child(self);

        while symbols.contains(&self.current().get_kind()) {
            let op = self.advance();
            let rhs = child(self);
            lhs = Rc::new(Expression::Binary { lhs, rhs, op: op.value })
        }

        lhs
    }
    fn right_rec(&mut self,
        symbols: &[TokenKind],
        parent: fn(&mut Parser<'a>) -> Rc<Expression>,
        child: fn(&mut Parser<'a>) -> Rc<Expression>,
    ) -> Rc<Expression> {
        let lhs = child(self);

        if symbols.contains(&self.current().get_kind()) {
            let op = self.advance();
            let rhs = parent(self);
            Rc::new(Expression::Binary { lhs, rhs, op: op.value })
        } else { lhs }
    }
    fn expression(&mut self) -> Rc<Expression> { self.assignment() }
    fn assignment(&mut self) -> Rc<Expression> { self.right_rec(&[TokenKind::Equals], Self::assignment, Self::logical_or) }
    fn logical_or(&mut self) -> Rc<Expression> { self.left_rec(&[TokenKind::LogOr], Self::logical_and) }
    fn logical_and(&mut self) -> Rc<Expression> { self.left_rec(&[TokenKind::LogAnd], Self::equality) }
    fn equality(&mut self) -> Rc<Expression> {
        let mut lhs = self.relation();

        if [TokenKind::EqualTo,
            TokenKind::NotEqualTo,
        ].contains(&self.current().get_kind()) {
            let op = self.advance();
            let rhs = self.relation();
            lhs = Rc::new(Expression::Binary { lhs, rhs, op: op.value });
        }

        lhs
    }
    fn relation(&mut self) -> Rc<Expression> {
        let mut lhs = self.bitwise_or();

        if [TokenKind::LessThan,
            TokenKind::MoreThan,
            TokenKind::MoreThanEq,
            TokenKind::LessThanEq
        ].contains(&self.current().get_kind()) {
            let op = self.advance();
            let rhs = self.bitwise_or();
            lhs = Rc::new(Expression::Binary { lhs, rhs, op: op.value });
        }

        lhs
    }
    fn bitwise_or(&mut self) -> Rc<Expression> { self.left_rec(&[TokenKind::BitOr], Self::bitwise_xor) }
    fn bitwise_xor(&mut self) -> Rc<Expression> { self.left_rec(&[TokenKind::BitXor], Self::bitwise_and) }
    fn bitwise_and(&mut self) -> Rc<Expression> { self.left_rec(&[TokenKind::Ampersand], Self::shift) }
    fn shift(&mut self) -> Rc<Expression> { self.left_rec(&[TokenKind::LShift, TokenKind::RShift], Self::addition) }
    fn addition(&mut self) -> Rc<Expression> { self.left_rec(&[TokenKind::Plus, TokenKind::Minus], Self::multiplication) }
    fn multiplication(&mut self) -> Rc<Expression> { self.left_rec(&[TokenKind::Star, TokenKind::Slash, TokenKind::Percent], Self::unary)}
    fn unary(&mut self) -> Rc<Expression> {
        if [TokenKind::LogNot,
            TokenKind::Minus,
            TokenKind::BitNeg
        ].contains(&self.current().get_kind()) {
            let op = self.advance();
            let child = self.unary();
            Rc::new(Expression::Unary { child, op: op.value })
        } else { self.reference() }
    }
    fn reference(&mut self) -> Rc<Expression> {
        if self.current().get_kind() == TokenKind::Ampersand {
            let op = self.advance();
            let child = self.access();
            Rc::new(Expression::Unary { child, op: op.value })
        } else { self.access() }
    }
    fn access(&mut self) -> Rc<Expression> {
        if self.current().get_kind() == TokenKind::Cast {
            self.advance();
            consume!(self, LParen, "expected '('".into(), Expression, EXPR_FOLLOW);

            let to = self.parse_type();
            consume!(self, Comma, "expected ','".into(), Expression, EXPR_FOLLOW);

            let value = self.expression();

            consume!(self, RParen, "expected ')'".into(), Expression, EXPR_FOLLOW);
            Rc::new(Expression::TypeCast { to, value })
        } else if self.current().get_kind() == TokenKind::At {
            let op = self.advance();
            let child = self.primary();
            Rc::new(Expression::Unary { child, op: op.value })
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

                    lhs = Rc::new(Expression::FunctionCall { name: lhs, args });
                    true
                }
                Token::LBracket => {
                    let index = self.expression();

                    lhs = Rc::new(Expression::ArrayAccess { lhs, index });
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
                    let name = match &*lhs {
                        Expression::Identifier(name) => name.clone(),
                        _ => panic!("expected a struct name before initializer"),
                    };
                    lhs = Rc::new(Expression::StructInitializer { name, values });
                    true
                }
                Token::Arrow => {
                    let member = consume!(self, Ident, "expected an identifer".into(), Expression, EXPR_FOLLOW);
                    lhs = Rc::new(Expression::Unary { child: lhs, op: Token::Star });
                    lhs = Rc::new(Expression::MemberAccess { lhs, member });
                    true
                }
                Token::Dot => {
                    let member = consume!(self, Ident, "expected an identifer".into(), Expression, EXPR_FOLLOW);
                    lhs = Rc::new(Expression::MemberAccess { lhs, member });
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

    fn primary(&mut self) -> Rc<Expression> {
        match self.advance().value {
            Token::Ident(x) => Rc::new(Expression::Identifier(x)),
            Token::Integer(x, k) => Rc::new(Expression::Integer(x.parse::<i128>().unwrap(), k)),
            Token::String(x) => Rc::new(Expression::String(x)),
            Token::Char(x) => Rc::new(Expression::Char(x)),
            Token::LParen => {
                let child = self.expression();
                consume!(self, RParen, "expected ')'".into(), Expression, EXPR_FOLLOW);
                child
            }
            x => {
                self.i -= 1; // go back to fix alignment
                self.error(format!("'{}' is not an expression", x.get_plaintext()), EXPR_FOLLOW);
                Parser::add_error::<Expression>()
            }
        }
    }
}
