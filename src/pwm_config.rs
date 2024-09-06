use crate::output::PwmPrescaler;

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub enum PwmConfig {
    /// PWM is deactivated
    #[default]
    Deactivated,
    Servo {
        channel_a: bool,
        channel_b: bool,
    },
    /// A duty cycle group can set individual duty cycles for channel A and B, but they share the
    /// same frequency.
    /// It can be configured with a prescaler and a frequency. The
    /// prescaler determines the base frequency of the PWM signal, the frequency then sets the
    /// fine-grained frequency of the PWM signal with the following formula:
    /// `frequency = prescaler / 2 / frequency`
    ///
    /// # Example
    /// Target frequency: `1 Hz`
    /// 1 Hz = PwmPrescaler::Prescale62_5kHz / 2 / 31250
    DutyCycle {
        prescaler: PwmPrescaler,
        frequency: u16,
        channel_a: bool,
        channel_b: bool,
    },
    /// A universal group can only configure frequency and duty cycle of channel A, while channel B
    /// always has 50% duty cycle and half the frequency of channel A.
    /// It can be configured with a prescaler and a frequency. The
    /// prescaler determines the base frequency of the PWM signal, the frequency then sets the
    /// fine-grained frequency of the PWM signal with the following formula:
    /// `frequency = prescaler / 2 / frequency`
    ///
    /// # Example
    /// Target frequency channel A: `1 Hz`
    /// 1 Hz = PwmPrescaler::Prescale62_5kHz / 2 / 31250
    /// => Channel B frequency: 0.5 Hz
    Universal {
        prescaler: PwmPrescaler,
        frequency: u16,
        duty_cycle: u16,
        channel_a: bool,
        channel_b: bool,
    },
    /// A frequency group can set individual frequencies for channel A and B, but they both have a
    /// duty cycle of 50%. The prescaler determines the base frequency of the PWM signal. The
    /// maximum frequency that can be set for the individual channels after the configuration is `20kHz`.
    Frequency {
        prescaler: PwmPrescaler,
        channel_a: bool,
        channel_b: bool,
    },
}
