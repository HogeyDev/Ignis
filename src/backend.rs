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

            let mut block_scope = scope.sub_scope();

            asm.push_str(block_scope.push("rbp".to_string()).as_str());
            asm.push_str("\tmov rbp, rsp\n");

            for statement in statements {
                asm.push_str(
                    compile_to_asm(program_config.clone(), statement, &mut block_scope).as_str(),
                );
            }

            asm.push_str("\tmov rsp, rbp\n");
            asm.push_str(block_scope.pop("rbp".to_string()).as_str());

            scope.absorb_sub_scope_globals(block_scope.to_owned());
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
            let mut asm = String::new();

            let function_data = scope.get_function_data(name.clone()); // also checks if function
                                                                       // exists
            for arg in arguments.iter().rev().cloned() {
                asm.push_str(compile_to_asm(program_config.clone(), arg, scope).as_str());
            }
            asm.push_str(format!("\tcall {}\n\tadd rsp, {}\n", name, 8 * arguments.len()).as_str());
            if function_data.0 != "void" {
                // has a notable return value
                asm.push_str(scope.push("rax".to_string()).as_str());
            }
            asm
        }
        AST::FunctionDeclaration {
            name,
            prototype,
            body,
            return_type,
        } => {
            let mut asm = String::new();

            asm.push_str(format!("global {}\n{}:\n", name, name).as_str());

            let mut body_scope = scope.sub_scope();

            let mut params = Vec::new();
            for param in prototype.iter().rev().cloned() {
                asm.push_str(
                    compile_to_asm(program_config.clone(), param.clone(), &mut body_scope).as_str(),
                );
                match *param {
                    AST::Parameter { param_type, name } => {
                        params.push((name, param_type));
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
            body_scope.stack_size += 1; // return address pushed after arguments on call

            scope.add_function(name, return_type, params);

            asm.push_str(compile_to_asm(program_config.clone(), body, &mut body_scope).as_str());
            asm.push_str("\tret\n");

            scope.absorb_sub_scope_globals(body_scope);

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
            asm.push_str(scope.pop(String::from("rbx")).as_str()); // rhs
            asm.push_str(scope.pop(String::from("rax")).as_str()); // lhs

            asm.push_str(match op {
                Operation::Add => "\tadd rax, rbx\n",
                // Operation::Sub => "",
                // Operation::Mul => "",
                // Operation::Div => "",
                // Operation::Mod => "",
                // Operation::Inc => "",
                // Operation::Dec => "",
                // Operation::Inv => "",
                // Operation::Neg => "",
                // Operation::Or => "",
                // Operation::And => "",
                // Operation::Eq => "",
                // Operation::Neq => "",
                Operation::LT => "\tcmp rax, rbx\n\tsetl al\n\tmovzx rax, al\n",
                // Operation::GT => "",
                // Operation::LTE => "",
                // Operation::GTE => "",
                Operation::ArrAcc => "\tmov rax, [rax + rbx]\n",
                _ => {
                    eprintln!("Unimplemented binary operation: {:?}", op);
                    process::exit(1);
                }
            });

            asm.push_str(scope.push("rax".to_string()).as_str());

            asm
        }
        AST::VariableCall { name } => {
            let mut asm = String::new();

            let offset = scope.get_variable_offset(name.clone());
            asm.push_str(format!("\tmov rax, qword [rsp+{}]\n", 8 * offset).as_str());
            asm.push_str(scope.push(format!("rax ; recalled `{}`", name)).as_str());

            asm
        }
        AST::Integer(value) => scope.push(format!("{} ; integer literal", value)),
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

            asm.push_str(scope.push(format!("0 ; created `{}`", name)).as_str());
            scope.add_variable(name.clone(), variable_type);

            asm
        }
        AST::VariableAssignment { name, value } => {
            let mut asm = String::new();

            asm.push_str(compile_to_asm(program_config, value, scope).as_str());
            asm.push_str(scope.pop(String::from("rax")).as_str());

            let offset = scope.get_variable_offset(name.clone());
            asm.push_str(
                format!(
                    "\tmov qword [rsp+{}], rax ; assigned `{}`\n",
                    8 * offset,
                    name
                )
                .as_str(),
            );

            asm
        }
        AST::Argument(value) => compile_to_asm(program_config, value, scope).to_string(),
        // AST::For {
        //     initializer,
        //     condition,
        //     updater,
        //     body,
        // } => {
        //     let mut asm = String::new();
        //
        //     asm
        // }
        AST::While { condition, body } => {
            let mut asm = String::new();

            let head_label_id = scope.add_label();
            let tail_label_id = scope.add_label();

            asm.push_str(format!("lbl{}:\n", head_label_id).as_str());
            asm.push_str(compile_to_asm(program_config.clone(), condition, scope).as_str());
            asm.push_str(scope.pop("rax".to_string()).as_str());
            asm.push_str(format!("\tcmp rax, 0\n\tje lbl{}\n", tail_label_id).as_str());

            asm.push_str(compile_to_asm(program_config, body, scope).as_str());
            asm.push_str(format!("\tjmp lbl{}\n", head_label_id).as_str());
            asm.push_str(format!("lbl{}:\n", tail_label_id).as_str());

            asm
        }
        AST::Asm(assembly) => {
            let mut asm = assembly.clone();
            asm.push('\n');
            asm
        }
        AST::String(value) => {
            let id = scope.add_string(value);
            format!("\tpush STR{}\n", id)
        }
        _ => {
            eprintln!("Could not find a way to compile {:?} to assembly", root);
            process::exit(1);
        }
    }
}
