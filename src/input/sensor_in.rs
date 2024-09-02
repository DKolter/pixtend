use crate::error::PiXtendError;
use deku::prelude::*;

#[derive(Debug, DekuWrite, DekuRead)]
pub struct SensorIn {
    pub sens0: Sensor,
    pub sens1: Sensor,
    pub sens2: Sensor,
    pub sens3: Sensor,
}

#[derive(Debug, DekuWrite, DekuRead)]
pub struct Sensor {
    #[deku(endian = "little")]
    pub temperature: u16,
    #[deku(endian = "little")]
    pub humidity: u16,
}

impl SensorIn {
    pub fn get_temperature_input(
        &self,
        index: u8,
        sensor: SensorKind,
    ) -> Result<f64, PiXtendError> {
        // A dht22 can be negative when the msb is set
        let negative = match sensor {
            SensorKind::DHT11 => false,
            SensorKind::DHT22 => match index {
                0 => self.sens0.temperature & 0x8000 != 0,
                1 => self.sens1.temperature & 0x8000 != 0,
                2 => self.sens2.temperature & 0x8000 != 0,
                3 => self.sens3.temperature & 0x8000 != 0,
                _ => return Err(PiXtendError::InvalidGpioInputIndex(index)),
            },
        };

        let factor = match negative {
            true => -1.0,
            false => 1.0,
        };

        let div = match sensor {
            SensorKind::DHT11 => 256.0,
            SensorKind::DHT22 => 10.0,
        };

        match index {
            0 => Ok((self.sens0.temperature & 0x7FFF) as f64 / div * factor),
            1 => Ok((self.sens1.temperature & 0x7FFF) as f64 / div * factor),
            2 => Ok((self.sens2.temperature & 0x7FFF) as f64 / div * factor),
            3 => Ok((self.sens3.temperature & 0x7FFF) as f64 / div * factor),
            _ => Err(PiXtendError::InvalidGpioInputIndex(index)),
        }
    }

    pub fn get_humidity_input(&self, index: u8, sensor: SensorKind) -> Result<f64, PiXtendError> {
        let div = match sensor {
            SensorKind::DHT11 => 25600.0,
            SensorKind::DHT22 => 1000.0,
        };

        match index {
            0 => Ok(self.sens0.humidity as f64 / div),
            1 => Ok(self.sens1.humidity as f64 / div),
            2 => Ok(self.sens2.humidity as f64 / div),
            3 => Ok(self.sens3.humidity as f64 / div),
            _ => Err(PiXtendError::InvalidGpioInputIndex(index)),
        }
    }
}

pub enum SensorKind {
    DHT11,
    DHT22,
}
