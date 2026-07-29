pub mod diagnostics;
pub mod analyzer;
pub mod parser;
pub mod lexer;
pub mod ssa;
pub mod cli;

use crate::{analyzer::Analyzer, cli::CliParser, lexer::{Lexer, TokenMeta}, parser::{Parser, RootAst}};

fn main() {
    let cli_parser: CliParser = CliParser::from(std::env::args().collect());
    if cli_parser.arguments.len() == 0 || cli_parser.arguments.len() > 1 {
        let reason = if cli_parser.arguments.len() > 1 { "More than one" } else { "No" };
        eprintln!("Error: {} main file found\n\tUsage: {} main.is -o output", reason, cli_parser.args.first().unwrap());
        std::process::exit(1);
    }
    let filename: &str = &cli_parser.arguments[0];
    let contents: String = std::fs::read_to_string(filename).unwrap();
    let lines: Vec<String> = contents.lines().map(|x| x.to_owned()).collect();

    let mut lexer: Lexer = Lexer::from(&filename, &lines, &contents);
    let tokens: Vec<TokenMeta> = lexer.run();

    let mut parser: Parser = Parser::from(&filename, lines, tokens);
    let root: RootAst = parser.run();
    if parser.err_count > 0 {
        eprintln!("\x1b[0;31merror\x1b[0;0m: ignis compiler failed with {} errors", parser.err_count);
        std::process::exit(1);
    }

    let mut analyzer = Analyzer::from(parser);
    analyzer.run(&root);
    if analyzer.err_count > 0 {
        eprintln!("\x1b[0;31merror\x1b[0;0m: ignis compiler failed with {} errors", analyzer.err_count);
        std::process::exit(1);
    }
}
