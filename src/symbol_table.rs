use std::collections::HashMap;

type Entry = ();
#[derive(Debug)]
pub struct SymbolTable {
    entries: HashMap<String, Entry>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            entries: HashMap::new(),
        }
    }

    pub fn get(&self, name: &String) -> Option<&Entry> {
        self.entries.get(name)
    }
    pub fn register(&mut self, name: String, value: Entry) {
        self.entries.insert(name, value);
    }
}
