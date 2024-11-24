use io::{read_file, write_file, SourceFile};
use std::process::{self, Command};
use codegen::compile_to_asm;
use scope::ScopeContext;
use compile::parse_file;
use config::get_config;
use cli::CliParser;

pub mod preprocessor;
pub mod modulizer;
pub mod codegen;
pub mod compile;
pub mod config;
pub mod parser;
pub mod lexer;
pub mod scope;
pub mod types;
pub mod util;
pub mod asm;
pub mod cli;
pub mod io;

fn main() {
    let cli_parser = CliParser::from(std::env::args().collect());
    if cli_parser.arguments.len() == 0 || cli_parser.arguments.len() > 1 {
        let reason = if cli_parser.arguments.len() > 1 { "More than one" } else { "No" };
        eprintln!("Error: {} main file found\n\tUsage: {} main.is -o output", reason, cli_parser.args.first().unwrap());
        process::exit(1);
    }
    let input_file_path = String::from(cli_parser.arguments.first().unwrap());
    let input_file = read_file(input_file_path.clone());
    let mut program_config = get_config(input_file_path, &cli_parser);

    let mut scope = ScopeContext::new();
    let parsed_input_file = parse_file(&program_config, input_file);
    let section_text = compile_to_asm(&mut program_config, parsed_input_file, &mut scope);
    let section_data = scope.compile_data();
    
    let compiled = format!(
        "section .text\n{}\nglobal _start\n_start:\n\tpush rbp\n\tmov rbp, rsp\n\tcall _main\n\tmov rdi, rax\n\tmov rax, 60\n\tsyscall\nsection .data\n{}",
        section_text, section_data
    );

    let output_file_path = cli_parser.option_value("o", "a.out");
    let output_asm_file = SourceFile {
        path: format!("{}{}", output_file_path, ".asm"),
        contents: compiled.clone(),
    };
    write_file(output_asm_file);

    let nasm_status = Command::new("nasm")
        .arg("-f").arg("elf64").arg(format!("{}.asm", output_file_path)).arg(format!("-o {}.o", output_file_path)).arg("-g")
        .output().unwrap();
    let ld_status = Command::new("ld")
        .arg("-m").arg("elf_x86_64").arg(format!("{}.o", output_file_path)).arg("-o").arg(output_file_path.clone())
        .output().unwrap();
    let rm_status = Command::new("rm")
        .arg(format!("{}.o", output_file_path))
        .output().unwrap();

    if !nasm_status.status.success() { eprint!("{}", String::from_utf8_lossy(&nasm_status.stderr)); process::exit(1); }
    if !ld_status.status.success() { eprint!("{}", String::from_utf8_lossy(&ld_status.stderr)); process::exit(1); }
    if !rm_status.status.success() { eprint!("{}", String::from_utf8_lossy(&rm_status.stderr)); process::exit(1); }

    if !program_config.debug_asm {
        Command::new("rm")
            .arg(format!("{}.asm", output_file_path))
            .output().unwrap();
    }
}
