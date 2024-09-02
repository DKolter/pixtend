use crate::error::PiXtendError;
use deku::prelude::*;

#[derive(Debug, DekuWrite, DekuRead)]
pub struct AnalogIn {
    #[deku(endian = "little")]
    pub in0: u16,
    #[deku(endian = "little")]
    pub in1: u16,
    #[deku(endian = "little")]
    pub in2: u16,
    #[deku(endian = "little")]
    pub in3: u16,
    #[deku(endian = "little")]
    pub in4: u16,
    #[deku(endian = "little")]
    pub in5: u16,
}

impl AnalogIn {
    pub fn get_analog_voltage_input(
        &self,
        index: u8,
        reference_voltage: ReferenceVoltage,
    ) -> Result<f64, PiXtendError> {
        let reference_voltage = match reference_voltage {
            ReferenceVoltage::V5 => 5.0,
            ReferenceVoltage::V10 => 10.0,
        };
        match index {
            0 => Ok(self.in0 as f64 * reference_voltage / 1024.0),
            1 => Ok(self.in1 as f64 * reference_voltage / 1024.0),
            2 => Ok(self.in2 as f64 * reference_voltage / 1024.0),
            3 => Ok(self.in3 as f64 * reference_voltage / 1024.0),
            _ => Err(PiXtendError::InvalidAnalogVoltageInputIndex(index)),
        }
    }

    pub fn get_analog_current_input(&self, index: u8) -> Result<f64, PiXtendError> {
        match index {
            4 => Ok(self.in4 as f64 * 0.020158400229358),
            5 => Ok(self.in5 as f64 * 0.020158400229358),
            _ => Err(PiXtendError::InvalidAnalogCurrentInputIndex(index)),
        }
    }
}

/// Reference voltage for analog inputs
pub enum ReferenceVoltage {
    /// 0V to 5V
    V5,
    /// 0V to 10V
    V10,
}
