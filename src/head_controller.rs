use crate::file_io::FileIOImpl;
use crate::gpio::{Direction, RpiGpioController};
use crate::stepper_motor_controller::StepperMotorController;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

pub struct HeadControllerParameters {
    pub pins: Vec<u8>,
    pub speed: u64,
    pub degrees: i32,
    pub turn_wait_milli_seconds: u64,
}

pub fn start_turning_head(
    params: HeadControllerParameters,
    stop_thread: Arc<Mutex<bool>>,
) -> JoinHandle<()> {
    let gpio_controllers = params
        .pins
        .iter()
        .map(|&pin| RpiGpioController::new(FileIOImpl {}, Direction::Out, pin).unwrap())
        .collect::<Vec<_>>();

    let stepper_motor = StepperMotorController::new(gpio_controllers, params.speed);
    let turn_wait = std::time::Duration::from_millis(params.turn_wait_milli_seconds);
    std::thread::spawn(move || {
        while !*stop_thread.lock().unwrap() {
            stepper_motor.turn_degrees(params.degrees).unwrap();
            std::thread::sleep(turn_wait);
            stepper_motor.turn_degrees(-params.degrees).unwrap();
            std::thread::sleep(turn_wait);
        }
        println!("Head controlling thread stopped");
    })
}
