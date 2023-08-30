use crate::gpio::{GpioController, PinValue};
use std::io::Result;
use std::{thread, time};

pub struct StepperMotorController<T: GpioController> {
    pins: Vec<T>,
    duration_milli_seconds: time::Duration,
}

impl<T> StepperMotorController<T>
where
    T: GpioController,
{
    pub fn new(pins: Vec<T>, duration_milli_seconds: u64) -> StepperMotorController<T> {
        let duration_milli_seconds = time::Duration::from_millis(duration_milli_seconds);
        StepperMotorController {
            pins,
            duration_milli_seconds,
        }
    }

    pub fn turn(&self, count: i32) -> Result<()> {
        if count > 0 {
            for _ in 0..count {
                self.do_step1()?;
                self.do_step2()?;
                self.do_step3()?;
                self.do_step4()?;
                self.do_step5()?;
                self.do_step6()?;
                self.do_step7()?;
                self.do_step8()?;
            }
        } else {
            for _ in 0..count {
                self.do_step8()?;
                self.do_step7()?;
                self.do_step6()?;
                self.do_step5()?;
                self.do_step4()?;
                self.do_step3()?;
                self.do_step2()?;
                self.do_step1()?;
            }
        }
        Ok(())
    }

    pub fn turn_degrees(&self, degrees: i32) -> Result<()> {
        self.turn(degrees * 512 / 360)
    }

    fn do_step1(&self) -> Result<()> {
        self.pins[3].write(PinValue::High)?;
        self.sleep();
        self.pins[3].write(PinValue::Low)
    }

    fn do_step2(&self) -> Result<()> {
        self.pins[3].write(PinValue::High)?;
        self.pins[2].write(PinValue::High)?;
        self.sleep();
        self.pins[3].write(PinValue::Low)?;
        self.pins[2].write(PinValue::Low)
    }

    fn do_step3(&self) -> Result<()> {
        self.pins[2].write(PinValue::High)?;
        self.sleep();
        self.pins[2].write(PinValue::Low)
    }

    fn do_step4(&self) -> Result<()> {
        self.pins[2].write(PinValue::High)?;
        self.pins[1].write(PinValue::High)?;
        self.sleep();
        self.pins[2].write(PinValue::Low)?;
        self.pins[1].write(PinValue::Low)
    }

    fn do_step5(&self) -> Result<()> {
        self.pins[1].write(PinValue::High)?;
        self.sleep();
        self.pins[1].write(PinValue::Low)
    }

    fn do_step6(&self) -> Result<()> {
        self.pins[1].write(PinValue::High)?;
        self.pins[0].write(PinValue::High)?;
        self.sleep();
        self.pins[1].write(PinValue::Low)?;
        self.pins[0].write(PinValue::Low)
    }

    fn do_step7(&self) -> Result<()> {
        self.pins[0].write(PinValue::High)?;
        self.sleep();
        self.pins[0].write(PinValue::Low)
    }

    fn do_step8(&self) -> Result<()> {
        self.pins[3].write(PinValue::High)?;
        self.pins[0].write(PinValue::High)?;
        self.sleep();
        self.pins[3].write(PinValue::Low)?;
        self.pins[0].write(PinValue::Low)
    }

    fn sleep(&self) {
        thread::sleep(self.duration_milli_seconds)
    }
}
