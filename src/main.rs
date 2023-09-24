pub mod file_io;
pub mod gpio;
pub mod head_controller;
pub mod led_controller;
pub mod light_controller;
pub mod stepper_motor_controller;

use std::io::Result;
use std::sync::{Arc, Mutex};

use light_controller::{start_flickering_light, LightControllerParameters};

use head_controller::{start_turning_head, HeadControllerParameters};

fn main() -> Result<()> {
    let stop_threads = Arc::new(Mutex::new(false));
    let head_controller_params = HeadControllerParameters {
        pins: vec![12, 16, 20, 21],
        speed: 5,
        degrees: 30,
        turn_wait_milli_seconds: 3000,
    };
    let head_controller_handle =
        start_turning_head(head_controller_params, Arc::clone(&stop_threads));

    let light_controller_params = LightControllerParameters {
        flickers: vec![200, 100, 100, 500, 2000, 200, 1000, 3000],
    };
    let light_controller_handle =
        start_flickering_light(light_controller_params, Arc::clone(&stop_threads));

    std::thread::sleep(std::time::Duration::from_millis(20000));

    println!("Sending stop signal to threads...");
    let mut stop = stop_threads.lock().unwrap();
    *stop = true;
    println!("Signal sent");

    head_controller_handle.join().unwrap();
    light_controller_handle.join().unwrap();

    println!("Both threads joined to main.");

    Ok(())
}
