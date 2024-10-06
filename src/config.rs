use crate::cli::CliParser;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub main_file: String,
    pub root_path: String,
    pub imported_files: Vec<String>, // this is actually modified by the compiler. it describes the files that have already been imported to prevent re-definitions and overlap
    pub debug_tokens: bool,
    pub debug_ast: bool,
    pub debug_asm: bool,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration {
            main_file: String::new(),
            root_path: String::new(),
            imported_files: Vec::new(),
            debug_tokens: false,
            debug_asm: false,
            debug_ast: false,
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_config(main_file: String, cli: &CliParser) -> Configuration {
    let root_path = std::env::current_dir()
        .expect("Could not get current working directory")
        .to_str()
        .unwrap()
        .to_string();
    let mut config = Configuration::new();
    config.main_file = main_file;
    config.root_path = root_path;
    config.debug_tokens = cli.flag_value("debug-tokens");
    config.debug_ast = cli.flag_value("debug-ast");
    config.debug_asm = cli.flag_value("debug-asm");
    config
}
