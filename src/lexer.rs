pub struct Lexer {
    index: usize,
    content: String,
}

impl Lexer {
    pub fn new(content: String) -> Self {
        Self {
            index: 0,
            content,
        }
    }
}
