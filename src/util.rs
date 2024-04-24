use std::process;

pub fn get_asm_size_prefix(width: i64) -> String {
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
