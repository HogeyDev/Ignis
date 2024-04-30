use std::process;

use crate::{
    compile::parse_file,
    config::Configuration,
    io::read_file,
    parser::{Operation, AST},
    scope::ScopeContext,
    types::{
        ast_to_type_tree, calculate_expression_type, get_type_size, string_to_collapsed_type_tree,
        Type,
    },
    util::{asm_size_prefix, asm_size_to_register},
};

pub fn compile_to_asm(
    program_config: &mut Configuration,
    root: Box<AST>,
    scope: &mut ScopeContext,
) -> String {
    match *root {
        AST::Block(statements) => {
            let mut asm = String::new();

            let mut block_scope = scope.sub_scope();

            for statement in statements {
                asm.push_str(compile_to_asm(program_config, statement, &mut block_scope).as_str());
            }

            scope.absorb_sub_scope_globals(block_scope.to_owned());
            asm
        }
        AST::Import { module } => {
            let path_with_ending = module.replace('.', "/") + ".is";
            let full_path = program_config.root_path.clone() + "/" + path_with_ending.as_str();
            let file = read_file(full_path.clone());

            if program_config.imported_files.contains(&full_path) {
                return "".to_string();
            }
            program_config.imported_files.push(full_path);

            compile_to_asm(program_config, parse_file(program_config, file), scope)
        }
        AST::FunctionCall { name, arguments } => {
            let mut asm = String::new();

            let function_data = scope.get_function_data(name.clone()); // also checks if function
                                                                       // exists
            if function_data.1.len() != arguments.len() {
                eprintln!(
                    "[ASM] Function `{}` expected `{}` arguments, but recieved `{}`",
                    name,
                    function_data.1.len(),
                    arguments.len()
                );
                process::exit(1);
            }
            let mut added_stack_size = 0;
            for (i, arg) in arguments.iter().cloned().enumerate() {
                asm.push_str(compile_to_asm(program_config, arg.clone(), scope).as_str());

                // check typing
                let arg_type = calculate_expression_type(arg, scope).unwrap();
                let func_arg_type =
                    string_to_collapsed_type_tree(function_data.1[i].clone(), scope).unwrap();
                if arg_type != func_arg_type {
                    // eprintln!("{:?}\n{:?}", func_arg_type, arg_type);
                    eprintln!("[ASM] Function `{}` expected argument of type `{}`, but recieved argument of type `{}`", name, func_arg_type.to_string(), arg_type.to_string());
                    process::exit(1);
                }
                added_stack_size += 8;
                // added_stack_size += get_type_size(arg_type).unwrap();
            }
            asm.push_str(format!("\tcall _{}\n\tadd rsp, {}\n", name, added_stack_size).as_str());
            scope.stack_size -= added_stack_size as i64;
            if function_data.0 != "void" {
                // has a notable return value
                let type_size =
                    get_type_size(string_to_collapsed_type_tree(function_data.0, scope).unwrap())
                        .unwrap() as i64;
                let register = asm_size_to_register(type_size, "a");
                if type_size < 8 {
                    asm.push_str(format!("\tmovzx rax, {}\n", register).as_str());
                }
                asm.push_str(scope.push("rax".to_string(), type_size).as_str());
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

            body_scope.add_parameter("".to_string(), "@usize".to_string(), 8);

            let mut params = Vec::new();
            for param in prototype.iter().rev().cloned() {
                asm.push_str(
                    compile_to_asm(program_config, param.clone(), &mut body_scope).as_str(),
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

            body_scope.add_function(name, return_type, params.clone());

            asm.push_str(compile_to_asm(program_config, body, &mut body_scope).as_str());
            asm.push_str("\tmov rsp, rbp\n\tpop rbp\n\tret\n");

            scope.absorb_sub_scope_globals(body_scope);

            asm
        }
        AST::Parameter { param_type, name } => {
            // println!(
            //     "PRM: {name} | {}",
            scope.add_parameter(name.clone(), param_type, 8);
            // );
            String::new()
        }
        AST::If {
            condition,
            body,
            alt,
        } => {
            let mut asm = String::new();

            let end_label_id = scope.add_label();

            asm.push_str(compile_to_asm(program_config, condition, scope).as_str());
            asm.push_str(scope.pop("rax".to_string(), 8).as_str());
            asm.push_str(format!("\tcmp rax, 0\n\tje lbl{}\n", end_label_id).as_str());

            asm.push_str(compile_to_asm(program_config, body, scope).as_str());
            asm.push_str(format!("lbl{}:\n", end_label_id).as_str());
            if let Some(val) = alt {
                asm.push_str(compile_to_asm(program_config, val, scope).as_str());
            }

            // asm.push_str(compile_to_asm(program_config.clone(), condition, scope).as_str());
            // asm.push_str(compile_to_asm(program_config, body, scope).as_str());

            asm
        }
        AST::BinaryExpression { op, lhs, rhs } => {
            let mut asm = String::new();

            asm.push_str(compile_to_asm(program_config, lhs.clone(), scope).as_str());
            asm.push_str(compile_to_asm(program_config, rhs.clone(), scope).as_str());
            asm.push_str(scope.pop(String::from("rbx"), 8).as_str()); // rhs
            asm.push_str(scope.pop(String::from("rax"), 8).as_str()); // lhs
            let lhs_typing = calculate_expression_type(lhs, scope).unwrap();
            // let rhs_typing = calculate_expression_type(rhs, scope).unwrap();

            // if lhs_typing != rhs_typing {
            //     eprintln!(
            //         "[ASM] Cannot do binary operation `{:?}` on mismatching types:\n\t`{:?}` != `{:?}`",
            //         op, lhs_typing, rhs_typing
            //     );
            //     process::exit(1);
            // }

            asm.push_str(
                match op {
                    Operation::Add => "\tadd rax, rbx\n".to_string(),
                    Operation::Sub => "\tsub rax, rbx\n".to_string(),
                    Operation::Mul => "\timul rax, rbx\n".to_string(),
                    Operation::Div => "\tmov rdx, 0\n\tdiv rbx\n".to_string(),
                    Operation::Mod => "\tmov rdx, 0\n\tdiv rbx\n\tmov rax, rdx\n".to_string(),
                    Operation::Or => "\tor rax, rbx\n".to_string(),
                    Operation::And => "\tand rax, rbx\n".to_string(),
                    Operation::Eq => "\tcmp rax, rbx\n\tsete al\n\tmovzx rax, al\n".to_string(),
                    Operation::Neq => "\tcmp rax, rbx\n\tsetne al\n\tmovzx rax, al\n".to_string(),
                    Operation::LT => "\tcmp rax, rbx\n\tsetl al\n\tmovzx rax, al\n".to_string(),
                    Operation::GT => "\tcmp rax, rbx\n\tsetg al\n\tmovzx rax, al\n".to_string(),
                    Operation::LTE => "\tcmp rax, rbx\n\tsetle al\n\tmovzx rax, al\n".to_string(),
                    Operation::GTE => "\tcmp rax, rbx\n\tsetge al\n\tmovzx rax, al\n".to_string(),
                    Operation::ArrAcc => {
                        let element_size = match *lhs_typing.clone() {
                            Type::DynamicArray(sub) => get_type_size(sub).unwrap(),
                            Type::FixedArray(_, sub) => get_type_size(sub).unwrap(),
                            _ => {
                                eprintln!("[ASM] Array access on non array type");
                                process::exit(1);
                            }
                        };
                        if element_size == 4 {
                            format!(
                                "\timul rbx, {}\n\txor ecx, ecx\n\tmov ecx, dword [rax + rbx]\n",
                                element_size
                            )
                        } else if element_size == 8 {
                            format!(
                                "\timul rbx, {}\n\tmov rax, qword [rax + rbx]\n",
                                element_size,
                            )
                        } else {
                            format!(
                                "\timul rbx, {}\n\tmovzx rax, {} [rax + rbx]\n",
                                element_size,
                                asm_size_prefix(element_size.try_into().unwrap_or(0))
                            )
                        }
                    }
                    _ => {
                        eprintln!("[ASM] Unimplemented binary operation: {:?}", op);
                        process::exit(1);
                    }
                }
                .as_str(),
            );

            let lhs_size = get_type_size(lhs_typing).unwrap();
            asm.push_str(
                scope
                    .push("rax".to_string(), lhs_size.try_into().unwrap())
                    .as_str(),
            );

            asm
        }
        AST::UnaryExpression { op, child } => {
            let mut asm = String::new();

            asm.push_str(compile_to_asm(program_config, child.clone(), scope).as_str());
            asm.push_str(scope.pop(String::from("rax"), 8).as_str()); // lhs
            let typing = calculate_expression_type(child.clone(), scope).unwrap();

            asm.push_str(
                match op {
                    Operation::Inc => "\tinc rax\n".to_string(),
                    Operation::Dec => "\tdec rax\n".to_string(),
                    Operation::Inv => "\tnot rax\n".to_string(),
                    Operation::Neg => "\tneg rax\n".to_string(),
                    Operation::Ref => {
                        match *child {
                            AST::VariableCall { name } => {
                                // this is good!
                                let stack_offset = scope.get_variable_offset(name).0;
                                format!("\tmov rax, rbp\n\tsub rax, {}\n", stack_offset)
                            }
                            _ => {
                                // this is bad!
                                eprintln!("[ASM] Cannot reference non variable value");
                                process::exit(1);
                            }
                        }
                    }
                    Operation::Deref => {
                        let size = get_type_size(typing.clone()).unwrap() as i64;
                        let asm_sizing = asm_size_prefix(size);
                        let register = asm_size_to_register(size, "a");
                        match *typing {
                            Type::Pointer(_) => {
                                format!("\tmov {}, {} [rax]\n", register, asm_sizing)
                            }
                            _ => {
                                eprintln!("[ASM] Cannot dereference non-pointer type");
                                process::exit(1);
                            }
                        }
                    }
                    _ => {
                        eprintln!("[ASM] Unimplemented unary operation: {:?}", op);
                        process::exit(1);
                    }
                }
                .as_str(),
            );

            asm.push_str(
                scope
                    .push(
                        "rax".to_string(),
                        get_type_size(typing).unwrap().try_into().unwrap(),
                    )
                    .as_str(),
            );

            asm
        }
        AST::VariableCall { name } => {
            let mut asm = String::new();

            let variable_type = scope.get_variable_data(name.clone()).0;
            let type_size = get_type_size(
                string_to_collapsed_type_tree(variable_type, scope).unwrap(),
            )
            .unwrap() as i64;
            let offset = scope.get_variable_offset(name.clone()).1;
            let register = asm_size_to_register(type_size, "a");
            let asm_sizing = asm_size_prefix(type_size);

            asm.push_str(format!("\tmov {}, {} [rbp{}]\n", register, asm_sizing, offset).as_str());
            if type_size < 8 {
                asm.push_str("\tmovzx rax, al\n");
            }
            asm.push_str(scope.push(format!("rax; recalled `{}`", name), 8).as_str());

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
            asm.push_str("\tmov rsp, rbp\n\tpop rbp\n\tret\n");

            asm
        }
        AST::VariableDeclaration {
            variable_type,
            name,
        } => {
            let mut asm = String::new();

            let collapsed = string_to_collapsed_type_tree(variable_type.clone(), scope).unwrap();
            let width = get_type_size(collapsed).unwrap().try_into().unwrap();
            let offset = scope.add_variable(name.clone(), variable_type, width).1;
            asm.push_str(
                format!(
                    "\tsub rsp, {}\n\tmov {} [rbp{}], 0\n",
                    width,
                    asm_size_prefix(width),
                    offset
                )
                .as_str(),
            );

            asm
        }
        AST::VariableAssignment { name, value } => {
            let mut asm = String::new();

            asm.push_str(compile_to_asm(program_config, value.clone(), scope).as_str());
            let lhs_string_type = scope.get_variable_data(name.clone()).0;
            let lhs_typing = string_to_collapsed_type_tree(lhs_string_type.clone(), scope).unwrap();
            let rhs_typing = calculate_expression_type(value, scope).unwrap();

            if lhs_typing != rhs_typing {
                eprintln!(
                    "[ASM] Attempted to assign expression of type `{}` to variable of type `{}`",
                    rhs_typing.to_string(),
                    lhs_string_type,
                );
                process::exit(1);
            }

            let variable_type = scope.get_variable_data(name.clone()).0;
            let type_size = get_type_size(
                string_to_collapsed_type_tree(variable_type, scope).unwrap(),
            )
            .unwrap() as i64;
            let asm_sizing = asm_size_prefix(type_size);
            let register = asm_size_to_register(type_size, "a");

            let offset = scope.get_variable_offset(name.clone()).1;
            asm.push_str(scope.pop("rax".to_string(), 8).as_str());
            asm.push_str(
                format!(
                    "\tmov {} [rbp{}], {}; assigned `{}`\n",
                    asm_sizing, offset, register, name
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
            asm.push_str(compile_to_asm(program_config, condition, scope).as_str());
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
        AST::Character(value) => {
            let mut asm = String::new();
            asm.push_str(format!("\tmov rdx, {}\n", value as i8).as_str());
            asm.push_str(scope.push("rdx".to_string(), 8).as_str());
            asm
        }
        AST::Struct { name, members } => {
            scope.add_struct(name, members);
            String::new()
        }
        AST::StructInitializer {
            spreads,
            name,
            members,
        } => {
            let mut asm = String::new();

            let member_types = scope.get_struct_data(name.clone());
            if spreads {
                // scope.add_struct(name, member_types);
                for (_, (member_name, member_type)) in member_types.iter().enumerate() {
                    let expected = string_to_collapsed_type_tree(member_type.clone(), scope);
                    let recieved = ast_to_type_tree(members[0].1.clone(), scope);
                    if match *members[0].1 {
                        AST::Integer(0) => true,
                        _ => false,
                    } {
                    } else if expected != recieved {
                        eprintln!("[ASM] Cannot initialize struct `{}` because member `{}` expects type `{:?}`, but recieved type `{:?}`", name, member_name, expected, recieved);
                        process::exit(1);
                    }
                    asm.push_str(
                        compile_to_asm(program_config, members[0].1.clone(), scope).as_str(),
                    );
                }
            } else {
            }

            asm
        }
        AST::MemberAccess { accessed, member } => {
            let mut asm = String::new();

            let name = match *accessed {
                AST::VariableCall { name } => name,
                _ => {
                    eprintln!("[ASM] Cannot perform member access on non-variable\n\tAccessed `{}` from `{:?}`", member, accessed);
                    process::exit(1);
                }
            };

            let data = scope.get_variable_data(name.clone());
            let member_types = scope.get_struct_data(data.0);
            let inter_offset = scope.get_variable_offset(name).0;
            let mut intra_offset = 0;
            let mut type_size = 0;
            for (member_name, member_type) in member_types {
                if member_name == member {
                    type_size =
                        get_type_size(string_to_collapsed_type_tree(member_type, scope).unwrap())
                            .unwrap();
                    break;
                }
                intra_offset +=
                    get_type_size(string_to_collapsed_type_tree(member_type, scope).unwrap())
                        .unwrap();
            }
            let type_size: i64 = type_size as i64;
            let register = asm_size_to_register(type_size, "a");
            let asm_sizing = asm_size_prefix(type_size);
            asm.push_str(
                format!(
                    "\tmov {}, {} [rbp{:+}]",
                    register,
                    asm_sizing,
                    inter_offset - intra_offset as i64
                )
                .as_str(),
            );

            asm
        }
        _ => {
            eprintln!(
                "[ASM] Could not find a way to compile {:?} to assembly",
                root
            );
            process::exit(1);
        }
    }
}
