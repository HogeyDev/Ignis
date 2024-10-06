use std::{fs, process};

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub path: String,
    pub contents: String,
}

pub fn read_file(file_path: String) -> SourceFile {
    let contents = fs::read_to_string(file_path.clone()); 
    if contents.is_err() {
        eprintln!("Cannot find file `{file_path}`");
        process::exit(1);
    }
    SourceFile {
        path: file_path,
        contents: contents.unwrap(),
    }
}

pub fn write_file(file: SourceFile) {
    fs::write(file.path, file.contents).expect("Could not open file to write.");
}
