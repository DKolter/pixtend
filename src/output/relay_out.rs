use deku::prelude::*;

#[derive(Debug, DekuRead, DekuWrite, Default)]
pub struct RelayOut {
    #[deku(pad_bits_before = "4")]
    #[deku(bits = "1")]
    pub relay3: bool,
    #[deku(bits = "1")]
    pub relay2: bool,
    #[deku(bits = "1")]
    pub relay1: bool,
    #[deku(bits = "1")]
    pub relay0: bool,
}

#[test]
fn test_relay_out_control() {
    let data = [0b0000_0101];
    let (_, relay_out) = RelayOut::from_bytes((data.as_ref(), 0)).unwrap();
    assert_eq!(relay_out.relay0, true);
    assert_eq!(relay_out.relay1, false);
    assert_eq!(relay_out.relay2, true);
    assert_eq!(relay_out.relay3, false);
    assert_eq!(relay_out.to_bytes().unwrap(), data);

    let data = [0b0000_1001];
    let (_, relay_out) = RelayOut::from_bytes((data.as_ref(), 0)).unwrap();
    assert_eq!(relay_out.relay0, true);
    assert_eq!(relay_out.relay1, false);
    assert_eq!(relay_out.relay2, false);
    assert_eq!(relay_out.relay3, true);
    assert_eq!(relay_out.to_bytes().unwrap(), data);
}
