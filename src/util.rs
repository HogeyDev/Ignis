use std::{backtrace::Backtrace, process::{self, exit}};

use crate::{
    codegen::compile_to_asm, config::Configuration, parser::{Operation, AST}, scope::ScopeContext, types::{calculate_ast_type, get_type_size, is_primative_type, string_to_collapsed_type_tree, Type}
};

pub fn asm_size_prefix(width: i64) -> String {
    match width {
        1 => "byte",
        2 => "word",
        4 => "dword",
        8 => "qword",
        _ => {
            eprintln!("[ASM] Binary width is not valid: `{width}`");
            process::exit(1);
        }
    }
    .to_string()
}

pub fn asm_size_to_register(width: i64, reg: &str) -> String {
    let working_registers = vec!["a", "b", "c", "d"];
    if !working_registers.contains(&reg) {
        eprintln!(
            "asm_size_to_register only works with registers `{:?}`, but recieved `{}`",
            working_registers, reg
        );
        process::exit(1);
    }
    match width {
        1 => format!("{}l", reg),
        2 => format!("{}x", reg),
        4 => format!("e{}x", reg),
        8 => format!("r{}x", reg),
        _ => {
            let bt = Backtrace::capture();
            eprintln!("{bt}");
            eprintln!("[ASM] Binary width is not valid for a register: `{width}`");
            process::exit(1);
        }
    }
}

pub fn initialize_type(scope: &mut ScopeContext, val_type: Box<Type>, loc: (&str, i64)) -> String {
    let mut asm = String::new();
    match *val_type {
        Type::Struct(name, ..) => {
            let members: Vec<Box<Type>> = scope.get_struct_data(name).iter().map(|x| string_to_collapsed_type_tree(x.1.clone(), scope).unwrap()).collect();
            let mut size_accumulator = 0;
            for member_type in members {
                let member_size = get_type_size(scope, member_type.clone()).unwrap();
                asm.push_str(&initialize_type(scope, member_type.clone(), (loc.0, loc.1 + size_accumulator)));
                size_accumulator += member_size as i64;
            }
        }
        Type::Primative(_) => {
            let size = get_type_size(scope, val_type).unwrap() as i64;
            let prefix = asm_size_prefix(size);
            asm.push_str(&format!("\tmov {prefix} [{}{:+}], 0\n", loc.0, loc.1))
        }
        Type::FixedArray(size, sub) => {
            let sub_size = get_type_size(scope, sub.clone()).unwrap();
            for i in 0..size {
                asm.push_str(&initialize_type(scope, sub.clone(), (loc.0, loc.1 + (sub_size * i) as i64)));
            }
        }
        Type::Slice(_) => {
            asm.push_str(&format!("\tmov qword [{}{:+}], 0\n", loc.0, loc.1));
        }
        Type::Pointer(_) => {
            asm.push_str(&format!("\tmov qword [{}{:+}], 0\n", loc.0, loc.1));
        }
        _ => {
            eprintln!(
                "[ASM] Cannot zero initialize this thingy:\n\t{:?}",
                val_type
            );
            process::exit(1);
        }
    }
    asm
}

pub fn resolve_address(program_config: &mut Configuration, scope: &mut ScopeContext, ast: Box<AST>) -> Result<String, String> {
    // let typing = calculate_ast_type(ast.clone(), scope)?;
    // println!("\t{:?}\n\t{:?}", ast, typing);
    match *ast.clone() {
        AST::VariableCall { name } => {
            let location = scope.get_variable_location(name);
            Ok(format!("\tlea rdx, qword [{}{:+}]\n", location.0, location.1))
        }
        AST::MemberAccess { accessed, member } => {
            let struct_typing = calculate_ast_type(accessed.clone(), scope)?;
            let base_addr = resolve_address(program_config, scope, accessed.clone())?;
            let offset = match *struct_typing {
                Type::Struct(name, ..) => scope.get_struct_member_offset(name, member.clone()).unwrap(),
                _ => {
                    eprintln!(
                        "Failure to resolve: {:?}\n\tgoes with {:?}",
                        struct_typing, ast
                    );
                    process::exit(1);
                }
            };
            if offset < 0 {
                Ok(format!("{base_addr}\tsub rdx, {} ; `{member}`\n", -offset))
            } else {
                Ok(format!("{base_addr}\tadd rdx, {offset} ; `{member}`\n"))
            }
        }
        AST::UnaryExpression { op, child } => {
            match op {
                Operation::Deref => {
                    let child_calculated = compile_to_asm(program_config, child, scope);
                    Ok(format!(";TESTTESTTESTTEST\n{};TESTESTESTESTESTESTESTESTEST\n\tpop rdx\n", child_calculated))
                }
                _ => Err(format!("Cannot resolve address of: {:?}\n\tReason: `Unknown UnaryOperation`", ast)),
            }
        }
        AST::BinaryExpression { op, lhs, rhs } => {
            // eprintln!("{op:?}\n{lhs:#?}\n{rhs:#?}");
            match op {
                Operation::ArrAcc => {
                    let lhs_type = calculate_ast_type(lhs.to_owned(), scope)?;
                    let child_type = match *lhs_type {
                        Type::FixedArray(_, child) => {
                            child
                        }
                        Type::Slice(child) => {
                            child
                        }
                        x => {
                            eprintln!("Attempting to calculate resulting memory address from indexing a non array type: {x:?}");
                            exit(1);
                        }
                    };
                    let array_base = match *lhs {
                        AST::VariableCall { name } => scope.get_variable_location(name),
                        _ => unreachable!("Honestly I hope that this is unreachable, because I can't seem to think of a single reason why you would be indexing an array which isn't stored in a variable")
                    };
                    let child_size = get_type_size(scope, child_type)?;
                    // eprintln!("{array_name}: {array_base}");

                    let rhs_resolution = compile_to_asm(program_config, rhs, scope);

                    Ok(format!("{rhs_resolution}\tpop rcx\n\tlea rdx, qword [{}{:+}+rcx*{child_size}] ; rcx is multiplied to adjust for type sizing\n", array_base.0, array_base.1))
                    // Ok(format!("{child_size}*{}"))
                }
                // _ => Ok("\tlea rdx, qword [rsp+8]\n".to_owned()),
                _ => Err(format!("Cannot resolve address of: {:?}\n\tReason: `Unknown BinaryOperation`", ast)),
            }
        }
        _ => Err(format!("Cannot resolve address of: {:?}\n\tReason: `Unknown AST`", ast)),
    }
}

