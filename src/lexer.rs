use crate::{diagnostics, parser::TokenKind};

#[derive(Debug, Clone)]
pub struct TokenMeta {
    pub file_id: usize,
    pub pos: (usize, usize), // line, index
    pub value: Token,
}

impl TokenMeta {
    pub fn get_kind(&self) -> TokenKind {
        self.value.get_kind()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
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

    Ident(String),
    String(String),
    Integer(String, String),
    Char(char),

    PrimType(String),
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
    DoubleEquals,
    NotEquals,
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
    pub fn get_plaintext(&self) -> String {
        match self {
            Self::Continue => "continue",
            Self::Function => "fn",
            Self::TypeDef => "typedef",
            Self::Import => "import",
            Self::Return => "return",
            Self::Static => "static",
            Self::Struct => "struct",
            Self::Break => "break",
            Self::Defer => "defer",
            Self::While => "while",
            Self::Cast => "cast",
            Self::Else => "else",
            Self::Enum => "enum",
            Self::Spec => "spec",
            Self::Asm => "asm",
            Self::For => "for",
            Self::Let => "let",
            Self::If => "if",

            Self::Ident(x) => x,
            Self::String(x) => x,
            Self::Integer(x, _) => x,

            Self::PrimType(x) => x,
            Self::FuncType => "Func",

            Self::LBrace => "{",
            Self::RBrace => "}",
            Self::LParen => "(",
            Self::RParen => ")",
            Self::LBracket => "[",
            Self::RBracket => "]",

            Self::Colon => ":",
            Self::Semi => ";",
            Self::Comma => ",",

            Self::Equals => "=",
            Self::LogOr => "||",
            Self::LogAnd => "&&",
            Self::LogNot => "!",
            Self::DoubleEquals => "==",
            Self::NotEquals => "!=",
            Self::LessThan => "<",
            Self::MoreThan => ">",
            Self::LessThanEq => "<=",
            Self::MoreThanEq => ">=",
            Self::BitOr => "|",
            Self::BitXor => "^",
            Self::BitNeg => "~",
            Self::LShift => "<<",
            Self::RShift => ">>",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Star => "*",
            Self::Slash => "/",
            Self::Percent => "%",
            Self::Ampersand => "&",
            Self::At => "@",

            Self::Dot => ".",
            Self::Arrow => "->",
            x => {
                eprintln!("no viable conversion for {x:?}");
                std::process::exit(1);
            }
        }.to_owned()
    }
}

pub struct Lexer<'a> {
    filename: &'a str,
    file_id: usize,
    source: Vec<char>,
    lines: &'a Vec<String>,
    i: usize,
    pos: (usize, usize), // line, index
}

impl<'a> Lexer<'a> {
    pub fn from(filename: &'a str, file_id: usize, lines: &'a Vec<String>, source: &'a String) -> Self {
        Self {
            filename,
            file_id,
            source: source.chars().collect(),
            lines,
            i: 0,
            pos: (0, 0),
        }
    }

    fn get(&self, i: usize) -> Option<char> {
        if i < self.source.len() {
            return Some(self.source[i]);
        }
        None
    }
    fn rel(&self, off: isize) -> Option<char> {
        self.get((self.i as isize + off) as usize)
    }
    fn curr(&self) -> Option<char> {
        self.get(self.i)
    }
    fn advance(&mut self) {
        self.i += 1;
        self.pos.1 += 1;
    }

