use crate::{
    config::Configuration,
    io::SourceFile,
    lexer::{Token, TokenType, Tokenizer},
    parser::{Parser, AST}, preprocessor::PreProcessor,
};

pub fn parse_file(program_config: &Configuration, file: SourceFile) -> Box<AST> {
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


    if program_config.debug_tokens {
        for token in token_list.clone() {
            println!("{:?}", token);
        }
    }

    let mut parser = Parser::new(token_list);
    let parsed = parser.parse();
    if program_config.debug_ast {
        println!("{:#?}", &parsed);
    }
    let mut preprocessor = PreProcessor::new();
    let preprocessed = preprocessor.preprocess(parsed).0;
    preprocessed
}
