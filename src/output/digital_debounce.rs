use crate::error::PiXtendError;
use deku::prelude::*;

#[derive(Debug, DekuRead, DekuWrite, Default)]
pub struct DigitalDebounce {
    pub debounce01: u8,
    pub debounce23: u8,
    pub debounce45: u8,
    pub debounce67: u8,
    pub debounce89: u8,
    pub debounce1011: u8,
    pub debounce1213: u8,
    pub debounce1415: u8,
}

impl DigitalDebounce {
    pub fn set_digital_debounce(&mut self, index: u8, value: u8) -> Result<(), PiXtendError> {
        match index {
            0 => self.debounce01 = value,
            1 => self.debounce23 = value,
            2 => self.debounce45 = value,
            3 => self.debounce67 = value,
            4 => self.debounce89 = value,
            5 => self.debounce1011 = value,
            6 => self.debounce1213 = value,
            7 => self.debounce1415 = value,
            _ => return Err(PiXtendError::InvalidDigitalDebounceGroup(index)),
        }

        Ok(())
    }
}
