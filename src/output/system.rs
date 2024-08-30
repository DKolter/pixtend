use deku::prelude::*;

#[derive(Debug, DekuRead, DekuWrite, Default)]
pub struct System {
    #[deku(pad_bits_before = "3")]
    #[deku(bits = "1")]
    pub gpio_pullup_enable: bool,
    #[deku(bits = "1")]
    pub led_disable: bool,
    #[deku(bits = "1")]
    pub retain_enable: bool,
    #[deku(bits = "1")]
    pub retain_copy: bool,
    #[deku(bits = "1")]
    pub safe: bool,
}

#[test]
fn test_system_control() {
    let data = [0b0101_0101];
    let (_, system_control) = System::from_bytes((&data, 0)).unwrap();
    assert_eq!(system_control.safe, true);
    assert_eq!(system_control.retain_copy, false);
    assert_eq!(system_control.retain_enable, true);
    assert_eq!(system_control.led_disable, false);
    assert_eq!(system_control.gpio_pullup_enable, true);

    let data = [0b1010_1010];
    let (_, system_control) = System::from_bytes((&data, 0)).unwrap();
    assert_eq!(system_control.safe, false);
    assert_eq!(system_control.retain_copy, true);
    assert_eq!(system_control.retain_enable, false);
    assert_eq!(system_control.led_disable, true);
    assert_eq!(system_control.gpio_pullup_enable, false);

    let data = [0b0000_1111];
    let (_, system_control) = System::from_bytes((&data, 0)).unwrap();
    assert_eq!(system_control.safe, true);
    assert_eq!(system_control.retain_copy, true);
    assert_eq!(system_control.retain_enable, true);
    assert_eq!(system_control.led_disable, true);
    assert_eq!(system_control.gpio_pullup_enable, false);

    let system_control = System {
        gpio_pullup_enable: true,
        led_disable: false,
        retain_enable: true,
        retain_copy: true,
        safe: true,
    };
    let data = system_control.to_bytes().unwrap();
    assert_eq!(data, [0b00010111]);
}
