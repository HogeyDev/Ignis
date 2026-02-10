pub mod parser;
pub mod lexer;

use crate::{lexer::{Lexer, Token}, parser::{Declaration, Parser}};

fn main() {
    let contents: String = std::fs::read_to_string("example/boolean.is").unwrap();

    let mut lexer: Lexer = Lexer::from(contents);
    let tokens: Vec<Token> = lexer.run();

    let mut parser: Parser = Parser::from(tokens);
    let ast: Vec<Declaration> = parser.run();

    eprintln!("{ast:?}");
}
