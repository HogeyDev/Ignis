#[derive(Debug, Clone)]
pub enum Token {
    Function,
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

pub struct Lexer {
    source: Vec<char>,
    i: usize,
}

impl Lexer {
    pub fn from(contents: String) -> Self {
        Self {
            source: contents.chars().collect(),
            i: 0usize,
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

    pub fn run(&mut self) -> Vec<Token> {
        std::iter::from_fn(|| self.next_token()).collect()
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

                        '=' => if self.rel(1) == Some('=') { self.i += 1; Token::EqualTo } else { Token::Equals },
                        '|' => if self.rel(1) == Some('|') { self.i += 1; Token::LogOr } else { Token::BitOr },
                        '&' => if self.rel(1) == Some('&') { self.i += 1; Token::LogAnd } else { Token::Ampersand },
                        '!' => Token::LogNot,
                        '<' => match self.rel(1) {
                            Some('=') => { self.i += 1; Token::LessThanEq }
                            Some('<') => { self.i += 1; Token::LShift }
                            _ => { Token::LessThan }
                        }
                        '>' => match self.rel(1) {
                            Some('=') => { self.i += 1; Token::MoreThanEq }
                            Some('>') => { self.i += 1; Token::RShift }
                            _ => { Token::MoreThan }
                        }
                        '^' => Token::BitXor,
                        '~' => Token::BitNeg,
                        '+' => Token::Plus,
                        '-' => if self.rel(1) == Some('>') { self.i += 1; Token::Arrow } else { Token::Minus },
                        '*' => Token::Star,
                        '/' => Token::Slash,
                        '%' => Token::Percent,
                        '@' => Token::At,
                        
                        '.' => Token::Dot,
                        y => panic!("What is this: `{y}`"),
                    };
                    self.i += 1;
                    Some(z)
                }
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(x) = self.curr() && x.is_ascii_whitespace() { self.i += 1; }
    }
    fn skip_comments(&mut self) {
        if self.curr() == Some('/') && self.rel(1) == Some('/') {
            while self.curr() != Some('\n') { self.i += 1; }
        }
    }
    fn number(&mut self) -> Token {
        let mut value = "".to_owned();

        while let Some(x) = self.curr() && x.is_numeric() {
            value.push(x);
            self.i += 1;
        }

        Token::Integer(value)
    }
    fn string(&mut self) -> Token {
        let mut value = "".to_owned();
        self.i += 1;

        while let Some(x) = self.curr() && !['\n', '\"'].contains(&x) {
            value.push(x);
            self.i += 1;
        }
        self.i += 1;

        Token::String(value)
    }
    fn identifier(&mut self) -> Token {
        let mut value = self.curr().unwrap().to_string();
        self.i += 1;

        while let Some(x) = self.curr() && (x.is_alphanumeric() || x == '_') {
            value.push(x);
            self.i += 1;
        }

        match value.as_str() {
            "func" => Token::Function,
            "typedef" => Token::TypeDef,
            "import" => Token::Import,
            "return" => Token::Return,
            "struct" => Token::Struct,
            "while" => Token::While,
            "else" => Token::Else,
            "enum" => Token::Enum,
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
