use crate::gpio::{GpioController, PinValue};

pub struct LedController<T: GpioController> {
    pin: T,
    current_pin_value: PinValue,
}

impl<T> LedController<T>
where
    T: GpioController,
{
    pub fn new(pin: T) -> LedController<T> {
        LedController {
            pin,
            current_pin_value: PinValue::Low,
        }
    }

    pub fn turn_on(&self) -> std::io::Result<()> {
        self.pin.write(PinValue::High)
    }

    pub fn turn_off(&self) -> std::io::Result<()> {
        self.pin.write(PinValue::Low)
    }

    pub fn toggle(&self) -> std::io::Result<()> {
        match self.current_pin_value {
            PinValue::Low => self.turn_on(),
            PinValue::High => self.turn_off(),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use mockall::*;
//
//     #[test]
//     fn test_led_is_turned_on() {
//         let mut mock_gpio = crate::gpio::MockGpioController::new();
//         mock_gpio
//             .expect_write()
//             .with(predicate::eq(PinValue::High))
//             .returning(|_pin_value| Ok(()));
//         let led_controller = LedController::new(mock_gpio);
//         led_controller.turn_on();
//     }
// }