    pub fn run(&mut self) -> Vec<TokenMeta> {
        let mut tokens = Vec::new();
        while self.i < self.source.len() {
            let Some((value, pos)) = self.next_token() else { break; };

            tokens.push(TokenMeta { file_id: self.file_id, value, pos });
        }
        tokens
    }
    fn next_token(&mut self) -> Option<(Token, (usize, usize))> {
        self.skip_whitespace();

        let pos = self.pos;

        match self.curr() {
            None => None,
            Some(x) => {
                if x.is_numeric() {
                    Some((self.number(), pos))
                } else if x.is_alphabetic() || x == '_' {
                    Some((self.identifier(), pos))
                } else if x == '\"' {
                    Some((self.string(), pos))
                } else if x == '\'' {
                    self.advance();
                    let val = match self.curr().unwrap() {
                        '\\' => {
                            self.advance();
                            match self.curr() {
                                Some('a') => { self.advance(); Token::Char('\x07') }
                                Some('b') => { self.advance(); Token::Char('\x08') }
                                Some('t') => { self.advance(); Token::Char('\t') }
                                Some('n') => { self.advance(); Token::Char('\n') }
                                Some('v') => { self.advance(); Token::Char('\x0b') }
                                Some('f') => { self.advance(); Token::Char('\x0c') }
                                Some('r') => { self.advance(); Token::Char('\r') }
                                Some('e') => { self.advance(); Token::Char('\x1b') }
                                Some('\\') => { self.advance(); Token::Char('\\') }
                                Some('\'') => { self.advance(); Token::Char('\'') }
                                Some('\"') => { self.advance(); Token::Char('\"') }
                                Some('x') => {
                                    let Token::Integer(value, _) = self.number() else {
                                        diagnostics::lexer::hex_non_int(self.filename, self.lines, self.pos);
                                    };
                                    let Ok(value) = u8::from_str_radix(&value, 16) else {
                                        diagnostics::lexer::hex_non_int(self.filename, self.lines, self.pos);
                                    };

                                    Token::Char(value as char)
                                }
                                Some(x) if ('0'..'9').contains(&x) => {
                                    let Token::Integer(value, _) = self.number() else {
                                        diagnostics::lexer::oct_non_int(self.filename, self.lines, self.pos);
                                    };
                                    let Ok(value) = u8::from_str_radix(&value, 8) else {
                                        diagnostics::lexer::oct_non_int(self.filename, self.lines, self.pos);
                                    };

                                    Token::Char(value as char)
                                }
                                _ => diagnostics::lexer::unknown_escape_sequence(self.filename, self.lines, self.pos),
                            }
                        }
                        x => {
                            self.advance();
                            Token::Char(x)
                        }
                    };
                    if self.curr() == Some('\'') {
                        self.advance();
                    } else {
                        diagnostics::lexer::char_length(self.filename, self.lines, self.pos);
                    }
                    Some((val, pos))
                } else if x == '/' && self.rel(1) == Some('/') {
                    self.skip_comments();
                    self.next_token()
                } else {
                    let z = match x {
                        '{' => Token::LBrace,
                        '}' => Token::RBrace,
                        '(' => Token::LParen,
                        ')' => Token::RParen,
                        '[' => Token::LBracket,
                        ']' => Token::RBracket,

                        ':' => Token::Colon,
                        ';' => Token::Semi,
                        ',' => Token::Comma,

                        '=' => if self.rel(1) == Some('=') { self.advance(); Token::DoubleEquals } else { Token::Equals },
                        '|' => if self.rel(1) == Some('|') { self.advance(); Token::LogOr } else { Token::BitOr },
                        '&' => if self.rel(1) == Some('&') { self.advance(); Token::LogAnd } else { Token::Ampersand },
                        '!' => Token::LogNot,
                        '<' => match self.rel(1) {
                            Some('=') => { self.advance(); Token::LessThanEq }
                            Some('<') => { self.advance(); Token::LShift }
                            _ => { Token::LessThan }
                        }
                        '>' => match self.rel(1) {
                            Some('=') => { self.advance(); Token::MoreThanEq }
                            Some('>') => { self.advance(); Token::RShift }
                            _ => { Token::MoreThan }
                        }
                        '^' => Token::BitXor,
                        '~' => Token::BitNeg,
                        '+' => Token::Plus,
                        '-' => if self.rel(1) == Some('>') { self.advance(); Token::Arrow } else { Token::Minus },
                        '*' => Token::Star,
                        '/' => Token::Slash,
                        '%' => Token::Percent,
                        '@' => Token::At,
                        
                        '.' => Token::Dot,
                        _ => diagnostics::lexer::unknown_character(self.filename, self.lines, self.pos),
                    };
                    self.advance();
                    Some((z, pos))
                }
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(x) = self.curr() && x.is_whitespace() {
            self.advance();
            if x == '\n' {
                self.pos.0 += 1;
                self.pos.1 = 0;
            }
        }
    }
    fn skip_comments(&mut self) {
        if self.curr() == Some('/') && self.rel(1) == Some('/') {
            while self.curr() != Some('\n') { self.advance(); }
            self.advance();
            self.pos.0 += 1;
            self.pos.1 = 0;
        }
    }
    fn number(&mut self) -> Token {
        let mut value = "".to_owned();

        while let Some(x) = self.curr() && x.is_numeric() {
            value.push(x);
            self.advance();
        }
        let kind = match self.curr() {
            Some(c) if c == 'i' || c == 'u' => {
                let Some((Token::PrimType(t), (_, _))) = self.next_token() else {
                    diagnostics::lexer::int_follower(self.filename, self.lines, self.pos);
                };
                t
            }
            _ => "i32".into(),
        };

        Token::Integer(value, kind)
    }
    fn string(&mut self) -> Token {
        let mut value = "".to_owned();
        self.advance();

        while let Some(x) = self.curr() && x != '\"' {
            if x == '\n' {
                diagnostics::lexer::unterminated_string(self.filename, self.lines, self.pos);
            }
            value.push(x);
            self.advance();
        }
        self.advance();

        Token::String(value)
    }
    fn identifier(&mut self) -> Token {
        let mut value = self.curr().unwrap().to_string();
        self.advance();

        while let Some(x) = self.curr() && (x.is_alphanumeric() || x == '_') {
            value.push(x);
            self.advance();
        }

        match value.as_str() {
            "continue" => Token::Continue,
            "func" => Token::Function,
            "typedef" => Token::TypeDef,
            "import" => Token::Import,
            "return" => Token::Return,
            "static" => Token::Static,
            "struct" => Token::Struct,
            "break" => Token::Break,
            "defer" => Token::Defer,
            "while" => Token::While,
            "cast" => Token::Cast,
            "else" => Token::Else,
            "enum" => Token::Enum,
            "spec" => Token::Spec,
            "asm" => Token::Asm,
            "for" => Token::For,
            "let" => Token::Let,
            "if" => Token::If,

            prim @ ("void" | "char" | "usize" | "isize"
                | "u64" | "i64" | "u32" | "i32" 
                | "u16" | "i16" | "u8" | "i8") => Token::PrimType(prim.to_owned()),
            "Func" => Token::FuncType,
                
            _ => Token::Ident(value),
        }
    }
}
