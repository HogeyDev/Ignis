use std::process::exit;

use crate::parser::AST;

pub struct Modulizer {
    pub scope: Box<AST>,
    pub index: usize,
    pub functions: Vec<(String, String, Vec<(String, String)>)>, // [NAME, RETURN, [PARAM, TYPE]]
    pub globals: Vec<String>,
}

impl Modulizer {
    pub fn new(file_root_scope: Box<AST>) -> Self {
        match *file_root_scope {
            AST::Block(_) => {},
            _ => {
                eprintln!("{file_root_scope:?}\nCannot modulize because input is not a Scope");
                exit(1);
            }
        };

        let mut modulizer = Self {
            scope: file_root_scope,
            index: 0,
            functions: Vec::new(),
            globals: Vec::new(),
        };

        modulizer.find_functions();
        // modulizer.find_globals();

        modulizer
    }
    pub fn find_functions(&mut self) {
        match *self.scope.clone() {
            AST::Block(statements) => {
                for stmt in statements {
                    match *stmt {
                        AST::FunctionDeclaration { name, return_type, prototype, .. } => {
                            let function = (name, return_type, prototype.iter().map(|x| match *x.to_owned() {
                                AST::Parameter { param_type, name } => (name, param_type),
                                _ => unreachable!(),
                            }).collect());
                            self.functions.push(function);
                        }
                        _ => continue,
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}
