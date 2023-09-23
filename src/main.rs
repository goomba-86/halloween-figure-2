pub mod file_io;
pub mod gpio;
pub mod led_controller;
pub mod stepper_motor_controller;

use crate::file_io::FileIOImpl;
use crate::gpio::{Direction, RpiGpioController};
use crate::led_controller::LedController;
use crate::stepper_motor_controller::StepperMotorController;
use std::io::Result;
use std::{thread, time};

fn main() -> Result<()> {
    let pins = vec![
        RpiGpioController::new(FileIOImpl {}, Direction::Out, 12).unwrap(),
        RpiGpioController::new(FileIOImpl {}, Direction::Out, 16).unwrap(),
        RpiGpioController::new(FileIOImpl {}, Direction::Out, 20).unwrap(),
        RpiGpioController::new(FileIOImpl {}, Direction::Out, 21).unwrap(),
    ];

    let stepper_motor = StepperMotorController::new(pins, 5);
    let turn_wait_milli_seconds = time::Duration::from_millis(3000);

    thread::spawn(|| {
        let led_controller =
            LedController::new(RpiGpioController::new(FileIOImpl {}, Direction::Out, 13).unwrap());
        let led_blink_interval = time::Duration::from_millis(500);
        loop {
            led_controller.turn_on().unwrap();
            thread::sleep(led_blink_interval);
            led_controller.turn_off().unwrap_or_default();
            thread::sleep(led_blink_interval);
        }
    });

    loop {
        stepper_motor.turn_degrees(30)?;
        thread::sleep(turn_wait_milli_seconds);
        stepper_motor.turn_degrees(-30)?;
        thread::sleep(turn_wait_milli_seconds);
    }
}
