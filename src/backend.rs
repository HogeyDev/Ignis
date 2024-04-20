use std::process;

use crate::{
    compile::parse_file,
    config::Configuration,
    io::read_file,
    parser::{Operation, AST},
    scope::ScopeContext,
    types::calculate_expression_type,
    util::get_asm_size_prefix,
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

            for statement in statements {
                asm.push_str(
                    compile_to_asm(program_config.clone(), statement, &mut block_scope).as_str(),
                );
            }

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
            asm.push_str(
                format!("\tcall _{}\n\tadd rsp, {}\n", name, 8 * arguments.len()).as_str(),
            );
            if function_data.0 != "void" {
                // has a notable return value
                asm.push_str(scope.push("rax".to_string(), 8).as_str());
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

            asm.push_str(format!("global _{}\n_{}:\n", name, name).as_str());

            let mut body_scope = scope.sub_scope();

            body_scope.add_parameter("".to_string(), "usize *".to_string(), 8);

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
            // body_scope.stack_size += 8; // return address pushed after arguments on call

            asm.push_str("\tpush rbp\n\tmov rbp, rsp\n");

            scope.add_function(name, return_type, params.clone());

            asm.push_str(compile_to_asm(program_config.clone(), body, &mut body_scope).as_str());
            asm.push_str("\tmov rsp, rbp\n\tpop rbp\n\tret\n");

            scope.absorb_sub_scope_globals(body_scope);

            asm
        }
        AST::Parameter { param_type, name } => {
            scope.add_parameter(name, param_type, 8);
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

            asm.push_str(compile_to_asm(program_config.clone(), lhs.clone(), scope).as_str());
            asm.push_str(compile_to_asm(program_config, rhs.clone(), scope).as_str());
            asm.push_str(scope.pop(String::from("rbx"), 8).as_str()); // rhs
            asm.push_str(scope.pop(String::from("rax"), 8).as_str()); // lhs
            let lhs_typing = calculate_expression_type(lhs, scope).unwrap();
            let _rhs_typing = calculate_expression_type(rhs, scope).unwrap();

            asm.push_str(
                match op {
                    Operation::Add => "\tadd rax, rbx\n".to_string(),
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
                    Operation::LT => "\tcmp rax, rbx\n\tsetl al\n\tmovzx rax, al\n".to_string(),
                    // Operation::GT => "",
                    // Operation::LTE => "",
                    // Operation::GTE => "",
                    Operation::ArrAcc => {
                        if lhs_typing.1 == 4 {
                            format!(
                                "\timul rbx, {}\n\txor ecx, ecx\n\tmov ecx, dword [rax + rbx]\n",
                                lhs_typing.1
                            )
                        } else if lhs_typing.1 == 8 {
                            format!(
                                "\timul rbx, {}\n\tmov rax, qword [rax + rbx]\n",
                                lhs_typing.1,
                            )
                        } else {
                            format!(
                                "\timul rbx, {}\n\tmovzx rax, {} [rax + rbx]\n",
                                lhs_typing.1,
                                get_asm_size_prefix(lhs_typing.1.try_into().unwrap_or(0))
                            )
                        }
                    }
                    _ => {
                        eprintln!("Unimplemented binary operation: {:?}", op);
                        process::exit(1);
                    }
                }
                .as_str(),
            );

            asm.push_str(scope.push("rax".to_string(), 8).as_str());

            asm
        }
        AST::VariableCall { name } => {
            let mut asm = String::new();

            let offset = scope.get_variable_offset(name.clone()).1;
            asm.push_str(format!("\tmov rax, qword [rbp{}]\n", offset).as_str());
            asm.push_str(scope.push(format!("rax ; recalled `{}`", name), 8).as_str());

            asm
        }
        AST::Integer(value) => {
            let mut asm = String::new();
            asm.push_str(format!("\tmov rdx, {}\n", value).as_str());
            asm.push_str(scope.push("rdx".to_string(), 8).as_str());
            asm
        }
        AST::Return(value) => {
            let mut asm = String::new();

            asm.push_str(compile_to_asm(program_config, value, scope).as_str());
            asm.push_str(scope.pop(String::from("rax"), 8).as_str());
            asm.push_str("\tret\n");

            asm
        }
        AST::VariableDeclaration {
            variable_type,
            name,
        } => {
            let mut asm = String::new();

            let width = 8; // TODO: Hardcoded to 8 byte width
            let offset = scope.add_variable(name.clone(), variable_type, width).1;
            asm.push_str(
                format!(
                    "\tsub rsp, {}\n\tmov {} [rbp{}], 0\n",
                    width,
                    get_asm_size_prefix(width),
                    offset
                )
                .as_str(),
            );

            asm
        }
        AST::VariableAssignment { name, value } => {
            let mut asm = String::new();

            asm.push_str(compile_to_asm(program_config, value, scope).as_str());

            let offset = scope.get_variable_offset(name.clone()).1;
            asm.push_str(scope.pop("rax".to_string(), 8).as_str());
            asm.push_str(
                format!("\tmov qword [rbp{}], rax ; assigned `{}`\n", offset, name).as_str(),
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
            asm.push_str(scope.pop("rax".to_string(), 8).as_str());
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
            let mut asm = String::new();
            let id = scope.add_string(value);
            asm.push_str(format!("\tmov rax, STR{}\n", id).as_str());
            asm.push_str(scope.push("rax".to_string(), 8).as_str());
            asm
        }
        _ => {
            eprintln!("Could not find a way to compile {:?} to assembly", root);
            process::exit(1);
        }
    }
}
