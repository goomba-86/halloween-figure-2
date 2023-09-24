use crate::file_io::FileIOImpl;
use crate::gpio::{Direction, RpiGpioController};
use crate::led_controller::LedController;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

pub struct LightControllerParameters {
    pub flickers: Vec<u64>,
}

pub fn start_flickering_light(
    params: LightControllerParameters,
    stop_thread: Arc<Mutex<bool>>,
) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let led_controller =
            LedController::new(RpiGpioController::new(FileIOImpl {}, Direction::Out, 13).unwrap());
        let mut flicker_index = 0;
        while !*stop_thread.lock().unwrap() {
            led_controller.turn_on().unwrap_or_default();
            std::thread::sleep(Duration::from_millis(params.flickers[flicker_index]));
            led_controller.turn_off().unwrap_or_default();
            std::thread::sleep(Duration::from_millis(params.flickers[flicker_index]) / 2);

            flicker_index += 1;
            if flicker_index >= params.flickers.len() {
                flicker_index = 0;
            }
        }
        println!("Light controller thread stopped.");
    })
}
