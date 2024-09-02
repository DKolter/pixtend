use crate::{error::PiXtendError, GpioConfig};
use deku::prelude::*;

#[derive(Debug, DekuRead, DekuWrite, Default)]
pub struct GpioOut {
    #[deku(pad_bits_before = "4")]
    #[deku(bits = "1")]
    pub gpio3: bool,
    #[deku(bits = "1")]
    pub gpio2: bool,
    #[deku(bits = "1")]
    pub gpio1: bool,
    #[deku(bits = "1")]
    pub gpio0: bool,
}

impl GpioOut {
    pub fn set_gpio_config(&mut self, index: u8, config: GpioConfig) -> Result<(), PiXtendError> {
        if index > 3 {
            return Err(PiXtendError::InvalidGpioOutputIndex(index));
        }

        // Only change the value if the config is an input with a pull-up resistor
        if config == GpioConfig::Input(true) {
            match index {
                0 => self.gpio0 = true,
                1 => self.gpio1 = true,
                2 => self.gpio2 = true,
                3 => self.gpio3 = true,
                _ => return Err(PiXtendError::InvalidGpioOutputIndex(index)),
            }
        }

        Ok(())
    }

    pub fn set_gpio_output(&mut self, index: u8, value: bool) -> Result<(), PiXtendError> {
        match index {
            0 => self.gpio0 = value,
            1 => self.gpio1 = value,
            2 => self.gpio2 = value,
            3 => self.gpio3 = value,
            _ => return Err(PiXtendError::InvalidGpioOutputIndex(index)),
        }

        Ok(())
    }
}

#[test]
fn test_gpio_out() {
    let data = vec![0b0000_1010];
    let (_, output) = GpioOut::from_bytes((data.as_ref(), 0)).unwrap();
    assert_eq!(output.gpio3, true);
    assert_eq!(output.gpio2, false);
    assert_eq!(output.gpio1, true);
    assert_eq!(output.gpio0, false);
    assert_eq!(output.to_bytes().unwrap(), data);

    let data = vec![0b0000_0110];
    let (_, output) = GpioOut::from_bytes((data.as_ref(), 0)).unwrap();
    assert_eq!(output.gpio3, false);
    assert_eq!(output.gpio2, true);
    assert_eq!(output.gpio1, true);
    assert_eq!(output.gpio0, false);
    assert_eq!(output.to_bytes().unwrap(), data);
}
