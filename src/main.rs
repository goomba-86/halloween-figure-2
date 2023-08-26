pub mod file_io;
pub mod input_output;

use crate::file_io::FileIO;

fn main() {
    let file_io = FileIO {
        file_path: String::from("test"),
    };
    println!("Hello, world!");
}
