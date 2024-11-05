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

pub fn initialize_struct(
    _scope: ScopeContext,
    offset: i64,
    struct_type: Box<Type>,
    values: Vec<String>,
) -> String {
    let mut asm = format!(
        "\tsub rsp, {}\n",
        get_type_size(struct_type.clone()).unwrap()
    );

    let members = match *struct_type {
        Type::Struct(_, members) => members,
        _ => {
            eprintln!(
                "[ASM] Cannot initialize struct, because it is not a struct lmao\n\t`{:?}`",
                struct_type
            );
            process::exit(1);
        }
    };
    if values.len() < members.len() {
        eprintln!(
            "[ASM] Struct initialization expected a minimum of {} values, but only recieved {}",
            members.len(),
            values.len()
        );
        process::exit(1);
    }

    let mut total_offset = 0;
    for (i, member) in members.iter().enumerate() {
        total_offset += get_type_size(member.to_owned()).unwrap() as i64;
        let size = get_type_size(member.to_owned()).unwrap() as i64;
        let asm_size = asm_size_prefix(size);
        asm.push_str(
            format!(
                "\tmov {} [rbp{:+}], {}\n",
                asm_size,
                -offset - total_offset,
                values[i]
            )
            .as_str(),
        );
    }

    asm
}

pub fn initialize_type(scope: &mut ScopeContext, val_type: Box<Type>) -> String {
    let mut asm = String::new();
    match *val_type {
        Type::Struct(_, members) => {
            for member_type in members {
                asm.push_str(initialize_type(scope, member_type).as_str());
            }
        }
        Type::Primative(_) => {
            let size = get_type_size(val_type).unwrap() as i64;
            asm.push_str(
                scope
                    .push("0 ; zero-initialize primative".to_string(), size)
                    .as_str(),
            )
        }
        Type::DynamicArray(_) => {
            asm.push_str(
                scope
                    .push("0 ; zero-initialize dynamic array".to_string(), 8)
                    .as_str(),
            );
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
    let _typing = calculate_ast_type(ast.clone(), scope)?;
    // println!("\t{:?}\n\t{:?}", ast, typing);
    match *ast.clone() {
        AST::VariableCall { name } => {
            Ok(format!("\tlea rdx, [rbp{:+}]\n", -scope.get_variable_offset(name)))
        }
        AST::MemberAccess { accessed, member } => {
            let struct_typing = calculate_ast_type(accessed.clone(), scope)?;
            let base_addr = resolve_address(program_config, scope, accessed.clone())?;
            let offset = match *struct_typing {
                Type::Struct(name, _) => -scope.get_struct_member_offset(name, member.clone()).unwrap(),
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
                Operation::Deref => resolve_address(program_config, scope, child),
                _ => Err(format!("Cannot resolve address of: {:?}\n\tReason: `Unknown UnaryOperation`", ast)),
            }
        }
        AST::BinaryExpression { op, lhs, rhs } => {
            // eprintln!("{op:?}\n{lhs:#?}\n{rhs:#?}");
            match op {
                Operation::ArrAcc => {
                    let lhs_type = calculate_ast_type(lhs.to_owned(), scope)?;
                    let child_type = match *lhs_type {
                        Type::FixedArray(_, child) => child,
                        _ => {
                            eprintln!("Attempting to calculate resulting memory address from indexing a non array type");
                            exit(1);
                        }
                    };
                    let array_name = match *lhs {
                        AST::VariableCall { name } => name,
                        _ => unreachable!("Honestly I hope that this is unreachable, because I can't seem to think of a single reason why you would be indexing an array which isn't stored in a variable")
                    };
                    let array_base = -scope.get_variable_offset(array_name);
                    let child_size = get_type_size(child_type)?;

                    let rhs_resolution = compile_to_asm(program_config, rhs, scope);

                    Ok(format!("{rhs_resolution}\tpop rcx\n\timul rcx, {child_size}\n\tlea rdx, qword [rbp{array_base:+}]\n\tsub rdx, rcx\n"))
                    // Ok(format!("{child_size}*{}"))
                }
                _ => Err(format!("Cannot resolve address of: {:?}\n\tReason: `Unknown BinaryOperation`", ast)),
            }
        }
        _ => Err(format!("Cannot resolve address of: {:?}\n\tReason: `Unknown AST`", ast)),
    }
}

pub fn move_type_on_stack(scope: &mut ScopeContext, moved_type: Box<Type>, from: String, to: String) -> String {
    let mut asm = String::new();

    let type_size = get_type_size(moved_type.clone()).unwrap() as i64;
    if type_size > 8 {
        // unimplemented!("Large type relocation");
        // has to be a struct
        let struct_name = if let Type::Struct(name, _) = *moved_type { name } else {
            eprintln!("Cannot move type `{:?}` on stack with size {type_size} (>8) ", moved_type);
            process::exit(1);
        };
        let members = scope.get_struct_data(struct_name);
        for (member_name, member_type) in members {
            println!("MEM: {member_name}: {member_type}");
        }
        process::exit(99);
    } else {
        let register = asm_size_to_register(type_size, "a");
        let prefix = asm_size_prefix(type_size);
        asm.push_str(format!("\tmov {register}, {prefix} [{from}]\n\tmov {prefix} [{to}], {register}\n").as_str());
    }

    asm
}

pub fn move_on_stack(scope: &mut ScopeContext, collapsed: Box<Type>, from_top: i64, to_top: i64) -> String {
    let mut asm = String::new();

    match *collapsed.clone() {
        Type::Primative(prim) => {
            if !is_primative_type(prim.clone()) { 
                eprintln!("`{prim}` is an imposter primative");
                exit(1);
            }
            let prim_size = get_type_size(string_to_collapsed_type_tree(prim.clone(), scope).unwrap()).unwrap().try_into().unwrap();
            let register = asm_size_to_register(prim_size, "a");
            let prefix = asm_size_prefix(prim_size);
            asm.push_str(&format!("\tmov {register}, {prefix} [rbp{:+}]\n\tmov {prefix} [rbp{:+}], {register}\n", -(from_top + prim_size), -(to_top + prim_size)));
        }
        // Type::DynamicArray(sub) => {
        //     asm.push_str();
        // },
        Type::FixedArray(size, sub) => {
            // let sub_size = get_type_size(sub.clone()).unwrap();
            for _ in 0..size {
                move_on_stack(scope, sub.clone(), from_top, to_top);
            }
        },
        // Type::Pointer(child) => {},
        // Type::Struct(_, members) => {},
        _ => {
            eprintln!("Cannot move type of {collapsed:#?}");
            exit(1);
        },
    }

    asm
}

pub fn push_variable(scope: &mut ScopeContext, collapsed: Box<Type>, location: i64) -> String {
    let type_size = get_type_size(collapsed.clone()).unwrap();
    let top = -scope.stack_size;
    let movement = move_on_stack(scope, collapsed, location, top);
    format!("\tsub rsp, {type_size}\n{movement}")
}

pub fn type_is_struct(scope: &ScopeContext, type_name: String) -> bool {
    scope.structs.iter().find(|x| x.0 == type_name).is_some()
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
