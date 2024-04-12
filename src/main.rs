use backend::compile_to_asm;
use compile::parse_file;
use config::get_config;
use io::{read_file, write_file, SourceFile};
use scope::ScopeContext;

use crate::config::Configuration;

pub mod backend;
pub mod compile;
pub mod config;
pub mod io;
pub mod lexer;
pub mod parser;
pub mod scope;
pub mod util;

fn main() {
    let input_file_path = String::from("example/hello_world.is");
    let input_file = read_file(input_file_path);
    let program_config = get_config();

    let mut scope = ScopeContext::new();
    let section_text = compile_to_asm(
        program_config.clone(),
        parse_file(program_config, input_file),
        &mut scope,
    );
    let section_data = scope.compile_strings();

    let compiled = format!(
        "section .text\n{}\nglobal _start\n_start:\n\tpush rbp\n\tmov rbp, rsp\n\tcall main\n\tmov rax, 60\n\tmov rdi, 0\n\tsyscall\nsection .data\n{}",
        section_text, section_data
    );

    let output_file = SourceFile {
        path: String::from("example/hello_world.asm"),
        contents: compiled.clone(),
    };
    write_file(output_file);
    // println!("{}", compiled)
    // println!(
    //     "AST:\n{:#?}",
    //     parse_file(
    //         Configuration::new(),
    //         read_file("example/stdio.is".to_string())
    //     )
    // );
}
