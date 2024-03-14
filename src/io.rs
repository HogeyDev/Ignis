use std::fs;

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub path: String,
    pub contents: String,
}

pub fn read_file(file_path: String) -> SourceFile {
    SourceFile {
        path: file_path.clone(),
        contents: fs::read_to_string(file_path).expect("Could not open file to read."),
    }
}

pub fn write_file(file: SourceFile) {
    fs::write(file.path, file.contents).expect("Could not open file to write.");
}
