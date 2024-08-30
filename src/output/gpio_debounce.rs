use deku::prelude::*;

#[derive(Debug, DekuRead, DekuWrite, Default)]
pub struct GpioDebounce {
    pub debounce01: u8,
    pub debounce23: u8,
}
