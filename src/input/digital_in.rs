use crate::error::PiXtendError;
use deku::prelude::*;

#[derive(Debug, DekuWrite, DekuRead)]
pub struct DigitalIn {
    #[deku(bits = "1")]
    pub in7: bool,
    #[deku(bits = "1")]
    pub in6: bool,
    #[deku(bits = "1")]
    pub in5: bool,
    #[deku(bits = "1")]
    pub in4: bool,
    #[deku(bits = "1")]
    pub in3: bool,
    #[deku(bits = "1")]
    pub in2: bool,
    #[deku(bits = "1")]
    pub in1: bool,
    #[deku(bits = "1")]
    pub in0: bool,
    #[deku(bits = "1")]
    pub in15: bool,
    #[deku(bits = "1")]
    pub in14: bool,
    #[deku(bits = "1")]
    pub in13: bool,
    #[deku(bits = "1")]
    pub in12: bool,
    #[deku(bits = "1")]
    pub in11: bool,
    #[deku(bits = "1")]
    pub in10: bool,
    #[deku(bits = "1")]
    pub in9: bool,
    #[deku(bits = "1")]
    pub in8: bool,
}

impl DigitalIn {
    pub fn get_digital_input(&self, index: u8) -> Result<bool, PiXtendError> {
        match index {
            0 => Ok(self.in0),
            1 => Ok(self.in1),
            2 => Ok(self.in2),
            3 => Ok(self.in3),
            4 => Ok(self.in4),
            5 => Ok(self.in5),
            6 => Ok(self.in6),
            7 => Ok(self.in7),
            8 => Ok(self.in8),
            9 => Ok(self.in9),
            10 => Ok(self.in10),
            11 => Ok(self.in11),
            12 => Ok(self.in12),
            13 => Ok(self.in13),
            14 => Ok(self.in14),
            15 => Ok(self.in15),
            _ => Err(PiXtendError::InvalidDigitalInputIndex(index)),
        }
    }
}
