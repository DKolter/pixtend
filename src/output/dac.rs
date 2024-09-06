use crate::Channel;
use deku::prelude::*;

#[derive(Debug, DekuRead, DekuWrite, Default, Clone, Copy)]
pub struct Dac {
    #[deku(bits = "1", pad_bits_after = "2")]
    channel: u8,
    #[deku(bits = "1")]
    enabled: bool,
    #[deku(bits = "10", pad_bits_after = "2", endian = "big")]
    value: u16,
}

impl Dac {
    pub fn new(channel: Channel, voltage: f64) -> Self {
        let voltage = voltage.clamp(0.0, 10.0);
        let voltage = ((voltage / 10.0) * 1023.0) as u16;
        let channel = match channel {
            Channel::A => 0,
            Channel::B => 1,
        };

        Self {
            channel,
            enabled: true,
            value: voltage,
        }
    }
}

#[test]
fn test_dac() {
    let data = vec![0b1001_1000, 0b1000_0100];
    let (_, dac) = Dac::from_bytes((data.as_ref(), 0)).unwrap();
    assert_eq!(dac.channel, 1);
    assert_eq!(dac.enabled, true);
    assert_eq!(dac.value, 545);
    assert_eq!(dac.to_bytes().unwrap(), data);

    let data = vec![0b0000_0001, 0b0000_1000];
    let (_, dac) = Dac::from_bytes((data.as_ref(), 0)).unwrap();
    assert_eq!(dac.channel, 0);
    assert_eq!(dac.enabled, false);
    assert_eq!(dac.value, 66);
    assert_eq!(dac.to_bytes().unwrap(), data);
}
