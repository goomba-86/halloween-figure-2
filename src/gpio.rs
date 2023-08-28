use crate::file_io::FileIO;

pub enum Direction {
    In,
    Out,
}

pub enum PinValue {
    High,
    Low,
}

pub trait GpioController {
    fn write(&self, pin_value: PinValue) -> std::io::Result<()>;
    fn read(&self) -> std::io::Result<PinValue>;
}

pub struct RpiGpioController {
    file_io: Box<dyn FileIO>,
    pub pin_number: u8,
}

impl RpiGpioController {
    const GPIO_SYSFS_PATH: &str = "/sys/class/gpio";

    pub fn new(
        file_io: Box<dyn FileIO>,
        direction: Direction,
        pin_number: u8,
    ) -> std::io::Result<RpiGpioController> {
        let rpi_gpio_controller = RpiGpioController {
            file_io,
            pin_number,
        };

        rpi_gpio_controller.export()?;
        rpi_gpio_controller.set_direction(direction)?;
        Ok(rpi_gpio_controller)
    }

    fn export(&self) -> std::io::Result<()> {
        self.file_io.write(
            &format!("{}/export", RpiGpioController::GPIO_SYSFS_PATH),
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
            RpiGpioController::GPIO_SYSFS_PATH,
            self.pin_number
        )
    }
}

impl GpioController for RpiGpioController {
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
    use mockall::*;
    use std::io::{Error, ErrorKind};

    #[test]
    fn test_create_new_gpio_controller() {
        let mock_file_io = set_up_file_io_initialization_mocks();
        let gpio_controller = RpiGpioController::new(Box::new(mock_file_io), Direction::Out, 1);
        assert!(gpio_controller.is_ok());
    }

    #[test]
    fn test_pin_export_fails_in_creation() {
        let mut mock_file_io = crate::file_io::MockFileIO::new();
        mock_file_io
            .expect_write()
            .with(predicate::eq("/sys/class/gpio/export"), predicate::eq("1"))
            .returning(|_file_path, _data| Err(Error::new(ErrorKind::Other, "File not found.")));

        let gpio_controller = RpiGpioController::new(Box::new(mock_file_io), Direction::Out, 1);
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
        let gpio_controller =
            RpiGpioController::new(Box::new(mock_file_io), Direction::Out, 1).unwrap();

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
        let gpio_controller =
            RpiGpioController::new(Box::new(mock_file_io), Direction::Out, 1).unwrap();

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
