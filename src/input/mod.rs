use crate::utils::calc_crc16;
use analog_in::AnalogIn;
use deku::prelude::*;
use digital_in::DigitalIn;
use gpio_in::GpioIn;
use sensor_in::SensorIn;
use state::State;

mod analog_in;
mod digital_in;
mod gpio_in;
mod sensor_in;
mod state;
mod warnings;

pub use analog_in::ReferenceVoltage;
pub use sensor_in::SensorKind;
pub use state::ErrorCode;
pub use warnings::Warnings;

#[derive(Debug, DekuWrite, DekuRead)]
pub struct Input {
    pub header: Header,
    #[deku(endian = "little")]
    header_crc: u16,
    pub data: Data,
    #[deku(endian = "little")]
    data_crc: u16,
}

impl Input {
    pub fn check_crc_valid(&self) -> bool {
        let header_crc = calc_crc16(self.header.to_bytes().into_iter().flatten());
        let data_crc = calc_crc16(self.data.to_bytes().into_iter().flatten());
        header_crc == self.header_crc && data_crc == self.data_crc
    }
}

#[derive(Debug, DekuWrite, DekuRead)]
pub struct Header {
    pub firmware: u8,
    pub hardware: u8,
    pub model: u8,
    pub state: State,
    #[deku(pad_bytes_after = "2")]
    pub warnings: Warnings,
}

#[derive(Debug, DekuWrite, DekuRead)]
pub struct Data {
    pub digital_in: DigitalIn,
    pub analog_in: AnalogIn,
    pub gpio_in: GpioIn,
    #[deku(pad_bytes_after = "5")]
    pub sensor_in: SensorIn,
    #[deku(count = "64")]
    pub retain: Vec<u8>,
}
