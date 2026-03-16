use std::collections::{HashMap, HashSet};

use crate::parser::{Declaration, RootAST, Statement, Type};

enum Symbol {
    Variable {
        kind: Type,
    },
    Enum {
        mods: HashSet<String>,
        variants: Vec<String>,
    },
    Struct {
        fields: HashMap<String, Type>,
    },
}

type Environment = Vec<HashMap<String, Symbol>>;
pub struct Analyzer {
    env: Environment,
    pub err_count: usize,
}

impl Analyzer {
    fn add_entry(&mut self, name: String, symbol: Symbol) {
        if self.env.is_empty() {
            self.env.push(HashMap::new())
        }

        if self.env.last_mut().unwrap().insert(name.clone(), symbol).is_some() {
            eprintln!("\x1b[0;31merror\x1b[0;0m: redefinition of {}", name);
        }
    }

    fn analyze_declaration(&mut self, decl: &Declaration) {
        match &decl {
            &Declaration::ParseError => unreachable!(),
            &Declaration::Enum { name, modifiers, variants } => {
                self.add_entry(name.into(), Symbol::Enum { mods: modifiers.clone(), variants: variants.clone() });
            }
            &Declaration::Struct { name, fields } => {
                self.add_entry(name.into(), Symbol::Struct { fields: fields.clone() });
            }
            &Declaration::Function { name, ret, params, body } => {
                self.add_entry(name.clone(),
                    Symbol::Variable {
                        kind: Type::Function {
                            ret: Box::new(ret.clone()),
                            params: params.iter().map(|x| x.1.clone()).collect() }
                    });

                self.analyze_statement(body);
            }
            &Declaration::TypeDef { name, kind } => {

            }
            &Declaration::Statement(stmt) => {}
        }
    }

    fn analyze_statement(&mut self, body: &Statement) {}

    pub fn run(&mut self, root: &RootAST) {
        let mut env: Environment = Environment::new();
        for decl in root {
            self.analyze_declaration(decl);
        }
    }
}
