#[derive(Debug)]
pub struct Configuration {
    pub import_path_priority: Vec<String>,
}

impl Configuration {
    pub fn new() -> Self {
        Self {
            import_path_priority: Vec::new(),
        }
    }
}
