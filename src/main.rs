use crate::lexer::Lexer;

mod lexer;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let input_file = args[0].as_str();
    let source = std::fs::read_to_string(input_file)?;
    
    let tokens = Lexer::new(source).tokens();
    
    Ok(())
}
