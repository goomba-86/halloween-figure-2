use crate::input_output::InputOutput;

pub struct FileIO {
    pub file_path: String,
}

impl InputOutput for FileIO {
    fn write(&self) -> () {}

    fn read(&self) -> String {
        String::from("")
    }
}
