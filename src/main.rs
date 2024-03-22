use compile::compile_file;
use io::{read_file, write_file, SourceFile};

pub mod compile;
pub mod io;
pub mod lexer;
pub mod parser;

fn main() {
    let input_file_path = String::from("example/hello_world.is");
    let input_file = read_file(input_file_path);

    let compiled = compile_file(input_file);

    let output_file = SourceFile {
        path: String::from("example/hello_world.asm"),
        contents: compiled,
    };
    write_file(output_file);
}
