use std::process;

use crate::{
    compile::parse_file,
    config::Configuration,
    io::read_file,
    parser::{Operation, AST},
    scope::ScopeContext,
};

pub fn compile_to_asm(
    program_config: Configuration,
    root: Box<AST>,
    scope: &mut ScopeContext,
) -> String {
    match *root {
        AST::Block(statements) => {
            let mut asm = String::new();

            let block_scope = &mut scope.sub_scope();
            for statement in statements {
                asm.push_str(
                    compile_to_asm(program_config.clone(), statement, block_scope).as_str(),
                );
            }
            scope.absorb_strings(block_scope.to_owned());
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
            body,
            ..
        } => {
            let mut asm = String::new();

            asm.push_str(format!("global {}\n{}:\n", name, name).as_str());

            let body_scope = &mut scope.sub_scope();

            body_scope.stack_size += 1; // return address pushed after arguments on call

            for param in prototype.iter().rev().cloned() {
                asm.push_str(compile_to_asm(program_config.clone(), param, body_scope).as_str());
            }

            asm.push_str(&compile_to_asm(program_config.clone(), body, body_scope).as_str());

            scope.absorb_strings(body_scope.to_owned());

            asm
        }
        AST::Parameter { param_type, name } => {
            scope.add_variable(name, param_type);
            String::new()
        }
        AST::If { condition, body } => {
            let mut asm = String::new();

            asm.push_str(compile_to_asm(program_config.clone(), condition, scope).as_str());
            asm.push_str(compile_to_asm(program_config, body, scope).as_str());

            asm
        }
        AST::BinaryExpression { op, lhs, rhs } => {
            let mut asm = String::new();

            asm.push_str(compile_to_asm(program_config.clone(), lhs, scope).as_str());
            asm.push_str(compile_to_asm(program_config, rhs, scope).as_str());
            asm.push_str(scope.pop(String::from("rbx")).as_str());
            asm.push_str(scope.pop(String::from("rax")).as_str());

            asm.push_str(match op {
                Operation::LTE => "",
                _ => {
                    eprintln!("Unimplemented operation: {:?}", op);
                    process::exit(1);
                }
            });

            asm
        }
        AST::VariableCall { name } => {
            let mut asm = String::new();

            let offset = scope.get_variable(name).1;
            asm.push_str(scope.push(format!("QWORD [rsp+{}]\n", 8 * offset)).as_str());

            asm
        }
        AST::Integer(value) => scope.push(format!("{}", value)),
        AST::Return(value) => {
            let mut asm = String::new();

            asm.push_str(compile_to_asm(program_config, value, scope).as_str());
            asm.push_str(scope.pop(String::from("rax")).as_str());
            asm.push_str("\tret\n");

            asm
        }
        AST::VariableDeclaration {
            variable_type,
            name,
        } => {
            let mut asm = String::new();

            scope.add_variable(name, variable_type);
            asm.push_str("\tpush 0\n");

            asm
        }
        AST::VariableAssignment { name, value } => {
            let mut asm = String::new();

            asm.push_str(compile_to_asm(program_config, value, scope).as_str());
            scope.pop(String::from("rax"));

            let offset = scope.get_variable(name).1;
            asm.push_str(format!("\tmov QWORD [rsp+{}], rax\n", 8 * offset).as_str());

            asm
        }
        AST::Argument(value) => compile_to_asm(program_config, value, scope).to_string(),
        AST::For {
            initializer,
            condition,
            updater,
            body,
        } => {
            let mut asm = String::new();

            asm
        }
        AST::String(value) => {
            let id = scope.add_string(value);
            format!("\tpush .STR{}\n", id)
        }
        _ => {
            eprintln!("Could not find a way to compile {:?} to assembly", root);
            process::exit(1);
        }
    }
}
