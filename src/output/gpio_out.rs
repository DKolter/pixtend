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
