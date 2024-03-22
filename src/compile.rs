use crate::{
    io::SourceFile,
    lexer::{Token, TokenType, Tokenizer},
    parser::Parser,
};

pub fn compile_file(file: SourceFile) -> String {
    let mut tokenizer = Tokenizer::new(file);
    let mut token_list = tokenizer.tokenize();

    token_list.insert(
        0,
        Token {
            token_type: TokenType::LeftBrace,
            value: String::from("{"),
        },
    );
    token_list.push(Token {
        token_type: TokenType::RightBrace,
        value: String::from("}"),
    });

    // for token in token_list {
    //     println!("{:?}", token);
    // }

    let mut parser = Parser::new(token_list);
    let ast = parser.parse();
    println!("{:#?}", ast);

    String::new()
}
