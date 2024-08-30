use deku::prelude::*;

#[derive(Debug, DekuRead, DekuWrite, Default)]
pub struct Pwm {
    pub group0: PwmGroup,
    pub group1: PwmGroup,
    pub group2: PwmGroup,
}

#[derive(Debug, DekuRead, DekuWrite, Default)]
pub struct PwmGroup {
    pub ctrl0: PwmCtrl,
    #[deku(endian = "little")]
    pub ctrl1: u16,
    #[deku(endian = "little")]
    pub channel0: u16,
    #[deku(endian = "little")]
    pub channel1: u16,
}

#[derive(Debug, DekuRead, DekuWrite, Default)]
pub struct PwmCtrl {
    pub prescaler: PwmPrescaler,
    #[deku(bits = "1")]
    pub enable_b: bool,
    #[deku(bits = "1")]
    pub enable_a: bool,
    #[deku(pad_bits_before = "1")]
    pub mode: PwmMode,
}

#[derive(Debug, DekuRead, DekuWrite, PartialEq, Eq, Default)]
#[deku(id_type = "u8")]
#[deku(bits = "3")]
pub enum PwmPrescaler {
    #[default]
    #[deku(id = "0")]
    Deactivated,
    #[deku(id = "1")]
    Prescale16MHz,
    #[deku(id = "2")]
    Prescale2MHz,
    #[deku(id = "3")]
    Prescale250kHz,
    #[deku(id = "4")]
    Prescale62_5kHz,
    #[deku(id = "5")]
    Prescale15_625kHz,
}

#[derive(Debug, DekuRead, DekuWrite, PartialEq, Eq, Default)]
#[deku(id_type = "u8")]
#[deku(bits = "2")]
pub enum PwmMode {
    #[default]
    #[deku(id = "0")]
    Servo,
    #[deku(id = "1")]
    DutyCycle,
    #[deku(id = "2")]
    Universal,
    #[deku(id = "3")]
    Frequency,
}

#[test]
fn test_pwm_ctrl() {
    let data = [0b0010_1001];
    let (_, pwm_ctrl) = PwmCtrl::from_bytes((&data, 0)).unwrap();
    assert_eq!(pwm_ctrl.prescaler, PwmPrescaler::Prescale16MHz);
    assert_eq!(pwm_ctrl.enable_b, false);
    assert_eq!(pwm_ctrl.enable_a, true);
    assert_eq!(pwm_ctrl.mode, PwmMode::DutyCycle);
    assert_eq!(pwm_ctrl.to_bytes().unwrap(), data);

    let data = [0b1011_0011];
    let (_, pwm_ctrl) = PwmCtrl::from_bytes((&data, 0)).unwrap();
    assert_eq!(pwm_ctrl.prescaler, PwmPrescaler::Prescale15_625kHz);
    assert_eq!(pwm_ctrl.enable_b, true);
    assert_eq!(pwm_ctrl.enable_a, false);
    assert_eq!(pwm_ctrl.mode, PwmMode::Frequency);
    assert_eq!(pwm_ctrl.to_bytes().unwrap(), data);
}
