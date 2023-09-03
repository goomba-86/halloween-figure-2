use mockall::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[automock]
pub trait FileIO {
    fn write(&self, file_path: &str, data: &str) -> std::io::Result<()>;
    fn read(&self, file_path: &str) -> std::io::Result<String>;
    fn exists(&self, path: &str) -> bool;
}

pub struct FileIOImpl {}

impl FileIO for FileIOImpl {
    fn write(&self, file_path: &str, data: &str) -> std::io::Result<()> {
        let file_result = File::create(file_path);
        let mut file = match file_result {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Unable to open file {file_path}. Error: {e}");
                return Err(e);
            }
        };
        match file.write_all(data.as_bytes()) {
            Ok(()) => Ok(()),
            Err(e) => {
                eprintln!("Unable to write file {file_path}. Error: {e}");
                return Err(e);
            }
        }
    }

    fn read(&self, file_path: &str) -> std::io::Result<String> {
        let file_result = File::open(file_path);
        let file = match file_result {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Unable to open file {file_path}. Error: {e}");
                return Err(e);
            }
        };
        let mut buffer_reader = BufReader::new(file);
        let mut contents = String::new();
        buffer_reader.read_to_string(&mut contents)?;
        Ok(contents)
    }

    fn exists(&self, path: &str) -> bool {
        Path::new(path).exists()
    }
}