pub fn move_on_stack(scope: &mut ScopeContext, collapsed: Box<Type>, from_bottom: (&str, i64), to_bottom: (&str, i64)) -> String {
    let mut asm = String::new();

    match *collapsed.clone() {
        Type::Primative(prim) => {
            if !is_primative_type(prim.clone()) { 
                eprintln!("`{prim}` is an imposter primative");
                exit(1);
            }
            let prim_size = get_type_size(scope, string_to_collapsed_type_tree(prim.clone(), scope).unwrap()).unwrap().try_into().unwrap();
            let register = asm_size_to_register(prim_size, "a");
            let prefix = asm_size_prefix(prim_size);
            asm.push_str(&format!("\tmov {register}, {prefix} [{}{:+}]\n\tmov {prefix} [{}{:+}], {register}\n",
                            from_bottom.0,
                            from_bottom.1,
                            to_bottom.0,
                            to_bottom.1
                        ));
        }
        Type::Slice(_) => {
            asm.push_str(&format!("\tmov r10, qword [{}{:+}]\n\tmov qword [{}{:+}], r10\n", from_bottom.0, from_bottom.1, to_bottom.0, to_bottom.1));
        },
        Type::FixedArray(size, sub) => {
            let sub_size = get_type_size(scope, sub.clone()).unwrap();
            for i in (0..size).rev() {
                asm.push_str(&move_on_stack(scope, sub.clone(), (from_bottom.0, from_bottom.1 + ((i * sub_size) as i64)), (to_bottom.0, to_bottom.1 + ((i * sub_size) as i64))));
            }
        },
        Type::Pointer(_) => {
            asm.push_str(&move_on_stack(scope, Box::new(Type::Primative("usize".to_owned())), from_bottom, to_bottom));
        },
        Type::Struct(name, ..) => {
            let members: Vec<Box<Type>> = scope.get_struct_data(name).iter().map(|x| string_to_collapsed_type_tree(x.1.clone(), scope).unwrap()).collect();
            let mut total_size = 0;
            for member in members.iter().rev() {
                let child_size = get_type_size(scope, member.clone()).unwrap() as i64;
                asm.push_str(&move_on_stack(scope, member.clone(), (from_bottom.0, from_bottom.1 + total_size), (to_bottom.0, to_bottom.1 + total_size)));
                total_size += child_size;
            }
        },
        _ => {
            eprintln!("Cannot move type of {collapsed:#?}");
            exit(1);
        },
    }

    asm
}

pub fn push_from_stack(scope: &mut ScopeContext, collapsed: Box<Type>, location: (&str, i64)) -> String {
    let type_size = get_type_size(scope, collapsed.clone()).unwrap() as i64;
    let movement = move_on_stack(scope, collapsed, location, ("rsp", 0));
    scope.stack_size += type_size;
    format!("\tsub rsp, {type_size} ; allocated space to push into\n{movement}")
}

pub fn type_is_struct(scope: &ScopeContext, type_name: String) -> bool {
    scope.structs.iter().find(|x| x.0 == type_name).is_some()
}

pub fn type_is_typedef(scope: &ScopeContext, type_name: String) -> bool {
    scope.defined_types.iter().find(|x| x.0 == type_name).is_some()
}

pub fn parse_expansion_string(exp: String) -> Vec<(bool, String)> {
    let mut res = Vec::new();

    let mut i = 0;
    let mut buf = String::new();
    while let Some(c) = exp.chars().nth(i) {
        if c == '\\' {
            if buf.len() > 0 { res.push((true, buf.clone())); }
            buf.clear();
            let mut num_buf = String::new();
            while c.is_ascii_digit() {
                num_buf.push(c);
                i += 1;
            }
            res.push((false, num_buf));
        } else {
            buf.push(c);
            i += 1;
        }
    }

    return res;
}
