use crate::parser::AST;

pub struct Modulizer {
    pub file_root_scope: AST,
    pub index: usize,
    pub functions: Vec<(String, Vec<String>)>, // [NAME, [ARGS]]
    pub globals: Vec<String>,
}

impl Modulizer {
    pub fn new(file_root_scope: AST) -> Self {
        Self {
            file_root_scope,
            index: 0,
            functions: Vec::new(),
            globals: Vec::new(),
        }
    }
}
