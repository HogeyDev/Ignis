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
    let compiled = compile_to_asm(
        program_config.clone(),
        parse_file(program_config, input_file),
        ScopeContext::new().borrow_mut(),
    );

    let output_file = SourceFile {
        path: String::from("example/hello_world.asm"),
        contents: compiled.clone(),
    };
    write_file(output_file);
    println!("{}", compiled)
}
