use std::process;

use crate::io::SourceFile;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Null,

    Identifier,
    Function,
    If,
    Asm,
    For,
    Let,
    Else,
    Enum,
    Macro,
    While,
    Import,
    Return,
    Static,
    Struct,

    Def,
    TypeDef,

    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    Period,
    Colon,
    SemiColon,
    Comma,
    BlockSeparator,

    Equals,
    NotEqualsTo,
    EqualsTo,
    LessThan,
    LessThanEqualsTo,
    MoreThan,
    MoreThanEqualsTo,
    Increment,
    Decrement,
    Bang,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    DoublePipe,
    DoubleAmpersand,
    Arrow,
    LessThanBang,

    String,
    Integer,
    Character,

    At,
    Ampersand,

    EndOfFile,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

pub struct Tokenizer {
    pub source_file: SourceFile,
    pub source: String,
    pub current_character: char,
    pub index: usize,
}

impl Tokenizer {
    pub fn new(source_file: SourceFile) -> Tokenizer {
        let mut token = Tokenizer {
            index: 0,
            source_file: source_file.clone(),
            source: source_file.clone().contents,
            current_character: '\0',
        };
        if !token.source.is_empty() {
            token.current_character = source_file.contents.as_bytes()[0] as char;
        }
        token
    }
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        if self.source.is_empty() {
            return tokens;
        }

        loop {
            let token = self.get_next_token();
            if token.token_type == TokenType::EndOfFile {
                break;
            }
            tokens.push(token);
        }

