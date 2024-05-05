use std::process;

use crate::types::{get_type_size, string_to_collapsed_type_tree, Type};

#[derive(Debug, Clone)]
pub struct ScopeContext {
    pub stack_size: i64, // in bytes
    pub label_counter: usize,
    pub variables: Vec<(String, String, i64)>, // [NAME, TYPE, LOCATION]
    pub functions: Vec<(String, String, Vec<(String, String)>)>, // [NAME, TYPE, [[ARG0, TYPE], [ARG1, TYPE], ... [ARGN, TYPE]]]
    pub strings: Vec<(String, usize)>,                           // [VALUE, ID]
    pub structs: Vec<(String, Vec<(String, String)>)>,           // [NAME, [MEMBER, TYPE]]
    pub defined_types: Vec<(String, String)>,                    // [NAME, TYPE]
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
            label_counter: 0,
            variables: Vec::new(),
            functions: Vec::new(),
            strings: Vec::new(),
            structs: Vec::new(),
            defined_types: Vec::new(),
        }
    }
    pub fn add_parameter(&mut self, name: String, param_type: String, width: i64) -> i64 {
        let first_offset = self
            .variables
            .first()
            .unwrap_or(&("".to_string(), "".to_string(), 0))
            .2;
        let new_offset = first_offset - width;
        self.variables.insert(0, (name, param_type, new_offset));
        new_offset
    }
    pub fn variable_exists(&self, name: String) -> bool {
        self.variables.iter().any(|i| i.0 == name)
    }
    pub fn add_variable(
        &mut self,
        name: String,
        variable_type: String,
        width: i64,
    ) -> (i64, String) {
        if self.variable_exists(name.clone()) {
            eprintln!("[BlockScope] Variable `{}` already exists", name);
            process::exit(1);
        }
        // println!("{:#?}", self.variables);
        // println!(
        //     "Size before: {}\t|\tnew var: {name}: {width}",
        //     self.stack_size
        // );
        self.stack_size += width;
        self.variables
            .push((name.clone(), variable_type.clone(), self.stack_size));
        self.get_variable_offset(name)
    }
    pub fn get_variable_data(&self, name: String) -> (String, i64) {
        match self.variables.iter().find(|x| x.0 == name) {
            Some(value) => (value.clone().1, value.2),
            None => {
                eprintln!("[BlockScope] Could not find variable named `{}`", name);
                process::exit(1);
            }
        }
    }
    pub fn get_variable_offset(&self, name: String) -> (i64, String) {
        let numerical = self.get_variable_data(name).1;
        let mut stringified = format!("{}", -numerical);
        if stringified.chars().nth(0).unwrap_or('0') != '-' {
            // positive number
            stringified.insert(0, '+');
        }

        (numerical, stringified)
    }
    pub fn add_function(
        &mut self,
        name: String,
        function_type: String,
        args: Vec<(String, String)>,
    ) {
        self.functions.push((name, function_type, args));
    }
    pub fn get_function_data(&self, name: String) -> (String, Vec<String>) {
        match self.functions.iter().find(|x| x.0 == name) {
            Some(value) => (
                value.clone().1,
                value.2.iter().map(|x| x.1.clone()).collect(),
            ),
            None => {
                eprintln!("[BlockScope] Could not find function named `{}`", name);
                process::exit(1);
            }
        }
    }
    pub fn absorb_functions(&mut self, scope: ScopeContext) {
        for func in scope.functions {
            if self.functions.iter().any(|x| x.0 == func.0) {
                continue;
            }
            self.functions.push(func);
        }
    }
    pub fn add_struct(&mut self, name: String, members: Vec<(String, String)>) {
        self.structs.push((name.clone(), members.clone()));
        let type_string = Type::Struct(
            name.clone(),
            members
                .iter()
                .map(|x| string_to_collapsed_type_tree(x.1.clone(), self).unwrap())
                .collect(),
        )
        .to_string();
        self.defined_types.push((name, type_string));
    }
    pub fn get_struct_data(&self, name: String) -> Vec<(String, String)> {
        // println!("SFS: {}", name);
        self.structs.iter().find(|x| x.0 == name).unwrap().1.clone()
    }
    pub fn get_struct_member_offset(&self, name: String, member: String) -> Result<i64, String> {
        let struct_data = self.get_struct_data(name.clone());
        let mut tot_off = 0;
        let mut found_result = false;
        for mem in struct_data {
            if mem.0 == member {
                found_result = true;
                break;
            }
            tot_off += get_type_size(string_to_collapsed_type_tree(mem.1, self)?)? as i64;
        }
        if found_result {
            Ok(tot_off)
        } else {
            Err(format!(
                "Could not find member `{}` in struct `{}`",
                member, name
            ))
        }
    }
    pub fn absorb_structs(&mut self, scope: ScopeContext) {
        for str in scope.structs {
            if self.structs.iter().any(|x| x.0 == str.0) {
                continue;
            }
            self.structs.push(str);
        }
    }
    pub fn add_type(&mut self, name: String, type_string: String) {
        self.defined_types.push((name, type_string));
    }
    pub fn absorb_types(&mut self, scope: ScopeContext) {
        for ty in scope.defined_types {
            if self.defined_types.iter().any(|x| x.0 == ty.0) {
                continue;
            }
            self.defined_types.push(ty);
        }
    }
    pub fn push(&mut self, source: String, width: i64) -> String {
        self.stack_size += width;
        format!("\tpush {}\n", source)
    }
    pub fn pop(&mut self, destination: String, width: i64) -> String {
        self.stack_size -= width;
        format!("\tpop {}\n", destination)
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
    pub fn add_label(&mut self) -> usize {
        self.label_counter += 1;
        self.label_counter - 1
    }
    pub fn absorb_labels(&mut self, scope: ScopeContext) {
        self.label_counter = std::cmp::max(scope.label_counter, self.label_counter);
    }
    pub fn sub_scope(&self) -> ScopeContext {
        self.clone()
        // ScopeContext {
        //     stack_size: self.stack_size,
        //     label_counter: self.label_counter,
        //     variables: self.variables.clone(),
        //     functions: self.functions.clone(),
        //     strings: self.strings.clone(),
        //     structs: self.structs.clone(),
        //     defined_types: self.defined_types.clone(),
        // }
    }
    pub fn absorb_stack(&mut self, _scope: ScopeContext) {
        // self.stack_size = std::cmp::max(self.stack_size, scope.stack_size);
    }
    pub fn absorb_sub_scope_globals(&mut self, sub_scope: ScopeContext) {
        self.absorb_strings(sub_scope.clone());
        self.absorb_functions(sub_scope.clone());
        self.absorb_labels(sub_scope.clone());
        self.absorb_stack(sub_scope.clone());
        self.absorb_structs(sub_scope.clone());
        self.absorb_types(sub_scope);
    }
}
