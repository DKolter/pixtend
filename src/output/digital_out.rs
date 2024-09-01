use crate::error::PiXtendError;
use deku::prelude::*;

#[derive(Debug, DekuRead, DekuWrite, Default)]
pub struct DigitalOut {
    #[deku(bits = "1")]
    pub out7: bool,
    #[deku(bits = "1")]
    pub out6: bool,
    #[deku(bits = "1")]
    pub out5: bool,
    #[deku(bits = "1")]
    pub out4: bool,
    #[deku(bits = "1")]
    pub out3: bool,
    #[deku(bits = "1")]
    pub out2: bool,
    #[deku(bits = "1")]
    pub out1: bool,
    #[deku(bits = "1")]
    pub out0: bool,
    #[deku(pad_bits_before = "4")]
    #[deku(bits = "1")]
    pub out11: bool,
    #[deku(bits = "1")]
    pub out10: bool,
    #[deku(bits = "1")]
    pub out9: bool,
    #[deku(bits = "1")]
    pub out8: bool,
}

impl DigitalOut {
    pub fn set_digital_output(&mut self, index: u8, value: bool) -> Result<(), PiXtendError> {
        match index {
            0 => self.out0 = value,
            1 => self.out1 = value,
            2 => self.out2 = value,
            3 => self.out3 = value,
            4 => self.out4 = value,
            5 => self.out5 = value,
            6 => self.out6 = value,
            7 => self.out7 = value,
            8 => self.out8 = value,
            9 => self.out9 = value,
            10 => self.out10 = value,
            11 => self.out11 = value,
            _ => return Err(PiXtendError::InvalidDigitalOutputIndex(index)),
        }

        Ok(())
    }
}

#[test]
fn test_digital_out_control() {
    let data = [0b1010_1010, 0b0000_1010];
    let (_, digital_out_control) = DigitalOut::from_bytes((&data, 0)).unwrap();
    assert_eq!(digital_out_control.out0, false);
    assert_eq!(digital_out_control.out1, true);
    assert_eq!(digital_out_control.out2, false);
    assert_eq!(digital_out_control.out3, true);
    assert_eq!(digital_out_control.out4, false);
    assert_eq!(digital_out_control.out5, true);
    assert_eq!(digital_out_control.out6, false);
    assert_eq!(digital_out_control.out7, true);
    assert_eq!(digital_out_control.out8, false);
    assert_eq!(digital_out_control.out9, true);
    assert_eq!(digital_out_control.out10, false);
    assert_eq!(digital_out_control.out11, true);

    let data = digital_out_control.to_bytes().unwrap();
    assert_eq!(data, [0b1010_1010, 0b0000_1010]);
}
