pub mod symbol_table;
pub mod parser;
pub mod errors;
pub mod lexer;

use crate::{lexer::{Lexer, TokenMeta}, parser::{Declaration, Parser}};

fn main() {
    let filename: &str = "std/stdmem.is";
    let contents: String = std::fs::read_to_string(filename).unwrap();
    let lines: Vec<String> = contents.lines().map(|x| x.to_owned()).collect();

    let mut lexer: Lexer = Lexer::from(&filename, &lines, &contents);
    let tokens: Vec<TokenMeta> = lexer.run();
    // eprintln!("{tokens:#?}");

    let mut parser: Parser = Parser::from(&filename, lines, tokens);
    let ast: Vec<Declaration> = parser.run();

    // eprintln!("{ast:#?}");
}
