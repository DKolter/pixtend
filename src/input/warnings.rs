use deku::prelude::*;

#[derive(Debug, DekuWrite, DekuRead, Clone, Copy)]
pub struct Warnings {
    #[deku(pad_bits_before = "4")]
    #[deku(bits = "1")]
    pub i2c_error: bool,
    #[deku(bits = "1")]
    pub voltage_error: bool,
    #[deku(pad_bits_after = "1")]
    #[deku(bits = "1")]
    pub retain_crc_error: bool,
}

#[test]
fn test_warnings() {
    let data = [0b0000_1010];
    let (_, warnings) = Warnings::from_bytes((data.as_ref(), 0)).unwrap();
    assert_eq!(warnings.i2c_error, true);
    assert_eq!(warnings.voltage_error, false);
    assert_eq!(warnings.retain_crc_error, true);
    assert_eq!(warnings.to_bytes().unwrap(), data);

    let data = [0b0000_0100];
    let (_, warnings) = Warnings::from_bytes((data.as_ref(), 0)).unwrap();
    assert_eq!(warnings.i2c_error, false);
    assert_eq!(warnings.voltage_error, true);
    assert_eq!(warnings.retain_crc_error, false);
    assert_eq!(warnings.to_bytes().unwrap(), data);
}
