use crate::{
    config::Configuration,
    io::SourceFile,
    lexer::{Token, TokenType, Tokenizer},
    parser::{Parser, AST},
};

pub fn parse_file(_program_config: Configuration, file: SourceFile) -> Box<AST> {
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
    // token_list.push(Token {
    //     token_type: TokenType::RightBrace,
    //     value: String::from("}"),
    // });

    let mut parser = Parser::new(token_list);
    parser.parse()
}
