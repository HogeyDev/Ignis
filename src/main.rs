pub mod diagnostics;
pub mod analyzer;
pub mod config;
pub mod import;
pub mod parser;
pub mod lexer;
pub mod cli;
pub mod ir;

use crate::{analyzer::Analyzer, cli::CliParser, config::Configuration, lexer::{Lexer, TokenMeta}, parser::Parser, ir::IrBuilder};

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

    let mut config = Configuration::new();
    config.import_path_priority.insert(0, "./std/".to_owned()); // this should be the default stdlib path
    if let Some(stdlib) = cli_parser.option_value("stdlib") {
        config.import_path_priority.push(stdlib);
    }

    let mut inc_files = Vec::new();
    let mut lexer: Lexer = Lexer::from(&filename, 0, &lines, &contents);
    let tokens: Vec<TokenMeta> = lexer.run();

    let mut parser: Parser = Parser::from(&config, &filename, lines, tokens, &mut inc_files);
    let root = parser.run();
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

    let mut ir_builder: IrBuilder = IrBuilder::new(&analyzer);
    ir_builder.run(&root);
}
