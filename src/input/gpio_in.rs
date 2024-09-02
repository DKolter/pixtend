use crate::error::PiXtendError;
use deku::prelude::*;

#[derive(Debug, DekuWrite, DekuRead)]
pub struct GpioIn {
    #[deku(pad_bits_before = "4")]
    #[deku(bits = "1")]
    pub in3: bool,
    #[deku(bits = "1")]
    pub in2: bool,
    #[deku(bits = "1")]
    pub in1: bool,
    #[deku(bits = "1")]
    pub in0: bool,
}

impl GpioIn {
    pub fn get_gpio_input(&self, index: u8) -> Result<bool, PiXtendError> {
        match index {
            0 => Ok(self.in0),
            1 => Ok(self.in1),
            2 => Ok(self.in2),
            3 => Ok(self.in3),
            _ => Err(PiXtendError::InvalidGpioInputIndex(index)),
        }
    }
}

#[test]
fn test_gpio_in() {
    let data = [0b0000_1010];
    let (_, gpio_in) = GpioIn::from_bytes((data.as_ref(), 0)).unwrap();
    assert_eq!(gpio_in.in3, true);
    assert_eq!(gpio_in.in2, false);
    assert_eq!(gpio_in.in1, true);
    assert_eq!(gpio_in.in0, false);
    assert_eq!(gpio_in.to_bytes().unwrap(), data);

    let data = [0b0000_0110];
    let (_, gpio_in) = GpioIn::from_bytes((data.as_ref(), 0)).unwrap();
    assert_eq!(gpio_in.in3, false);
    assert_eq!(gpio_in.in2, true);
    assert_eq!(gpio_in.in1, true);
    assert_eq!(gpio_in.in0, false);
    assert_eq!(gpio_in.to_bytes().unwrap(), data);
}
