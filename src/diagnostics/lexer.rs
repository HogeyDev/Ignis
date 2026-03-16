use crate::diagnostics::util::error_align_caret;

pub fn unknown_character<'a>(filename: &'a str, lines: &'a Vec<String>, pos: (usize, usize)) -> ! {
    eprintln!("{filename}:{}:{}", pos.0+1, pos.1+1);
    let (line, off) = error_align_caret(&lines[pos.0], pos.1);
    eprintln!("{}", line);
    eprintln!("{: >width$}\nunknown character `{}`", '^', lines[pos.0].chars().nth(pos.1).unwrap(), width = off);
    std::process::exit(1);
}

pub fn unterminated_string<'a>(filename: &'a str, lines: &'a Vec<String>, pos: (usize, usize)) -> ! {
    eprintln!("{filename}:{}:{}", pos.0+1, pos.1+1);
    let (line, off) = error_align_caret(&lines[pos.0], pos.1);
    eprintln!("{}", line);
    eprintln!("{: >width$}\nstring is not terminated", '^', width = off);
    std::process::exit(1);
}

pub fn unknown_escape_sequence<'a>(filename: &'a str, lines: &'a Vec<String>, pos: (usize, usize)) -> ! {
    eprintln!("{filename}:{}:{}", pos.0+1, pos.1+1);
    let (line, off) = error_align_caret(&lines[pos.0], pos.1);
    eprintln!("{}", line);
    eprintln!("{: >width$}\nunknown escape sequence", '^', width = off);
    std::process::exit(1);
}

pub fn char_length<'a>(filename: &'a str, lines: &'a Vec<String>, pos: (usize, usize)) -> ! {
    eprintln!("{filename}:{}:{}", pos.0+1, pos.1+1);
    let (line, off) = error_align_caret(&lines[pos.0], pos.1);
    eprintln!("{}", line);
    eprintln!("{: >width$}\nchar type can only fit a single character", '^', width = off);
    std::process::exit(1);
}

pub fn hex_non_int<'a>(filename: &'a str, lines: &'a Vec<String>, pos: (usize, usize)) -> ! {
    eprintln!("{filename}:{}:{}", pos.0+1, pos.1+1);
    let (line, off) = error_align_caret(&lines[pos.0], pos.1);
    eprintln!("{}", line);
    eprintln!("{: >width$}\nexpected a hexadecimal integer to follow '\\x'", '^', width = off);
    std::process::exit(1);
}

pub fn oct_non_int<'a>(filename: &'a str, lines: &'a Vec<String>, pos: (usize, usize)) -> ! {
    eprintln!("{filename}:{}:{}", pos.0+1, pos.1+1);
    let (line, off) = error_align_caret(&lines[pos.0], pos.1);
    eprintln!("{}", line);
    eprintln!("{: >width$}\nexpected an octal integer to follow '\\'", '^', width = off);
    std::process::exit(1);
}
