use crate::{io::SourceFile, lexer::Tokenizer, parser::Parser};

pub fn compile_file(file: SourceFile) -> String {
    let mut tokenizer = Tokenizer::new(file);
    let token_list = tokenizer.tokenize();

    // for token in token_list {
    //     println!("{:?}", token);
    // }

    let mut parser = Parser::new(token_list);
    let ast = parser.parse();
    println!("{:#?}", ast);

    String::new()
}
