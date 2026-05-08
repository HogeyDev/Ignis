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

trait Ast: Sized {
    type Id;

    fn error_variant() -> Self;
    fn add_to_arena(self, parser: &mut Parser) -> Self::Id;
}
pub type RootAst = Vec<DeclarationId>;

pub type TypeId = usize;
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    ParseError,
    Prim(String),
    Array {
        size: Option<ExpressionId>,
        kind: TypeId,
    },
    Pointer {
        kind: TypeId,
    },
    Ident(String),
    Function {
        ret: TypeId,
        params: Vec<TypeId>,
    },
}
impl Ast for Type {
    type Id = TypeId;

    fn error_variant() -> Self {
        Type::ParseError
    }
    fn add_to_arena(self, parser: &mut Parser) -> Self::Id {
        parser.add_type(self)
    }
}

pub type DeclarationId = usize;
#[derive(Debug, Clone)]
pub enum Declaration {
    ParseError,
    Struct {
        name: String,
        fields: HashMap<String, TypeId>
    },
    Enum {
        name: String,
        modifiers: HashSet<String>,
        variants: Vec<String>,
    },
    Function {
        name: String,
        ret: TypeId,
        params: Vec<(String, TypeId)>, // (name, type)
        body: StatementId,
    },
    TypeDef {
        name: String,
        kind: TypeId,
    },
    Statement(StatementId), // this is pretty much only for global variable declarations and imports
}
impl Ast for Declaration {
    type Id = DeclarationId;

    fn error_variant() -> Self {
        Declaration::ParseError
    }
    fn add_to_arena(self, parser: &mut Parser) -> Self::Id {
        parser.add_decl(self)
    }
}

pub type StatementId = usize;
#[derive(Debug, Clone)]
pub enum Statement {
    ParseError,
    Import(String), // path (relative?)
    Return(Option<ExpressionId>),
    VarDecl {
        is_static: bool,
        name: String,
        kind: Option<TypeId>,
        value: Option<ExpressionId>,
    },
    If {
        condition: ExpressionId,
        body: StatementId,
        alt: Option<StatementId>,
    },
    While {
        condition: ExpressionId,
        body: StatementId,
    },
    Break,
    Continue,
    Asm(String),
    Block(Vec<StatementId>),
    Expression(ExpressionId),
}
impl Ast for Statement {
    type Id = StatementId;

    fn error_variant() -> Self {
        Statement::ParseError
    }
    fn add_to_arena(self, parser: &mut Parser) -> Self::Id {
        parser.add_stmt(self)
    }
}

pub type ExpressionId = usize;
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    ParseError,
    Unary {
        child: ExpressionId,
        op: Token,
    },
    Binary {
        lhs: ExpressionId,
        rhs: ExpressionId,
        op: Token,
    },
    FunctionCall {
        name: ExpressionId,
        args: Vec<ExpressionId>,
    },
    ArrayAccess {
        lhs: ExpressionId,
        index: ExpressionId,
    },
    MemberAccess {
        lhs: ExpressionId,
        member: String,
    },
    StructInitializer {
        name: String,
        values: HashMap<String, ExpressionId>
    },

    TypeCast {
        to: TypeId,
        value: ExpressionId,
    },

    Integer(i128, String), // (value, type) : 69u32 -> (69, "u32")
    String(String),
    Char(char),
    Identifier(String),
    Group(ExpressionId),
}
impl Ast for Expression {
    type Id = ExpressionId;

    fn error_variant() -> Self {
        Expression::ParseError
    }
    fn add_to_arena(self, parser: &mut Parser) -> Self::Id {
        parser.add_expr(self)
    }
}

impl Type {
    pub fn is_integer(&self) -> bool {
        matches!(self, Type::Prim(p) if matches!(p.as_str(), "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" | "usize" | "isize"))
    }
}

pub struct Parser<'a> {
    filename: &'a str,
    source_lines: Vec<String>,

    tokens: Vec<TokenMeta>,
    i: usize,

    pub err_count: usize,
    // warn_count: usize,

    pub declaration_arena: Vec<Declaration>,
    pub statement_arena: Vec<Statement>,
    pub expression_arena: Vec<Expression>,
    pub type_arena: Vec<Type>,
}

