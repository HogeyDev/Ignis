use std::{path::Path, process::{self, exit}};

use crate::{
    compile::parse_file, config::Configuration, io::{read_file, SourceFile}, modulizer::Modulizer, parser::{Operation, AST}, scope::ScopeContext, types::{calculate_ast_type, get_type_size, string_to_collapsed_type_tree, Type}, util::{
        asm_size_prefix, asm_size_to_register, initialize_type, move_on_stack, push_from_stack, resolve_address
    }
};

pub fn compile_to_asm(
    program_config: &mut Configuration,
    root: Box<AST>,
    scope: &mut ScopeContext,
) -> String {
    match *root.clone() {
        AST::Null => String::new(),
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
            let mut paths = Vec::new();
            paths.push(program_config.root_path.clone() + "/" + path_with_ending.as_str());
            paths.push(program_config.std_path.clone() + "/" + path_with_ending.as_str());
            let mut file = SourceFile {
                path: String::new(),
                contents: String::new(),
            };
            let mut path_exists = false;
            for path in paths {
                // eprintln!("CHECKING: {path}\n\tFROM: {}", program_config.root_path);
                if Path::new(&path).exists() {
                    file = read_file(path.clone());

                    if program_config.imported_files.contains(&path) {
                        return "".to_string();
                    }
                    program_config.imported_files.push(path);
                    path_exists = true;
                    break;
                }
            }
            if !path_exists {
                eprintln!("Could not resolve import `{module}`");
                process::exit(1);
            }
            // eprintln!("{}\n{}", file.path, file.contents);

            let ast = parse_file(program_config, file);
            let modulizer = Modulizer::new(ast.clone());
            let functions = modulizer.functions;
            for function in functions {
                scope.add_function(function.0, function.1, function.2);
            }
            compile_to_asm(program_config, ast, scope)
        }
        AST::FunctionCall { name, arguments } => {
            let mut asm = String::new();

            let function_data = scope.get_function_data(name.clone()); // also checks if function exists
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
                let arg_type = calculate_ast_type(arg, scope).unwrap();
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

            let is_assignment = op == Operation::Assign;
            let is_access = op == Operation::ArrAcc;
            if !is_assignment && !is_access { asm.push_str(compile_to_asm(program_config, lhs.clone(), scope).as_str()); }
            asm.push_str(compile_to_asm(program_config, rhs.clone(), scope).as_str());
            if !is_assignment && !is_access {
                asm.push_str(scope.pop(String::from("rbx"), 8).as_str()); // rhs
                asm.push_str(scope.pop(String::from("rax"), 8).as_str()); // lhs
            }
            let lhs_typing = calculate_ast_type(lhs.clone(), scope).unwrap();
            let rhs_typing = calculate_ast_type(rhs.clone(), scope).unwrap();

            let lhs_size = get_type_size(lhs_typing.clone()).unwrap();

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
                        let calc_index = scope.pop(String::from("rbx"), 8); // rhs

                        let is_fixed = matches!(*lhs_typing, Type::FixedArray(_, _));

                        let error_finding_address = || {
                            eprintln!("Cannot resolve address of accessed array\n{lhs:#?}");
                            exit(1);
                        };
                        let find_address = match *lhs.clone() {
                            AST::VariableCall { name } => {
                                let offset = scope.get_variable_location(name);
                                format!("\t{} rax, qword [{}{:+}]\n", if is_fixed { "lea" } else { "mov" }, offset.0, offset.1)
                            }
                            AST::UnaryExpression { .. } |
                            AST::BinaryExpression { .. } => {
                                let compiled = compile_to_asm(program_config, lhs, scope);
                                format!("{compiled}\tpop rax\n")
                            }
                            _ => error_finding_address(),
                        };

                        let get_element = {
                            let sub_type = match *lhs_typing.clone() {
                                Type::Slice(sub) => sub,
                                Type::FixedArray(_, sub) => sub,
                                _ => {
                                    eprintln!("[ASM] Array access on non array type");
                                    process::exit(1);
                                }
                            };
                            let sub_size = get_type_size(sub_type.clone()).unwrap();
                            format!("\timul rbx, {sub_size}\n\tadd rax, rbx\n{}", push_from_stack(scope, sub_type, ("rax", 0)))
                        };
                        format!("{calc_index}{find_address}{get_element}")
                    }
                    Operation::Assign => {
                        let mut asm = String::new();

                        asm.push_str(resolve_address(program_config, scope, lhs.clone()).unwrap().as_str());
                        // let from_addr = resolve_address(scope, rhs.clone()).unwrap_or(scope.stack_size);
                        // asm.push_str(move_type_on_stack(scope, rhs_typing, "rsp".to_string(), "rdx".to_string()).as_str());
                        asm.push_str(&move_on_stack(scope, rhs_typing.clone(), ("rsp", 0), ("rdx", 0)));
                        let rhs_type_size = get_type_size(rhs_typing).unwrap() as i64;
                        asm.push_str(&format!("\tadd rsp, {rhs_type_size} ; cleaned up stack\n"));
                        scope.stack_size -= rhs_type_size;

                        asm
                    }
                    _ => {
                        eprintln!("[ASM] Unimplemented binary operation: {:?}", op);
                        process::exit(1);
                    }
                }
                .as_str(),
            );

            if !is_assignment && !is_access {
                asm.push_str(
                    scope
                        .push("rax".to_string(), lhs_size.try_into().unwrap())
                        .as_str(),
                );
            }

            asm
        }
        AST::UnaryExpression { op, child } => {
            let mut asm = String::new();

            // let is_memory_operation = [Operation::Deref, Operation::Ref].contains(&op);
            let is_memory_operation = op == Operation::Ref;
            if !is_memory_operation {
                asm.push_str(compile_to_asm(program_config, child.clone(), scope).as_str());
                asm.push_str(scope.pop(String::from("rax"), 8).as_str()); // lhs
            }
            let typing = calculate_ast_type(child.clone(), scope).unwrap();

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
                                let stack_offset = scope.get_variable_location(name);
                                // eprintln!("{:#?}", variable_type_size);
                                format!("\tlea rax, qword [{}{:+}]\n", stack_offset.0, stack_offset.1)
                                // format!("\tmov rax, rbp\n\tsub rax, {}\n", stack_offset)
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
                                format!("\tmov {}, {} [rax] ; deref or smth\n", register, asm_sizing)
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
                scope.push(
                        "rax".to_string(),
                        get_type_size(typing).unwrap().try_into().unwrap(),
                    ).as_str(),
            );

            asm
        }
        AST::VariableCall { name } => {
            let mut asm = String::new();

            let variable_type = string_to_collapsed_type_tree(scope.get_variable_data(name.clone()).0, scope).unwrap();
            let offset = scope.get_variable_location(name.clone());
            if get_type_size(variable_type.clone()).unwrap() > 8 {
                asm.push_str(&push_from_stack(scope, variable_type, (&offset.0, offset.1)));
                // todo!("Variable type too large");
            } else {
                let type_size = get_type_size(variable_type).unwrap() as i64;
                let register = asm_size_to_register(type_size, "a");
                let asm_sizing = asm_size_prefix(type_size);

                asm.push_str(&format!("\tmov {}, {} [{}{:+}]\n", register, asm_sizing, offset.0, offset.1));
                if type_size == 1 {
                    asm.push_str("\tmovzx rax, al\n");
                } else if type_size == 2 {
                    asm.push_str("\tmovzx rax, ax\n");
                } else if type_size == 4 {
                    asm.push_str("\tmovzx rax, eax\n");
                }
                asm.push_str(scope.push(format!("rax ; recalled `{}`", name), 8).as_str());
            }

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
            is_static,
        } => {
            let mut asm = String::new();

            let collapsed = string_to_collapsed_type_tree(variable_type.clone(), scope).unwrap();
            let width = get_type_size(collapsed.clone()).unwrap() as i64;
            scope.add_variable(name.clone(), variable_type, is_static, width);
            if !is_static {
                asm.push_str(&format!("\tsub rsp, {width} ; stack reserved for `{name}`\n"));
                asm.push_str(&initialize_type(scope, collapsed, ("rsp", 0)));
            }

            asm
        }
        AST::VariableAssignment { name, value } => {
            let mut asm = String::new();

            asm.push_str(compile_to_asm(program_config, value.clone(), scope).as_str());
            let lhs_string_type = scope.get_variable_data(name.clone()).0;
            let lhs_typing = string_to_collapsed_type_tree(lhs_string_type.clone(), scope).unwrap();
            let rhs_typing = calculate_ast_type(value, scope).unwrap();

            if lhs_typing != rhs_typing {
                eprintln!(
                    "[ASM] Attempted to assign expression of type `{}` to variable of type `{}`",
                    rhs_typing.to_string(),
                    lhs_typing.to_string(),
                );
                process::exit(1);
            }

            let lhs_type_size = get_type_size(lhs_typing.clone()).unwrap() as i64;
            if lhs_type_size > 8 {
                unimplemented!("Please review the surrounding code before using this feature cause the previous line feels awfully sketchy and I don't have the time to do a deep dive rn.");
                match *lhs_typing.clone() {
                    Type::Struct(_, members) => {
                        let offset = scope.get_variable_location(name);
                        let temporary_start = -scope.stack_size;
                        let mut internal_offset = 0;
                        for member in members {
                            let member_size = get_type_size(member).unwrap() as i64;

                            asm.push_str(
                                format!(
                                    "\tmov rax, {} [rbp{:+}] ; FORK\n\tmov {} [{}{:+}], rax\n",
                                    "qword",
                                    temporary_start - internal_offset,
                                    "qword",
                                    offset.0,
                                    offset.1 - internal_offset
                                )
                                .as_str(),
                            );

                            internal_offset += member_size;
                        }
                        asm.push_str(&format!("\tadd rsp, {}\n", lhs_type_size));
                    }
                    _ => {
                        eprintln!(
                            "[ASM] type `{}` has size {} (>8), but isn't a struct",
                            lhs_typing.to_string(),
                            lhs_type_size
                        );
                    }
                }
            } else {
                let asm_sizing = asm_size_prefix(lhs_type_size);
                let register = asm_size_to_register(lhs_type_size, "a");

                let offset = scope.get_variable_location(name.clone());
                // asm.push_str(scope.pop(register.clone(), lhs_type_size).as_str());
                scope.stack_size -= lhs_type_size;
                asm.push_str(&format!("\tmov {register}, {asm_sizing} [rsp]\n\tadd rsp, {lhs_type_size}\n\tmov {asm_sizing} [{}{:+}], {register} ; assigned `{name}`\n", offset.0, offset.1));
            }

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
            asm.push_str(format!("\tmov rdx, STR{}\n", id).as_str());
            asm.push_str(scope.push("rdx".to_string(), 8).as_str());
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
            ref name,
            ref members,
        } => {
            let mut asm = String::new();

            let member_types = scope.get_struct_data(name.clone());
            if spreads {
                // scope.add_struct(name, member_types);
                for (_, (member_name, member_type)) in member_types.iter().enumerate() {
                    let expected = string_to_collapsed_type_tree(member_type.clone(), scope);
                    let recieved = calculate_ast_type(members[0].1.clone(), scope);
                    if match *members[0].1 {
                        AST::Integer(num) => num == 0,
                        _ => false,
                    } {
                        let struct_type = calculate_ast_type(root.clone(), scope).unwrap();
                        asm.push_str(&initialize_type(scope, struct_type, ("rsp", 0)));
                    } else if expected != recieved {
                        eprintln!("[ASM] Cannot initialize struct `{}` because member `{}` expects type `{:?}`, but recieved type `{:?}`", name, member_name, expected, recieved);
                        process::exit(1);
                    }
                    asm.push_str(
                        compile_to_asm(program_config, members[0].1.clone(), scope).as_str(),
                    );
                }
            } else {
                unreachable!();
            }

            asm
        }
        AST::MemberAccess { accessed, member } => {
            let mut asm = String::new();

            let accessed_type = calculate_ast_type(accessed.clone(), scope).unwrap();
            // eprintln!("HAS TYPE: {:#?}", accessed_type.clone());
            let struct_name = match *accessed_type.clone() {
                Type::Struct(name, _) => name,
                _ => {
                    eprintln!("[ASM] Cannot perform member access on non-struct\n\tAccessed `{}` from `{:?}`", member, accessed);
                    process::exit(1);
                }
            };
            let member_types = scope.get_struct_data(struct_name.clone());
            let external_offset = resolve_address(program_config, scope, accessed.clone()).unwrap();
            let internal_offset = scope.get_struct_member_offset(struct_name, member.clone()).unwrap();
            // eprintln!("{:?}.{:?} @ {} + {}", accessed, member, external_offset, internal_offset);
            // let mut member_type_size = -1;
            let member_located = member_types
                                    .iter()
                                    .find(|x| { x.0 == member })
                                    .unwrap();
            let member_type = string_to_collapsed_type_tree(member_located.1.to_string(), scope).unwrap();
            let member_type_size = get_type_size(member_type.clone()).unwrap() as i64;
            // for (member_name, member_type) in member_types {
            //     if member_name.to_string() == member {
            //         member_type_size =
            //             get_type_size(string_to_collapsed_type_tree(member_type.to_string(), scope).unwrap())
            //                 .unwrap() as i64;
            //         break;
            //     }
            // }
            // if member_type_size < 0 {
            //     eprintln!("Could not find member `{member}` in struct `{:?}`", accessed_type);
            //     process::exit(1);
            // }
            let register = asm_size_to_register(member_type_size, "a");
            let size_prefix = asm_size_prefix(member_type_size);
            /*
                person: Person
                ptr: @Person    // located at rbp+16
                (@ptr).name     // located at ptr+0
                
                lea rax, [rbp+16]       ; load address of ptr into rax
                mov rax, qword [rax]    ; load address of person into rax
                mov rax, qword [rax+0]  ; load name into rax
            */
            asm.push_str(
                format!("{external_offset}\tlea {register}, {size_prefix} [rdx] ; started to access `{member}`\n")
                .as_str()
            );
            // asm.push_str(
            //     format!("\tlea {register}, {size_prefix} [rbp+{external_offset}] ; starting to access `{member}`\n")
            //     .as_str(),
            // );
            let mut scoping_accessed = accessed.clone();
            while let AST::UnaryExpression { op: Operation::Deref, child: sub_type } = *scoping_accessed.clone() {
                scoping_accessed = sub_type;
                asm.push_str(&format!("\tmov {register}, qword [{register}]\n"));
            }
            asm.push_str(&format!("\tmov {register}, {size_prefix} [{register}{internal_offset:+}] ; finished accessing `{member}`\n"));
            asm.push_str(scope.push("rax".to_string(), member_type_size).as_str());

            asm
        }
        AST::TypeDefinition { name, type_string } => {
            scope.defined_types.push((name, type_string));

            String::new()
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
