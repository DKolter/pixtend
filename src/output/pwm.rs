use crate::{error::PiXtendError, Channel, PwmConfig};
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
    pub channel_b: bool,
    #[deku(bits = "1")]
    pub channel_a: bool,
    #[deku(pad_bits_before = "1")]
    pub mode: PwmMode,
}

#[derive(Debug, DekuRead, DekuWrite, PartialEq, Eq, Default, Clone, Copy)]
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

impl Pwm {
    pub fn set_pwm_config(&mut self, index: u8, config: PwmConfig) -> Result<(), PiXtendError> {
        match index {
            0 => self.group0 = config.into(),
            1 => self.group1 = config.into(),
            2 => self.group2 = config.into(),
            _ => return Err(PiXtendError::InvalidPwmOutputGroupIndex(index)),
        }

        Ok(())
    }

    pub fn set_channel_value(
        &mut self,
        index: u8,
        channel: Channel,
        value: u16,
    ) -> Result<(), PiXtendError> {
        match (index, channel) {
            (0, Channel::A) => self.group0.channel0 = value,
            (0, Channel::B) => self.group0.channel1 = value,
            (1, Channel::A) => self.group1.channel0 = value,
            (1, Channel::B) => self.group1.channel1 = value,
            (2, Channel::A) => self.group2.channel0 = value,
            (2, Channel::B) => self.group2.channel1 = value,
            _ => return Err(PiXtendError::InvalidPwmOutputGroupIndex(index)),
        }

        Ok(())
    }
}

impl From<PwmConfig> for PwmGroup {
    fn from(config: PwmConfig) -> Self {
        PwmGroup {
            ctrl0: PwmCtrl {
                mode: match config {
                    PwmConfig::Deactivated | PwmConfig::Servo { .. } => PwmMode::Servo,
                    PwmConfig::DutyCycle { .. } => PwmMode::DutyCycle,
                    PwmConfig::Universal { .. } => PwmMode::Universal,
                    PwmConfig::Frequency { .. } => PwmMode::Frequency,
                },
                prescaler: match config {
                    PwmConfig::Deactivated | PwmConfig::Servo { .. } => PwmPrescaler::Deactivated,
                    PwmConfig::DutyCycle { prescaler, .. }
                    | PwmConfig::Universal { prescaler, .. }
                    | PwmConfig::Frequency { prescaler, .. } => prescaler,
                },
                channel_a: match config {
                    PwmConfig::Deactivated => false,
                    PwmConfig::Servo { channel_a, .. }
                    | PwmConfig::DutyCycle { channel_a, .. }
                    | PwmConfig::Universal { channel_a, .. }
                    | PwmConfig::Frequency { channel_a, .. } => channel_a,
                },
                channel_b: match config {
                    PwmConfig::Deactivated => false,
                    PwmConfig::Servo { channel_b, .. }
                    | PwmConfig::DutyCycle { channel_b, .. }
                    | PwmConfig::Universal { channel_b, .. }
                    | PwmConfig::Frequency { channel_b, .. } => channel_b,
                },
            },
            ctrl1: match config {
                PwmConfig::Deactivated | PwmConfig::Servo { .. } | PwmConfig::Frequency { .. } => 0,
                PwmConfig::DutyCycle { frequency, .. } | PwmConfig::Universal { frequency, .. } => {
                    frequency
                }
            },
            channel0: 0,
            channel1: 0,
        }
    }
}

#[test]
fn test_pwm_ctrl() {
    let data = [0b0010_1001];
    let (_, pwm_ctrl) = PwmCtrl::from_bytes((&data, 0)).unwrap();
    assert_eq!(pwm_ctrl.prescaler, PwmPrescaler::Prescale16MHz);
    assert_eq!(pwm_ctrl.channel_b, false);
    assert_eq!(pwm_ctrl.channel_a, true);
    assert_eq!(pwm_ctrl.mode, PwmMode::DutyCycle);
    assert_eq!(pwm_ctrl.to_bytes().unwrap(), data);

    let data = [0b1011_0011];
    let (_, pwm_ctrl) = PwmCtrl::from_bytes((&data, 0)).unwrap();
    assert_eq!(pwm_ctrl.prescaler, PwmPrescaler::Prescale15_625kHz);
    assert_eq!(pwm_ctrl.channel_b, true);
    assert_eq!(pwm_ctrl.channel_a, false);
    assert_eq!(pwm_ctrl.mode, PwmMode::Frequency);
    assert_eq!(pwm_ctrl.to_bytes().unwrap(), data);
}
