use std::process;

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
            eprintln!("[ASM] Binary width is not valid for a register: `{width}`");
            process::exit(1);
        }
    }
}
