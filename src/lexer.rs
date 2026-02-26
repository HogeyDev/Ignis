use crate::{errors, parser::TokenKind};

#[derive(Debug, Clone)]
pub struct TokenMeta {
    pub pos: (usize, usize), // line, index
    pub value: Token,
}

impl TokenMeta {
    pub fn get_kind(&self) -> TokenKind {
        self.value.get_kind()
    }
}

#[derive(Debug, Clone)]
pub enum Token {
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

    Ident(String),
    String(String),
    Integer(String),

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

pub struct Lexer<'a> {
    filename: &'a str,
    source: Vec<char>,
    lines: &'a Vec<String>,
    i: usize,
    pos: (usize, usize), // line, index
}

impl<'a> Lexer<'a> {
    pub fn from(filename: &'a str, lines: &'a Vec<String>, source: &'a String) -> Self {
        Self {
            filename,
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
            let pos = self.pos;
            let Some(token) = self.next_token() else { break; };

            tokens.push(TokenMeta { value: token, pos });
        }
        tokens
    }
    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        self.skip_comments();

        match self.curr() {
            None => None,
            Some(x) => {
                if x.is_numeric() {
                    Some(self.number())
                } else if x.is_alphabetic() || x == '_' {
                    Some(self.identifier())
                } else if x == '\"' {
                    Some(self.string())
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

                        '=' => if self.rel(1) == Some('=') { self.advance(); Token::EqualTo } else { Token::Equals },
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
                        _ => errors::lexer::unknown_character(self.filename, self.lines, self.pos),
                    };
                    self.advance();
                    Some(z)
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

        Token::Integer(value)
    }
    fn string(&mut self) -> Token {
        let mut value = "".to_owned();
        self.advance();

        while let Some(x) = self.curr() && x != '\"' {
            if x == '\n' {
                errors::lexer::unterminated_string(self.filename, self.lines, self.pos);
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
            "func" => Token::Function,
            "typedef" => Token::TypeDef,
            "import" => Token::Import,
            "return" => Token::Return,
            "struct" => Token::Struct,
            "while" => Token::While,
            "cast" => Token::Cast,
            "else" => Token::Else,
            "enum" => Token::Enum,
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
