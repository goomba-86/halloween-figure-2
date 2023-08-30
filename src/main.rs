pub mod file_io;
pub mod gpio;

use crate::file_io::FileIOImpl;
use crate::gpio::{Direction, RpiGpioController};

fn main() {
    let gpio_controller = RpiGpioController::new(FileIOImpl {}, Direction::Out, 10);
    println!("Hello, world!");
}
