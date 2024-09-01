use crate::error::PiXtendError;
use deku::prelude::*;

#[derive(Debug, DekuRead, DekuWrite, Default)]
pub struct GpioDebounce {
    pub debounce01: u8,
    pub debounce23: u8,
}

impl GpioDebounce {
    pub fn set_gpio_debounce(&mut self, index: u8, value: u8) -> Result<(), PiXtendError> {
        match index {
            0 => self.debounce01 = value,
            1 => self.debounce23 = value,
            _ => return Err(PiXtendError::InvalidGpioDebounceGroup(index)),
        }

        Ok(())
    }
}
