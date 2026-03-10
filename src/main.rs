pub mod symbol_table;
pub mod diagnostics;
pub mod parser;
pub mod lexer;

use crate::{lexer::{Lexer, TokenMeta}, parser::{Declaration, Parser}};

fn main() {
    let filename: &str = "example/preprocessing.is";
    let contents: String = std::fs::read_to_string(filename).unwrap();
    let lines: Vec<String> = contents.lines().map(|x| x.to_owned()).collect();

    let mut lexer: Lexer = Lexer::from(&filename, &lines, &contents);
    let tokens: Vec<TokenMeta> = lexer.run();

    let mut parser: Parser = Parser::from(&filename, lines, tokens);
    let ast: Vec<Declaration> = parser.run();

    if parser.err_count > 0 {
        eprintln!("\x1b[0;31merror\x1b[0;0m: ignis compiler failed with {} errors", parser.err_count);
        std::process::exit(1);
    }
}