fn concat_tokenlist(metalist: &[&[TokenKind]]) -> Vec<TokenKind> {
    metalist.iter().map(|x| x.iter().map(|x| *x).collect::<Vec<TokenKind>>()).flatten().collect()
}

macro_rules! consume {
    ( $self:expr, Ident, $message:expr, $error:tt, $( $follow:expr ),+ ) => {
        {
            let Some(Token::Ident(value)) = $self.consume(TokenKind::Ident).map(|x| x.value) else {
                $self.error($message, concat_tokenlist(&[$($follow),+]).as_slice());
                return $self.add_error::<$error>();
            };
            value
        }
    };
    ( $self:expr, String, $message:expr, $error:tt, $( $follow:expr ),+ ) => {
        {
            let Some(Token::String(value)) = $self.consume(TokenKind::String).map(|x| x.value) else {
                $self.error($message, concat_tokenlist(&[$($follow),+]).as_slice());
                return $self.add_error::<$error>();
            };
            value
        }
    };
    ( $self:expr, $kind:ident, $message:expr, $error:tt, $( $follow:expr ),+ ) => {
        {
            if $self.consume(TokenKind::$kind).is_none() {
                $self.error($message, concat_tokenlist(&[$($follow),+]).as_slice());
                return $self.add_error::<$error>();
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

            declaration_arena: Vec::new(),
            statement_arena: Vec::new(),
            expression_arena: Vec::new(),
            type_arena: Vec::new(),
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

    pub fn add_decl(&mut self, decl: Declaration) -> DeclarationId {
        let id = self.declaration_arena.len();
        self.declaration_arena.push(decl);
        id
    }
    pub fn add_stmt(&mut self, stmt: Statement) -> StatementId {
        let id = self.statement_arena.len();
        self.statement_arena.push(stmt);
        id
    }
    pub fn add_expr(&mut self, expr: Expression) -> ExpressionId {
        let id = self.expression_arena.len();
        self.expression_arena.push(expr);
        id
    }
    pub fn add_type(&mut self, kind: Type) -> TypeId {
        let id = self.type_arena.len();
        self.type_arena.push(kind);
        id
    }

    fn add_error<T: Ast>(&mut self) -> T::Id {
        let err_node = T::error_variant();
        err_node.add_to_arena(self)
    }

    pub fn run(&mut self) -> RootAst {
        std::iter::from_fn(|| if self.i < self.tokens.len() { Some(self.declaration()) } else { None }).collect()
        // let mut program: Vec<Declaration> = Vec::new();

        // while self.i < self.tokens.len() {
        //     program.push(self.declaration());
        // }

        // program
    }

    fn declaration(&mut self) -> DeclarationId {
        match &self.current().value {
            Token::Struct => self.struct_decl(),
            Token::Enum => self.enum_decl(),
            Token::Function => self.function_decl(),
            Token::TypeDef => self.typedef_decl(),
            Token::Import => {
                let imp = self.import_st();
                self.add_decl(Declaration::Statement(imp))
            }
            Token::Static => {
                let var = self.var_decl();
                self.add_decl(Declaration::Statement(var))
            }
            x => {
                self.error(
                    format!("invalid declaration: '{}'", x.get_plaintext()),
                    DECL_FOLLOW
                );
                self.add_decl(Declaration::ParseError)
            }
        }
    }
    fn struct_decl(&mut self) -> DeclarationId {
        self.advance();
        let name = consume!(self, Ident, "expected an identifier".into(), Declaration, DECL_FOLLOW);
        let mut fields: HashMap<String, TypeId> = HashMap::new();

        consume!(self, LBrace, "expected '{'".into(), Declaration, DECL_FOLLOW);
        while let Token::Ident(field) = self.current().value.to_owned() {
            self.advance();
            consume!(self, Colon, "expected ':'".into(), Declaration, DECL_FOLLOW);
            let kind = self.parse_type();
            fields.insert(field.to_owned(), kind);
            consume!(self, Semi, "expected ';'".into(), Declaration, DECL_FOLLOW);
        }
        consume!(self, RBrace, "expected '}'".into(), Declaration, DECL_FOLLOW);

        self.add_decl(Declaration::Struct { name, fields })
    }
    fn enum_decl(&mut self) -> DeclarationId {
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

        self.add_decl(Declaration::Enum { name, modifiers, variants })
    }
    fn function_decl(&mut self) -> DeclarationId {
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
        self.add_decl(Declaration::Function { name, ret, params, body })
    }
    fn typedef_decl(&mut self) -> DeclarationId {
        self.advance();

        let curr = self.current().value.to_owned();
        let new_type = consume!(self, Ident, format!("expected an identifier, recieved {curr:?}"), Declaration, DECL_FOLLOW);
        consume!(self, Equals, "expected '='".into(), Declaration, DECL_FOLLOW);
        let value = self.parse_type();
        consume!(self, Semi, "expected ';'".into(), Declaration, DECL_FOLLOW);

        self.add_decl(Declaration::TypeDef { name: new_type, kind: value })
    }
    fn block(&mut self) -> StatementId {
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

        self.add_stmt(Statement::Block(statements))
    }
    
    fn statement(&mut self) -> StatementId {
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
                self.add_stmt(Statement::Expression(expr))
            }
        }
    }
    fn import_st(&mut self) -> StatementId {
        self.advance();

        let mut path = consume!(self, Ident, "expected an identifier".into(), Statement, STMT_FOLLOW);
        while self.current().get_kind() == TokenKind::Dot {
            consume!(self, Dot, "expected '.'".into(), Statement, STMT_FOLLOW);
            let subdir = consume!(self, Ident, "expected an identifier".into(), Statement, STMT_FOLLOW);

            path.push('/');
            path.push_str(&subdir);
        }

        consume!(self, Semi, "expected ';'".into(), Statement, STMT_FOLLOW);

        self.add_stmt(Statement::Import(path))
    }
    fn return_st(&mut self) -> StatementId {
        self.advance();
        if self.current().get_kind() == TokenKind::Semi {
            self.advance();
            self.add_stmt(Statement::Return(None))
        } else {
            let value = self.expression();
            consume!(self, Semi, "expected ';'".into(), Statement, STMT_FOLLOW);
            self.add_stmt(Statement::Return(Some(value)))
        }
    }
    fn var_decl(&mut self) -> StatementId {
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

        self.add_stmt(Statement::VarDecl { is_static, name, kind, value })
    }
    fn if_st(&mut self) -> StatementId {
        self.advance();

        consume!(self, LParen, "expected '('".into(), Statement, STMT_FOLLOW);
        let condition = self.expression();
        consume!(self, RParen, "expected ')'".into(), Statement, STMT_FOLLOW);

        let body = self.block();

        let alt = if self.current().get_kind() == TokenKind::Else {
            self.advance();
            Some(self.statement())
        } else { None };

        self.add_stmt(Statement::If { condition, body: body, alt })
    }
    fn while_st(&mut self) -> StatementId {
        self.advance();

        consume!(self, LParen, "expected '('".into(), Statement, STMT_FOLLOW);
        let condition = self.expression();
        consume!(self, RParen, "expected ')'".into(), Statement, STMT_FOLLOW);

        let body = self.block();
        self.add_stmt(Statement::While { condition, body })
    }
    fn asm_st(&mut self) -> StatementId {
        self.advance();
        
        let value = consume!(self, String, "expected a string".into(), Statement, STMT_FOLLOW);

        consume!(self, Semi, "expected ';'".into(), Statement, STMT_FOLLOW);
        self.add_stmt(Statement::Asm(value))
    }
    fn for_st(&mut self) -> StatementId {
        self.advance();

        consume!(self, LParen, "expected '('".into(), Statement, STMT_FOLLOW, &[TokenKind::Semi]);
        let init = self.expression();
        consume!(self, Semi, "expected ';'".into(), Statement, STMT_FOLLOW, &[TokenKind::Semi]);

        let condition = self.expression();
        consume!(self, Semi, "expected ';'".into(), Statement, STMT_FOLLOW, &[TokenKind::Semi]);

        let updater = self.expression();
        consume!(self, Semi, "expected ';'".into(), Statement, STMT_FOLLOW);
        consume!(self, RParen, "expected ')'".into(), Statement, STMT_FOLLOW);

        let body_id = self.block();
        if let Statement::ParseError = self.statement_arena[body_id] {
            return self.add_stmt(Statement::ParseError);
        }

        let updater_stmt_id = self.add_stmt(Statement::Expression(updater));
        if let Statement::Block(ref mut xs) = self.statement_arena[body_id] {
            xs.push(updater_stmt_id);
        } else {
            unreachable!();
        }

        let init_stmt_id = self.add_stmt(Statement::Expression(init));
        let while_stmt_id = self.add_stmt(Statement::While { condition, body: body_id });
        self.add_stmt(Statement::Block(vec![ init_stmt_id, while_stmt_id ]))
    }

    fn parse_type(&mut self) -> TypeId {
        match self.current().value.to_owned() {
            Token::LBracket => {
                self.advance();
                let size = if self.current().get_kind() != TokenKind::RBracket {
                    Some(self.expression())
                } else { None };
                consume!(self, RBracket, "expected ']'".into(), Type, TYPE_FOLLOW);
                let kind = self.parse_type();
                self.add_type(Type::Array { size, kind })
            }
            Token::At => {
                self.advance();
                let kind = self.parse_type();
                self.add_type(Type::Pointer { kind })
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
                self.add_type(Type::Function { ret, params })
            }
            Token::PrimType(kind) => {
                self.advance();
                self.add_type(Type::Prim(kind.to_owned()))
            }
            Token::Ident(kind) => {
                self.advance();
                self.add_type(Type::Ident(kind.to_owned()))
            }
            x => {
                self.error(format!("invalid type: '{}'", x.get_plaintext()), TYPE_FOLLOW);
                self.add_error::<Type>()
            }
        }
    }

    fn left_rec(&mut self, symbols: &[TokenKind], child: fn(&mut Parser<'a>) -> ExpressionId) -> ExpressionId {
        let mut lhs = child(self);

        while symbols.contains(&self.current().get_kind()) {
            let op = self.advance();
            let rhs = child(self);
            lhs = self.add_expr(Expression::Binary { lhs, rhs, op: op.value })
        }

        lhs
    }
    fn right_rec(&mut self,
        symbols: &[TokenKind],
        parent: fn(&mut Parser<'a>) -> ExpressionId,
        child: fn(&mut Parser<'a>) -> ExpressionId,
    ) -> ExpressionId {
        let lhs = child(self);

        if symbols.contains(&self.current().get_kind()) {
            let op = self.advance();
            let rhs = parent(self);
            self.add_expr(Expression::Binary { lhs, rhs, op: op.value })
        } else { lhs }
    }
    fn expression(&mut self) -> ExpressionId { self.assignment() }
    fn assignment(&mut self) -> ExpressionId { self.right_rec(&[TokenKind::Equals], Self::assignment, Self::logical_or) }
    fn logical_or(&mut self) -> ExpressionId { self.left_rec(&[TokenKind::LogOr], Self::logical_and) }
    fn logical_and(&mut self) -> ExpressionId { self.left_rec(&[TokenKind::LogAnd], Self::equality) }
    fn equality(&mut self) -> ExpressionId {
        let mut lhs = self.relation();

        if [TokenKind::EqualTo,
            TokenKind::NotEqualTo,
        ].contains(&self.current().get_kind()) {
            let op = self.advance();
            let rhs = self.relation();
            lhs = self.add_expr(Expression::Binary { lhs, rhs, op: op.value });
        }

        lhs
    }
    fn relation(&mut self) -> ExpressionId {
        let mut lhs = self.bitwise_or();

        if [TokenKind::LessThan,
            TokenKind::MoreThan,
            TokenKind::MoreThanEq,
            TokenKind::LessThanEq
        ].contains(&self.current().get_kind()) {
            let op = self.advance();
            let rhs = self.bitwise_or();
            lhs = self.add_expr(Expression::Binary { lhs, rhs, op: op.value });
        }

        lhs
    }
    fn bitwise_or(&mut self) -> ExpressionId { self.left_rec(&[TokenKind::BitOr], Self::bitwise_xor) }
    fn bitwise_xor(&mut self) -> ExpressionId { self.left_rec(&[TokenKind::BitXor], Self::bitwise_and) }
    fn bitwise_and(&mut self) -> ExpressionId { self.left_rec(&[TokenKind::Ampersand], Self::shift) }
    fn shift(&mut self) -> ExpressionId { self.left_rec(&[TokenKind::LShift, TokenKind::RShift], Self::addition) }
    fn addition(&mut self) -> ExpressionId { self.left_rec(&[TokenKind::Plus, TokenKind::Minus], Self::multiplication) }
    fn multiplication(&mut self) -> ExpressionId { self.left_rec(&[TokenKind::Star, TokenKind::Slash, TokenKind::Percent], Self::unary)}
    fn unary(&mut self) -> ExpressionId {
        if [TokenKind::LogNot,
            TokenKind::Minus,
            TokenKind::BitNeg
        ].contains(&self.current().get_kind()) {
            let op = self.advance();
            let child = self.unary();
            self.add_expr(Expression::Unary { child, op: op.value })
        } else { self.reference() }
    }
    fn reference(&mut self) -> ExpressionId {
        if self.current().get_kind() == TokenKind::Ampersand {
            let op = self.advance();
            let child = self.access();
            self.add_expr(Expression::Unary { child, op: op.value })
        } else { self.access() }
    }
    fn access(&mut self) -> ExpressionId {
        if self.current().get_kind() == TokenKind::Cast {
            self.advance();
            consume!(self, LParen, "expected '('".into(), Expression, EXPR_FOLLOW);

            let to = self.parse_type();
            consume!(self, Comma, "expected ','".into(), Expression, EXPR_FOLLOW);

            let value = self.expression();

            consume!(self, RParen, "expected ')'".into(), Expression, EXPR_FOLLOW);
            self.add_expr(Expression::TypeCast { to, value })
        } else if self.current().get_kind() == TokenKind::At {
            let op = self.advance();
            let child = self.primary();
            self.add_expr(Expression::Unary { child, op: op.value })
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

                    lhs = self.add_expr(Expression::FunctionCall { name: lhs, args });
                    true
                }
                Token::LBracket => {
                    let index = self.expression();

                    lhs = self.add_expr(Expression::ArrayAccess { lhs, index });
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
                    let Expression::Identifier(name) = self.expression_arena[lhs].to_owned() else { panic!("expected a struct name before initializer"); };
                    lhs = self.add_expr(Expression::StructInitializer { name, values });
                    true
                }
                Token::Arrow => {
                    let member = consume!(self, Ident, "expected an identifer".into(), Expression, EXPR_FOLLOW);
                    lhs = self.add_expr(Expression::Unary { child: lhs, op: Token::Star });
                    lhs = self.add_expr(Expression::MemberAccess { lhs, member });
                    true
                }
                Token::Dot => {
                    let member = consume!(self, Ident, "expected an identifer".into(), Expression, EXPR_FOLLOW);
                    lhs = self.add_expr(Expression::MemberAccess { lhs, member });
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

    fn primary(&mut self) -> ExpressionId {
        match self.advance().value {
            Token::Ident(x) => self.add_expr(Expression::Identifier(x)),
            Token::Integer(x, k) => self.add_expr(Expression::Integer(x.parse::<i128>().unwrap(), k)),
            Token::String(x) => self.add_expr(Expression::String(x)),
            Token::Char(x) => self.add_expr(Expression::Char(x)),
            Token::LParen => {
                let child = self.expression();
                consume!(self, RParen, "expected ')'".into(), Expression, EXPR_FOLLOW);
                child
            }
            x => {
                self.i -= 1; // go back to fix alignment
                self.error(format!("'{}' is not an expression", x.get_plaintext()), EXPR_FOLLOW);
                self.add_error::<Expression>()
            }
        }
    }
}
