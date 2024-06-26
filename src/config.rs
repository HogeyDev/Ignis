#[derive(Debug, Clone)]
pub struct Configuration {
    pub main_file: String,
    pub root_path: String,
    pub imported_files: Vec<String>, // this is actually modified by the compiler. it means which
                                     // files that have already been imported to prevent re-definitions and overlap
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration {
            main_file: String::new(),
            root_path: String::new(),
            imported_files: Vec::new(),
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_config(main_file: String) -> Configuration {
    let root_path = std::env::current_dir()
        .expect("Could not get current working directory")
        .to_str()
        .unwrap()
        .to_string();
    Configuration {
        main_file,
        root_path,
        imported_files: Vec::new(),
    }
}
