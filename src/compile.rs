use crate::{io::SourceFile, lexer::Tokenizer};

pub fn compile_file(file: SourceFile) -> String {
    let mut tokenizer = Tokenizer::new(file);
    let token_list = tokenizer.tokenize();

    for token in token_list {
        println!("{:?}", token);
    }

    String::new()
}
