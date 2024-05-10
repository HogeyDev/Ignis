use std::{backtrace::Backtrace, process};

use crate::{
    parser::AST,
    scope::ScopeContext,
    types::{calculate_ast_type, get_type_size, Type},
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
        Type::Struct(_name, members) => members,
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
        total_offset += get_type_size(member.to_owned()).unwrap() as i64;
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

pub fn resolve_address(scope: &ScopeContext, ast: Box<AST>) -> Result<i64, String> {
    let typing = calculate_ast_type(ast.clone(), scope)?;
    // println!("\t{:?}\n\t{:?}", ast, typing);
    match *ast.clone() {
        AST::VariableCall { name } => Ok(scope.get_variable_offset(name).0 as i64),
        AST::MemberAccess { accessed, .. } => {
            let acc_typing = calculate_ast_type(accessed.clone(), scope)?;
            let base_addr = resolve_address(scope, accessed)?;
            let offset = match *acc_typing {
                Type::Struct(_name, _members) => 0,
                _ => {
                    eprintln!(
                        "Failure to resolve: {:?}\n\tgoes with {:?}",
                        acc_typing, ast
                    );
                    process::exit(1);
                }
            };
            Ok(base_addr + offset)
        }
        _ => Err(format!("Can't resolve address of: {:?}", ast)),
    }
}
