use deku::prelude::*;

#[derive(Debug, DekuWrite, DekuRead)]
pub struct State {
    pub error_code: ErrorCode,
    #[deku(pad_bits_before = "3")]
    #[deku(bits = "1")]
    pub run: bool,
}

#[derive(Debug, DekuWrite, DekuRead, PartialEq, Eq)]
#[deku(id_type = "u8", bits = "4")]
pub enum ErrorCode {
    #[deku(id = "0")]
    NoError,
    #[deku(id = "2")]
    DataCrcError,
    #[deku(id = "3")]
    DataBlockTooShort,
    #[deku(id = "4")]
    PiXtendModelMismatch,
    #[deku(id = "5")]
    HeaderCrcError,
    #[deku(id = "6")]
    SPIFrequencyTooHigh,
}

#[test]
fn test_state() {
    let data = [0b0000_0001];
    let (_, state) = State::from_bytes((data.as_ref(), 0)).unwrap();
    assert_eq!(state.error_code, ErrorCode::NoError);
    assert_eq!(state.run, true);
    assert_eq!(state.to_bytes().unwrap(), data);

    let data = [0b0110_0000];
    let (_, state) = State::from_bytes((data.as_ref(), 0)).unwrap();
    assert_eq!(state.error_code, ErrorCode::SPIFrequencyTooHigh);
    assert_eq!(state.run, false);
    assert_eq!(state.to_bytes().unwrap(), data);
}
