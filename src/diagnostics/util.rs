use crate::lexer::{Token, TokenMeta};

pub fn token_width(token: TokenMeta) -> usize {
    match token.value {
        Token::Continue
            => 8,

        Token::TypeDef
            => 7,

        Token::Import | 
        Token::Return | 
        Token::Static |
        Token::Struct
            => 6,

        Token::Break |
        Token::Defer |
        Token::While
            => 5,

        Token::Cast | 
        Token::Else | 
        Token::Enum |
        Token::Spec
            => 4,

        Token::Asm | 
        Token::For | 
        Token::Let
            => 3,

        Token::Arrow | 
        Token::DoubleEquals | 
        Token::FuncType | 
        Token::Function | 
        Token::If | 
        Token::LShift | 
        Token::LessThanEq | 
        Token::LogAnd | 
        Token::LogOr | 
        Token::MoreThanEq | 
        Token::NotEquals | 
        Token::RShift
            => 2,
        
        Token::Ampersand | 
        Token::At | 
        Token::BitNeg | 
        Token::BitOr | 
        Token::BitXor | 
        Token::Colon | 
        Token::Comma | 
        Token::Dot | 
        Token::Equals | 
        Token::LBrace | 
        Token::LBracket | 
        Token::LParen | 
        Token::LessThan | 
        Token::LogNot | 
        Token::Minus | 
        Token::MoreThan | 
        Token::Percent | 
        Token::Plus | 
        Token::RBrace | 
        Token::RBracket | 
        Token::RParen | 
        Token::Semi | 
        Token::Slash | 
        Token::Star |
        Token::Char(_)
            => 1,

        Token::Ident(x) | 
        Token::Integer(x, _) | 
        Token::PrimType(x) | 
        Token::String(x)
            => x.len(),
    }
}

pub fn error_align_caret(line: &String, pos: usize) -> (String, usize) {
    let mut fixed = String::new();
    let mut off = 0;

    for (i, c) in line.chars().enumerate() {
        if c == '\t' {
            fixed.push_str("    ");
            if i < pos { off += 4; }
        } else {
            fixed.push(c);
            if i < pos { off += 1; }
        }
    }

    (fixed, off)
}

pub fn print_error_header<'a>(filename: &'a str, pos: (usize, usize), msg: String) {
    eprintln!("\x1b[0;31merror\x1b[0;0m: {filename}:{}:{}: {msg}", pos.0+1, pos.1+1);
}
