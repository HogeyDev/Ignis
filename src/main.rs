use std::borrow::BorrowMut;

use backend::compile_to_asm;
use compile::parse_file;
use config::get_config;
use io::{read_file, write_file, SourceFile};
use scope::ScopeContext;

pub mod backend;
pub mod compile;
pub mod config;
pub mod io;
pub mod lexer;
pub mod parser;
pub mod scope;

fn main() {
    let input_file_path = String::from("example/hello_world.is");
    let input_file = read_file(input_file_path);
    let program_config = get_config();

    let mut scope = &mut ScopeContext::new();
    let section_text = compile_to_asm(
        program_config.clone(),
        parse_file(program_config, input_file),
        scope,
    );
    let section_data = scope.compile_strings();

    let compiled = format!("{}\n{}", section_text, section_data);

    let output_file = SourceFile {
        path: String::from("example/hello_world.asm"),
        contents: compiled.clone(),
    };
    write_file(output_file);
    println!("{}", compiled)
}
