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
            Direction::Out => self.file_io.write(
                &format!(
                    "{}/gpio{}/direction",
                    RpiGpioController::GPIO_SYSFS_PATH,
                    self.pin_number
                ),
                "out",
            )?,
            Direction::In => self.file_io.write(
                &format!(
                    "{}/gpio{}/direction",
                    RpiGpioController::GPIO_SYSFS_PATH,
                    self.pin_number
                ),
                "in",
            )?,
        }
        Ok(())
    }
}

impl GpioController for RpiGpioController {
    fn write(&self, pin_value: PinValue) -> std::io::Result<()> {
        Ok(())
    }

    fn read(&self) -> std::io::Result<PinValue> {
        Ok(PinValue::High)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::*;

    #[test]
    fn test_create_new_gpio_controller() {
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

        let gpio_controller = RpiGpioController::new(Box::new(mock_file_io), Direction::Out, 1);
        assert!(gpio_controller.is_ok());
    }
}
