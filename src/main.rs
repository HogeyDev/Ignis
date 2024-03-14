use compile::compile_file;
use io::{read_file, write_file, SourceFile};

pub mod compile;
pub mod io;
pub mod lexer;

fn main() {
    let input_file_path = "example/hello_world.is".to_string();
    let input_file = read_file(input_file_path);

    let compiled = compile_file(input_file);

    let output_file = SourceFile {
        path: "example/hello_world.asm".to_string(),
        contents: compiled,
    };
    write_file(output_file);
}
