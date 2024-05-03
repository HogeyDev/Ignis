use std::{backtrace::Backtrace, process};

use crate::{
    scope::ScopeContext,
    types::{get_type_size, Type},
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
        Type::Struct(members) => members,
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
