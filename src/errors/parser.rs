use crate::{errors::util::error_align_caret, parser::TokenKind};

pub fn expect_mismatch<'a>(filename: &'a str, lines: &'a Vec<String>, pos: (usize, usize), correct: TokenKind, wrong: TokenKind) -> ! {
    eprintln!("{filename}:{}:{}", pos.0+1, pos.1+1);
    let (line, off) = error_align_caret(&lines[pos.0], pos.1);
    eprintln!("{}", line);
    eprintln!("{: >width$}", '^', width = off);
    eprintln!("parser expected a {correct:?}, but instead got a {wrong:?}");
    std::process::exit(1);
}
