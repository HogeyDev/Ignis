use crate::io::SourceFile;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Identifier,
    Function,
    If,
    Let,
    Asm,
    For,
    While,
    Return,

    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,

    Colon,
    SemiColon,
    Comma,

    Equals,

    EqualsTo,
    LessThan,
    LessThanEqualsTo,
    MoreThan,
    MoreThanEqualsTo,
    Increment,
    Decrement,
    Plus,
    Minus,
    Star,
    Slash,

    String,
    Integer,

    EndOfFile,
}

#[derive(Debug)]
pub struct Token {
    pub variation: TokenType,
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
        return Tokenizer {
            index: 0,
            source_file: source_file.clone(),
            source: source_file.clone().contents,
            current_character: source_file.contents.as_bytes()[0] as char,
        };
    }
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            let token = self.get_next_token();
            if token.variation == TokenType::EndOfFile {
                break;
            }
            tokens.push(token);
        }

        return tokens;
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
    fn parse_identifier(&mut self) -> Token {
        let mut token = Token {
            value: "".to_string(),
            variation: TokenType::Identifier,
        };
        while self.current_character.is_alphanumeric() {
            token.value.push(self.current_character);
            self.next();
        }
        return token;
    }
    fn parse_string(&mut self) -> Token {
        let mut token = Token {
            value: "".to_string(),
            variation: TokenType::String,
        };
        self.next();
        while self.current_character != '\"' {
            token.value.push(self.current_character);
            self.next();
        }
        self.next();
        return token;
    }
    fn parse_number(&mut self) -> Token {
        let mut token = Token {
            value: "".to_string(),
            variation: TokenType::String,
        };
        self.next();
        while self.current_character.is_digit(10) {
            token.value.push(self.current_character);
            self.next();
        }
        self.next();
        return token;
    }
    fn skip_and_return(&mut self, token_type: TokenType) -> Token {
        self.next();
        return Token {
            value: "".to_string(),
            variation: token_type,
        };
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
        if self.current_character.is_digit(10) {
            return self.parse_number();
        }

        return self.skip_and_return(match self.current_character {
            '(' => TokenType::LeftParenthesis,
            ')' => TokenType::RightParenthesis,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ':' => TokenType::Colon,
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
                } else {
                    TokenType::Minus
                }
            }
            '*' => TokenType::Star,
            '/' => TokenType::Slash,
            _ => TokenType::EndOfFile,
        });
    }
}