        tokens
    }
    fn next(&mut self) {
        self.index += 1;
        if self.index >= self.source.len() {
            self.current_character = '\0';
        } else {
            self.current_character = self.source.as_bytes()[self.index] as char;
        }
    }
    fn skip_whitespace(&mut self) {
        while self.current_character.is_whitespace() {
            self.next();
        }
    }
    fn skip_comment(&mut self) {
        while self.current_character != '\n' {
            self.next();
        }
        self.next();
    }
    fn parse_identifier(&mut self) -> Token {
        let mut token = Token {
            value: String::new(),
            token_type: TokenType::Identifier,
        };
        while self.current_character.is_alphanumeric() || self.current_character == '_' {
            token.value.push(self.current_character);
            self.next();
        }
        token.token_type = match token.value.as_str() {
            "func" => TokenType::Function,
            "if" => TokenType::If,
            "asm" => TokenType::Asm,
            "for" => TokenType::For,
            "let" => TokenType::Let,
            "else" => TokenType::Else,
            "enum" => TokenType::Enum,
            "macro" => TokenType::Macro,
            "while" => TokenType::While,
            "import" => TokenType::Import,
            "return" => TokenType::Return,
            "static" => TokenType::Static,
            "struct" => TokenType::Struct,

            "def" => TokenType::Def,
            "typedef" => TokenType::TypeDef,

            _ => TokenType::Identifier,
        };
        token
    }
    fn parse_string(&mut self) -> Token {
        let mut token = Token {
            value: String::new(),
            token_type: TokenType::String,
        };
        self.next();
        while self.current_character != '\"' {
            token.value.push(self.current_character);
            self.next();
        }
        self.next();
        token
    }
    fn parse_character(&mut self) -> Token {
        let mut token = Token {
            value: String::new(),
            token_type: TokenType::Character,
        };
        self.next();
        if self.current_character == '\\' {
            token.value.push(match self.peek(1) {
                '\\' => '\\',
                'n' => '\n',
                't' => '\t',
                '0' => '\0',
                _ => {
                    eprintln!(
                        "Unknown (or unimplemented) escape character `{}`",
                        self.current_character
                    );
                    process::exit(1);
                }
            });
            self.next();
            self.next();
            // todo!("Implement complex characters (characters prefixed with `\\`)");
        } else {
            token.value.push(self.current_character);
            self.next();
        }
        self.next();
        token
    }
    fn parse_number(&mut self) -> Token {
        let is_hexadecimal = self.current_character == '0' && self.peek(1) == 'x';
        let mut token = Token {
            value: String::new(),
            token_type: TokenType::Integer,
        };
        if is_hexadecimal {
            self.next();
            self.next();
        }
        while self.current_character.is_ascii_digit()
            || (is_hexadecimal && self.current_character.is_ascii_hexdigit())
        {
            token.value.push(self.current_character);
            self.next();
        }
        if is_hexadecimal {
            token.value = i64::from_str_radix(token.value.as_str(), 16)
                .unwrap_or(0)
                .to_string();
        }
        token
    }
    fn skip_and_return(&mut self, token_type: TokenType) -> Token {
        let token: Token = if [
            TokenType::Increment,
            TokenType::Decrement,
            TokenType::EqualsTo,
            TokenType::LessThanEqualsTo,
            TokenType::MoreThanEqualsTo,
            TokenType::BlockSeparator,
            TokenType::Arrow,
            TokenType::LessThanBang,
        ]
        .contains(&token_type)
        {
            let mut name = String::from(self.current_character);
            self.next();
            name.push(self.current_character);
            Token {
                value: name,
                token_type,
            }
        } else {
            Token {
                value: String::from(self.current_character),
                token_type,
            }
        };
        self.next();
        token
    }
    fn peek(&self, offset: usize) -> char {
        return self.source.as_bytes()[self.index + offset] as char;
    }
    fn get_next_token(&mut self) -> Token {
        self.skip_whitespace();
        if self.current_character.is_alphabetic() {
            return self.parse_identifier();
        }
        if self.current_character == '\"' {
            return self.parse_string();
        }
        if self.current_character == '\'' {
            return self.parse_character();
        }
        if self.current_character.is_ascii_digit() {
            return self.parse_number();
        }
        if self.current_character == '/' && self.peek(1) == '/' {
            self.skip_comment();
            return self.get_next_token();
        }

        self.skip_and_return(match self.current_character {
            '(' => TokenType::LeftParenthesis,
            ')' => TokenType::RightParenthesis,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            '[' => TokenType::LeftBracket,
            ']' => TokenType::RightBracket,
            '.' => TokenType::Period,
            ':' => {
                if self.peek(1) == ':' {
                    TokenType::BlockSeparator
                } else {
                    TokenType::Colon
                }
            }
            ';' => TokenType::SemiColon,
            ',' => TokenType::Comma,
            '=' => {
                if self.peek(1) == '=' {
                    TokenType::EqualsTo
                } else {
                    TokenType::Equals
                }
            }
            '<' => {
                if self.peek(1) == '=' {
                    TokenType::LessThanEqualsTo
                } else if self.peek(1) == '!' {
                    TokenType::LessThanBang
                } else {
                    TokenType::LessThan
                }
            }
            '>' => {
                if self.peek(1) == '=' {
                    TokenType::MoreThanEqualsTo
                } else {
                    TokenType::MoreThan
                }
            }
            '+' => {
                if self.peek(1) == '+' {
                    TokenType::Increment
                } else {
                    TokenType::Plus
                }
            }
            '-' => {
                if self.peek(1) == '-' {
                    TokenType::Decrement
                } else if self.peek(1) == '>' {
                    TokenType::Arrow
                } else {
                    TokenType::Minus
                }
            }
            '!' => {
                if self.peek(1) == '=' {
                    TokenType::NotEqualsTo
                } else {
                    TokenType::Bang
                }
            }
            '*' => TokenType::Star,
            '/' => TokenType::Slash,
            '%' => TokenType::Percent,
            '@' => TokenType::At,
            '&' => TokenType::Ampersand,
            _ => {
                if self.index >= self.source.len() {
                    TokenType::EndOfFile
                } else {
                    eprintln!("[Lexer] Unexpected character: {}", self.current_character);
                    process::exit(1);
                }
            }
        })
    }
}
