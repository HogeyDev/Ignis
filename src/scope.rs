use std::{backtrace::Backtrace, process};

use crate::types::{get_type_size, string_to_collapsed_type_tree, Type};

#[derive(Debug, Clone)]
pub struct ScopeContext {
    pub stack_size: i64, // in bytes
    pub label_counter: usize,
    pub variables: Vec<(String, String, i64, bool, String)>, // [NAME, TYPE, LOCATION, IS_STATIC, BASE]
    pub functions: Vec<(String, String, Vec<(String, String)>)>, // [NAME, TYPE, [[ARG0, TYPE], [ARG1, TYPE], ... [ARGN, TYPE]]]
    pub strings: Vec<(String, usize)>,                           // [VALUE, ID]
    pub structs: Vec<(String, Vec<(String, String)>)>,           // [NAME, [MEMBER, TYPE]]
    pub enums: Vec<String>,
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
            enums: Vec::new(),
            defined_types: Vec::new(),
        }
    }
    pub fn add_parameter(&mut self, name: String, param_type: String, width: i64) -> i64 {
        let first_offset = self
            .variables
            .first()
            .unwrap_or(&("".to_string(), "".to_string(), 0, false, "".to_owned()))
            .2;
        let new_offset = first_offset - width;
        self.variables.insert(0, (name, param_type, new_offset, false, "rbp".to_owned()));
        new_offset
    }
    pub fn variable_exists(&self, name: String) -> bool {
        self.variables.iter().any(|i| i.0 == name)
    }
    pub fn add_variable(
        &mut self,
        name: String,
        variable_type: String,
        is_static: bool,
        width: i64,
    ) -> (String, i64) {
        if self.variable_exists(name.clone()) {
            eprintln!("[BlockScope] Variable `{}` already exists", name);
            process::exit(1);
        }
        let label = if !is_static { self.stack_size += width; 0 } else { self.add_label() };
        self.variables
            .push((name.clone(), variable_type.clone(), self.stack_size, is_static, if is_static { format!("GLO{}", label) } else { "rbp".to_owned() }));
        let location = self.get_variable_location(name);
        (location.0, -location.1)
    }
    pub fn get_variable_data(&self, name: String) -> (String, i64, String) {
        match self.variables.iter().find(|x| x.0 == name) {
            Some(value) => (value.1.clone(), value.2, value.4.clone()),
            None => {
                let bt = Backtrace::capture();
                eprintln!("{}", bt);
                eprintln!("[BlockScope] Could not find variable named `{}`", name);
                process::exit(1);
            }
        }
    }
    pub fn get_variable_location(&self, name: String) -> (String, i64) {
        let data = self.get_variable_data(name);
        (data.2, -data.1)
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
        let struct_data = self.structs.iter().find(|x| x.0 == name);
        if let Some(data) = struct_data {
            return data.1.clone();
        }
        eprintln!("Cannot find struct named `{}`", name);
        process::exit(1);
    }
    pub fn get_struct_member_offset(&self, name: String, member: String) -> Result<i64, String> {
        let struct_data = self.get_struct_data(name.clone());
        let mut tot_off = 0;
        let mut found_result = false;
        for mem in struct_data.iter().rev() {
            if mem.0 == member {
                found_result = true;
                break;
            }
            tot_off += get_type_size(string_to_collapsed_type_tree(mem.1.clone(), self)?)? as i64;
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
    pub fn absorb_statics(&mut self, scope: ScopeContext) {
        for str in scope.strings {
            if self.strings.iter().any(|x| x.1 == str.1) {
                continue;
            }
            self.strings.push(str);
        }
        for stat in scope.variables.iter().filter(|x| x.3).cloned().collect::<Vec<(String, String, i64, bool, String)>>() {
            if self.variables.iter().any(|x| x.0 == stat.0) {
                continue;
            }
            self.variables.push(stat);
        }
        self.strings.sort_by(|a, b| a.1.cmp(&b.1));
    }
    pub fn add_string(&mut self, value: String) -> usize {
        let id = self.strings.len();
        self.strings.push((value, id));
        id
    }
    pub fn compile_data(&self) -> String {
        let mut data = String::new();

        for str in self.strings.clone() {
            data.push_str(&format!("\tSTR{} db \"{}\", 0\n", str.1, str.0));
        }

        for stat in self.variables.iter().filter(|x| x.3).cloned().collect::<Vec<(String, String, i64, bool, String)>>() {
            let collapsed = string_to_collapsed_type_tree(stat.1, self).unwrap();
            data.push_str(&format!("\t{}: resb {}\n", stat.4, get_type_size(collapsed).unwrap()));
        }

        data
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
    }
    pub fn absorb_stack(&mut self, _scope: ScopeContext) {
        // self.stack_size = std::cmp::max(self.stack_size, scope.stack_size);
    }
    pub fn absorb_sub_scope_globals(&mut self, sub_scope: ScopeContext) {
        self.absorb_statics(sub_scope.clone());
        self.absorb_functions(sub_scope.clone());
        self.absorb_labels(sub_scope.clone());
        self.absorb_stack(sub_scope.clone());
        self.absorb_structs(sub_scope.clone());
        self.absorb_types(sub_scope);
    }
}
