pub fn unknown_character<'a>(filename: &'a str, lines: &'a Vec<String>, pos: (usize, usize)) -> ! {
    eprintln!("{}", lines[pos.0]);
    eprintln!("{: >width$}", '^', width = pos.1+1);
    eprintln!("unknown character @ {filename}:{}:{}", pos.0+1, pos.1+1);
    std::process::exit(1);
}

pub fn unterminated_string<'a>(filename: &'a str, lines: &'a Vec<String>, pos: (usize, usize)) -> ! {
    eprintln!("{}", lines[pos.0]);
    eprintln!("{: >width$}", '^', width = pos.1+1);
    eprintln!("string is not terminated @ {filename}:{}:{}", pos.0+1, pos.1+1);
    std::process::exit(1);
}

pub fn unknown_escape_sequence<'a>(filename: &'a str, lines: &'a Vec<String>, pos: (usize, usize)) -> ! {
    eprintln!("{}", lines[pos.0]);
    eprintln!("{: >width$}", '^', width = pos.1+1);
    eprintln!("string is not terminated @ {filename}:{}:{}", pos.0+1, pos.1+1);
    std::process::exit(1);
}
