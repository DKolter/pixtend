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
