use std::process;

#[derive(Debug, Clone)]
pub struct ScopeContext {
    pub stack_size: usize,
    pub variables: Vec<(String, String, usize)>, // [NAME, TYPE, LOCATION]
    pub functions: Vec<(String, String, Vec<String>)>, // [NAME, TYPE, [ARG0, ARG1, ... ARGN]]
    pub strings: Vec<(String, usize)>,           // [VALUE, ID]
}

impl Default for ScopeContext {
    fn default() -> Self {
        Self::new()
    }
}

impl ScopeContext {
    pub fn new() -> ScopeContext {
        ScopeContext {
            stack_size: 0,
            variables: Vec::new(),
            functions: Vec::new(),
            strings: Vec::new(),
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
        self.stack_size - 1
    }
    pub fn get_variable_data(&self, name: String) -> (String, usize) {
        match self.variables.iter().filter(|x| x.0 == name).nth(0) {
            Some(value) => {
                return (value.clone().1, value.2);
            }
            None => {
                eprintln!("Could not find variable named `{}`", name);
                process::exit(1);
            }
        }
    }
    pub fn get_variable_offset(&self, name: String) -> usize {
        self.stack_size - self.get_variable_data(name).1
    }
    pub fn push(&mut self, source: String) -> String {
        self.stack_size += 1;
        format!("\tpush {}\n", source)
    }
    pub fn pop(&mut self, destination: String) -> String {
        self.stack_size -= 1;
        format!("\tpop {}\n", destination)
    }
    pub fn sub_scope(&self) -> ScopeContext {
        ScopeContext {
            stack_size: self.stack_size,
            variables: self.variables.clone(),
            functions: self.functions.clone(),
            strings: self.strings.clone(),
        }
    }
    pub fn absorb_strings(&mut self, scope: ScopeContext) {
        for str in scope.strings {
            if self.strings.iter().any(|x| x.1 == str.1) {
                continue;
            }
            self.strings.push(str);
        }
        self.strings.sort_by(|a, b| a.1.cmp(&b.1));
    }
    pub fn add_string(&mut self, value: String) -> usize {
        let id = self.strings.len();
        self.strings.push((value, id));
        id
    }
    pub fn compile_strings(&self) -> String {
        let mut strings = String::new();

        for str in self.strings.clone() {
            strings.push_str(format!("\tSTR{} db \"{}\", 0\n", str.1, str.0).as_str());
        }

        strings
    }
}
