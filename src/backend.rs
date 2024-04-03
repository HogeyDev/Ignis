use std::process;

use crate::{
    compile::parse_file, config::Configuration, io::read_file, parser::AST, scope::ScopeContext,
};

pub fn compile_to_asm(
    program_config: Configuration,
    root: Box<AST>,
    scope: &mut ScopeContext,
) -> String {
    match *root {
        AST::Block(statements) => {
            let mut asm = String::new();
            for statement in statements {
                asm.push_str(compile_to_asm(program_config.clone(), statement, scope).as_str());
            }
            asm
        }
        AST::Import { module } => {
            let path_with_ending = module.replace('.', "/") + ".is";
            let full_path = program_config.root_path.clone() + "/" + path_with_ending.as_str();
            let file = read_file(full_path);
            compile_to_asm(
                program_config.clone(),
                parse_file(program_config, file),
                scope,
            )
        }
        AST::FunctionCall { name, arguments } => {
            // check if function exists
            // if scope

            let mut asm = String::new();

            for arg in arguments.iter().rev().cloned() {
                asm.push_str(compile_to_asm(program_config.clone(), arg, scope).as_str());
            }
            asm.push_str(format!("\tcall {}", name).as_str());
            asm
        }
        AST::FunctionDeclaration {
            name,
            prototype,
            // body,
            ..
        } => {
            let mut asm = String::new();

            asm.push_str(format!("global {}\n{}:\n", name, name).as_str());

            for parameter in prototype {
                asm.push_str(compile_to_asm(program_config.clone(), parameter, scope).as_str());
            }

            asm
        }
        AST::Parameter { param_type, name } => {
            let offset = scope.add_variable(name, param_type);
            scope.push(String::from("rax"));
            String::from("")
        }
        _ => {
            eprintln!("Could not find a way to compile {:?} to assembly", root);
            process::exit(1);
        }
    }
}
