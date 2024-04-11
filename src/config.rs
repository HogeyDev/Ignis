#[derive(Debug, Clone)]
pub struct Configuration {
    pub main_file: String,
    pub root_path: String,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration {
            main_file: String::new(),
            root_path: String::new(),
        }
    }
}

pub fn get_config() -> Configuration {
    let root_path = std::env::current_dir()
        .expect("Could not get current working directory")
        .to_str()
        .unwrap()
        .to_string();
    Configuration {
        main_file: String::from("example/hello_world.is"), // TODO: Hard-coded
        root_path,
    }
}
