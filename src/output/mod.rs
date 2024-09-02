use deku::prelude::*;
use digital_debounce::DigitalDebounce;
use digital_out::DigitalOut;
use gpio_ctrl::GpioCtrl;
use gpio_debounce::GpioDebounce;
use gpio_out::GpioOut;
use pwm::Pwm;
use relay_out::RelayOut;
use retain::Retain;
use system::System;
pub use watchdog::Watchdog;

use crate::utils::calc_crc16;

mod digital_debounce;
mod digital_out;
mod gpio_ctrl;
mod gpio_debounce;
mod gpio_out;
mod pwm;
mod relay_out;
mod retain;
mod system;
mod watchdog;

#[derive(Debug, DekuRead, DekuWrite, Default)]
pub struct Output {
    pub header: Header,
    #[deku(endian = "little", update = "self.calculate_header_crc()")]
    header_crc: u16,
    pub data: Data,
    #[deku(endian = "little", update = "self.calculate_data_crc()")]
    data_crc: u16,
}

#[derive(Debug, DekuRead, DekuWrite, Default)]
#[deku(magic = b"L")]
pub struct Header {
    #[deku(pad_bytes_before = "1")]
    pub watchdog: Watchdog,
    #[deku(pad_bytes_after = "3")]
    pub system: System,
}

#[derive(Debug, DekuRead, DekuWrite, Default)]
pub struct Data {
    pub digital_debounce: DigitalDebounce,
    pub digital_out: DigitalOut,
    pub relay_out: RelayOut,
    pub gpio_ctrl: GpioCtrl,
    pub gpio_out: GpioOut,
    pub gpio_debounce: GpioDebounce,
    pub pwm: Pwm,
    pub retain: Retain,
}

impl Output {
    fn calculate_header_crc(&self) -> u16 {
        calc_crc16(self.header.to_bytes().into_iter().flatten())
    }

    fn calculate_data_crc(&self) -> u16 {
        calc_crc16(self.data.to_bytes().into_iter().flatten())
    }
}

#[test]
fn test_output() {
    let output = Output::default();
    assert_eq!(output.to_bytes().unwrap().len(), 111);
}
