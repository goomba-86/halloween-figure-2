use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub trait FileIO {
    fn write(&self, file_path: &str, data: &str) -> std::io::Result<()>;
    fn read(&self, file_path: &str) -> std::io::Result<String>;
}

pub struct FileIOImpl {}

impl FileIO for FileIOImpl {
    fn write(&self, file_path: &str, data: &str) -> std::io::Result<()> {
        let mut file = File::open(file_path)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    fn read(&self, file_path: &str) -> std::io::Result<String> {
        let file = File::open(file_path)?;
        let mut buffer_reader = BufReader::new(file);
        let mut contents = String::new();
        buffer_reader.read_to_string(&mut contents)?;
        Ok(contents)
    }
}
