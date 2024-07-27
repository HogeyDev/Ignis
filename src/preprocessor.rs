use crate::parser::AST;


pub struct PreProcessor {
    pub ast: Box<AST>,
    pub index: usize,
}

impl PreProcessor {
    pub fn new(ast: Box<AST>) -> PreProcessor {
        PreProcessor {
            index: 0,
            ast,
        }
    }
    pub fn preprocess(&mut self) -> Box<AST> {
        self.ast.clone()
            /* TODO: Implement preprocessing (duh)
                1. keep iterating over the ast until no changes have been made
                2. return final ast
            */
    }
}
