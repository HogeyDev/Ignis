use std::process;

#[derive(Debug, Clone)]
pub struct ScopeContext {
    pub stack_size: usize,
    pub variables: Vec<(String, String, usize)>, // [NAME, TYPE, LOCATION]
    pub functions: Vec<(String, String, Vec<String>)>, // [NAME, TYPE, [ARG0, ARG1, ... ARGN]]
}

impl ScopeContext {
    pub fn new() -> ScopeContext {
        ScopeContext {
            stack_size: 0,
            variables: Vec::new(),
            functions: Vec::new(),
        }
    }
    pub fn variable_exists(&self, name: String) -> bool {
        self.variables.iter().any(|i| i.0 == name)
    }
    pub fn add_variable(&mut self, name: String, variable_type: String) -> usize {
        if self.variable_exists(name.clone()) {
            eprintln!("Variable '{}' already exists", name);
            process::exit(1);
        }
        self.variables
            .push((name.clone(), variable_type.clone(), self.stack_size));
        self.stack_size += 1;
        return self.stack_size - 1;
    }
    pub fn push(&mut self, source: String) -> String {
        self.stack_size += 1;
        format!("")
    }
}
