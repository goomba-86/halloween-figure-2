pub mod file_io;
pub mod gpio;
pub mod head_controller;
pub mod led_controller;
pub mod light_controller;
pub mod stepper_motor_controller;

use std::io::Result;
use std::sync::{Arc, Mutex};

use light_controller::{start_flickering_light, LightControllerParameters, LightFlickerTime};

use head_controller::{start_turning_head, HeadControllerParameters};

fn main() -> Result<()> {
    let stop_threads = Arc::new(Mutex::new(false));
    let head_controller_params = HeadControllerParameters {
        pins: vec![12, 16, 20, 21],
        speed: 5,
        degrees: 30,
        turn_wait_milli_seconds: 3000,
    };
    let _head_controller_handle =
        start_turning_head(head_controller_params, Arc::clone(&stop_threads));

    let light_controller_params = LightControllerParameters {
        flickers: vec![
            LightFlickerTime {
                on_time: 100,
                off_time: 500,
            },
            LightFlickerTime {
                on_time: 100,
                off_time: 500,
            },
            LightFlickerTime {
                on_time: 100,
                off_time: 100,
            },
            LightFlickerTime {
                on_time: 100,
                off_time: 100,
            },
            LightFlickerTime {
                on_time: 200,
                off_time: 2000,
            },
            LightFlickerTime {
                on_time: 100,
                off_time: 1000,
            },
            LightFlickerTime {
                on_time: 300,
                off_time: 1000,
            },
            LightFlickerTime {
                on_time: 500,
                off_time: 5000,
            },
        ],
    };
    let _light_controller_handle =
        start_flickering_light(light_controller_params, Arc::clone(&stop_threads));

    std::thread::sleep(std::time::Duration::from_secs(4 * 3600));

    Ok(())
}
