pub fn error_align_caret(line: &String, pos: usize) -> (String, usize) {
    let mut fixed = String::new();
    let mut off = 0;

    for (i, c) in line.chars().enumerate() {
        if c == '\t' {
            fixed.push_str("    ");
            if i < pos { off += 4; }
        } else {
            fixed.push(c);
            if i < pos { off += 1; }
        }
    }

    (fixed, off)
}
