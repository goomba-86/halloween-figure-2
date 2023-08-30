pub mod file_io;
pub mod gpio;
pub mod stepper_motor_controller;

use crate::file_io::FileIOImpl;
use crate::gpio::{Direction, RpiGpioController};

fn main() {
    let gpio_controller = RpiGpioController::new(FileIOImpl {}, Direction::Out, 10);
    println!("Hello, world!");
}
