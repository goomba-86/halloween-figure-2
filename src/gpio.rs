use crate::file_io::FileIO;
use mockall::*;
use std::{thread, time};

pub enum Direction {
    In,
    Out,
}

pub enum PinValue {
    High,
    Low,
}

#[automock]
pub trait GpioController {
    fn write(&self, pin_value: PinValue) -> std::io::Result<()>;
    fn read(&self) -> std::io::Result<PinValue>;
}

pub struct RpiGpioController<T: FileIO> {
    file_io: T,
    pub pin_number: u16,
}

impl<T> RpiGpioController<T>
where
    T: FileIO,
{
    const GPIO_SYSFS_PATH: &str = "/sys/class/gpio";

    pub fn new(
        file_io: T,
        direction: Direction,
        pin_number: u16,
    ) -> std::io::Result<RpiGpioController<T>> {
        let rpi_gpio_controller = RpiGpioController {
            file_io,
            pin_number,
        };

        rpi_gpio_controller.export()?;
        Self::sleep(50);
        rpi_gpio_controller.set_direction(direction)?;
        Self::sleep(50);
        Ok(rpi_gpio_controller)
    }

    fn export(&self) -> std::io::Result<()> {
        if self.file_io.exists(&self.gpio_pin_path()) {
            return Ok(());
        }

        self.file_io.write(
            &format!("{}/export", RpiGpioController::<T>::GPIO_SYSFS_PATH),
            &self.pin_number.to_string(),
        )?;
        Ok(())
    }

    fn set_direction(&self, direction: Direction) -> std::io::Result<()> {
        match direction {
            Direction::Out => self
                .file_io
                .write(&format!("{}/direction", self.gpio_pin_path()), "out")?,
            Direction::In => self
                .file_io
                .write(&format!("{}/direction", self.gpio_pin_path()), "in")?,
        }
        Ok(())
    }

    fn gpio_pin_path(&self) -> String {
        format!(
            "{}/gpio{}",
            RpiGpioController::<T>::GPIO_SYSFS_PATH,
            self.pin_number
        )
    }

    fn sleep(duration_milli_seconds: u64) {
        thread::sleep(time::Duration::from_millis(duration_milli_seconds))
    }
}

impl<T> GpioController for RpiGpioController<T>
where
    T: FileIO,
{
    fn write(&self, pin_value: PinValue) -> std::io::Result<()> {
        match pin_value {
            PinValue::High => self
                .file_io
                .write(&format!("{}/value", self.gpio_pin_path()), "1"),
            PinValue::Low => self
                .file_io
                .write(&format!("{}/value", self.gpio_pin_path()), "0"),
        }
    }

    fn read(&self) -> std::io::Result<PinValue> {
        Ok(PinValue::High)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind};

    #[test]
    fn test_create_new_gpio_controller() {
        let mock_file_io = set_up_file_io_initialization_mocks();
        let gpio_controller = RpiGpioController::new(mock_file_io, Direction::Out, 1);
        assert!(gpio_controller.is_ok());
    }

    #[test]
    fn test_pin_export_fails_in_creation() {
        let mut mock_file_io = crate::file_io::MockFileIO::new();
        mock_file_io
            .expect_write()
            .with(predicate::eq("/sys/class/gpio/export"), predicate::eq("1"))
            .returning(|_file_path, _data| Err(Error::new(ErrorKind::Other, "File not found.")));

        let gpio_controller = RpiGpioController::new(mock_file_io, Direction::Out, 1);
        assert!(gpio_controller.is_err());
    }

    #[test]
    fn test_that_high_pin_value_is_written_correctly() {
        let mut mock_file_io = set_up_file_io_initialization_mocks();
        mock_file_io
            .expect_write()
            .with(
                predicate::eq("/sys/class/gpio/gpio1/value"),
                predicate::eq("1"),
            )
            .returning(|_file_path, _data| Ok(()));
        let gpio_controller = RpiGpioController::new(mock_file_io, Direction::Out, 1).unwrap();

        let _ = gpio_controller.write(PinValue::High);
    }

    #[test]
    fn test_that_low_pin_value_is_written_correctly() {
        let mut mock_file_io = set_up_file_io_initialization_mocks();
        mock_file_io
            .expect_write()
            .with(
                predicate::eq("/sys/class/gpio/gpio1/value"),
                predicate::eq("0"),
            )
            .returning(|_file_path, _data| Ok(()));
        let gpio_controller = RpiGpioController::new(mock_file_io, Direction::Out, 1).unwrap();

        let _ = gpio_controller.write(PinValue::Low);
    }

    fn set_up_file_io_initialization_mocks() -> crate::file_io::MockFileIO {
        let mut mock_file_io = crate::file_io::MockFileIO::new();
        mock_file_io
            .expect_write()
            .with(predicate::eq("/sys/class/gpio/export"), predicate::eq("1"))
            .returning(|_file_path, _data| Ok(()));
        mock_file_io
            .expect_write()
            .with(
                predicate::eq("/sys/class/gpio/gpio1/direction"),
                predicate::eq("out"),
            )
            .returning(|_file_path, _data| Ok(()));
        mock_file_io
    }
}
