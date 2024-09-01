use deku::{DekuContainerWrite, DekuUpdate};
use error::PiXtendError;
use output::Output;
use rppal::{
    gpio::Gpio,
    spi::{Bus, Mode, SlaveSelect, Spi},
};

mod error;
mod gpio_config;
mod output;

pub use gpio_config::GpioConfig;
pub use output::Watchdog;

const SPI_ENABLE_PIN: u8 = 24;
const SPI_CLOCK_SPEED: u32 = 700_000;

pub struct PiXtend {
    spi: Spi,
    output: Output,
    gpio_configs: [GpioConfig; 4],
}

impl PiXtend {
    pub fn new() -> Result<Self, PiXtendError> {
        // Setting the SPI_ENABLE_PIN to high enables the communication with the PiXtend board
        Gpio::new()?
            .get(SPI_ENABLE_PIN)?
            .into_output_high()
            .set_reset_on_drop(false);

        // Create the SPI instance for PiXtend communication
        let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, SPI_CLOCK_SPEED, Mode::Mode0)?;

        // Create a default Output instance
        let output = Output::default();

        // Create default GPIO configurations
        let gpio_configs = [GpioConfig::default(); 4];

        Ok(Self {
            spi,
            output,
            gpio_configs,
        })
    }

    /// If the watchdog is activated, the communication between the Raspberry Pi and the PiXtend
    /// is monitored. If there is a pause between two valid cycles which is longer than the
    /// set time, the watchdog becomes active and puts the microcontroller into a safe state.
    /// An invalid cycle (e.g. due to a CRC error) is evaluated by the watchdog as if no
    /// cycle had been performed.
    pub fn set_watchdog(&mut self, watchdog: Watchdog) {
        self.output.header.watchdog = watchdog;
    }

    /// The Retain Copy option can be used to configure which data is visible in the retain
    /// input area. At the start value `0`, the last saved data is transferred from the
    /// microcontroller to the Raspberry Pi, normal Retain operation. If the value `1` is set for
    /// the RC bit, the last data sent by the Raspberry Pi is returned. In the retain area, the
    /// retain data is mirrored, but with a cycle delay. The content of the retain area is
    /// not lost, it is just not displayed as long as the RC bit is `1`.
    pub fn set_retain_copy(&mut self, value: bool) {
        self.output.header.system.retain_copy = value;
    }

    /// With the retain enable bit, the retain function of the PiXtend can be activated. For this
    /// purpose, a `1` is written in to this bit. By default, RE is `0`, after a reset or power-up
    /// and therefore retain is deactivated.
    pub fn set_retain_enable(&mut self, value: bool) {
        self.output.header.system.retain_enable = value;
    }

    /// Setting this option to `true` disables the status LED. The LED is active by default.
    pub fn set_led_disable(&mut self, value: bool) {
        self.output.header.system.led_disable = value;
    }

    /// The gpio pullup enable bit can be used to enable the pull-up resistors of the PiXtend
    /// GPIOS, but they are only activated, when the GPIOs are configured as inputs and have the
    /// pullup option set as well.
    pub fn set_gpio_pullup_enable(&mut self, value: bool) {
        self.output.header.system.gpio_pullup_enable = value;
    }

    /// Puts the microcontroller into the safe state defined as follows:
    /// - All digital outputs and relays are switched off / put into idle state
    /// - PWM outputs are switched to high impedance (tri-state)
    /// - Retain data is stored when retain option has been activated
    /// - The status LED "L1" flashes depending on the cause of the error
    /// - The microcontroller or the PiXtend device has to be restarted (power cycle)
    pub fn enable_safe_mode(&mut self) {
        self.output.header.system.safe = true;
    }

    /// Configures the digital debounce for the given group. There are 8 groups of two digital
    /// inputs each available:
    /// - Group 0: Digital input 0 and 1
    /// - Group 1: Digital input 2 and 3
    /// - Group 2: Digital input 4 and 5
    /// - Group 3: Digital input 6 and 7
    /// - Group 4: Digital input 8 and 9
    /// - Group 5: Digital input 10 and 11
    /// - Group 6: Digital input 12 and 13
    /// - Group 7: Digital input 14 and 15
    ///
    /// The debounce time is set in cycles. Each cycle has a duration of 30ms.
    pub fn set_digital_debounce(&mut self, group: u8, value: u8) -> Result<(), PiXtendError> {
        self.output
            .data
            .digital_debounce
            .set_digital_debounce(group, value)
    }

    /// Writes the given value to the digital output with the given index.
    /// Valid indexes are `0` to `11`, returns an error if the index is invalid.
    pub fn set_digital_output(&mut self, index: u8, value: bool) -> Result<(), PiXtendError> {
        self.output
            .data
            .digital_out
            .set_digital_output(index, value)
    }

    /// Writes the given value to the relay output with the given index.
    /// Valid indexes are `0` to `3`, returns an error if the index is invalid.
    pub fn set_relay_output(&mut self, index: u8, value: bool) -> Result<(), PiXtendError> {
        self.output.data.relay_out.set_relay_output(index, value)
    }

    /// Configures the GPIO with the given index. The configuration can be one of the following:
    /// - `GpioConfig::Output`: The GPIO is configured as an output
    /// - `GpioConfig::Input(false)`: The GPIO is configured as an input without a pull-up
    /// resistor
    /// - `GpioConfig::Input(true)`: The GPIO is configured as an input with a pull-up resistor
    /// - `GpioConfig::Sensor`: The GPIO is configured as a onewire sensor input, for example
    /// for a DHT11, DHT22 or AM2302 sensor
    ///
    /// Valid indexes are `0` to `3`, returns an error if the index is invalid.
    /// Also returns an error if you are trying to configure a GPIO as an input with a pull-up
    /// resistor, but the GPIO pullup enable bit is not globally enabled via `set_gpio_pullup_enable`.
    pub fn set_gpio_config(&mut self, index: u8, config: GpioConfig) -> Result<(), PiXtendError> {
        // To enable a pullup resistor on an input GPIO, the GPIO pullup enable bit must be set
        if config == GpioConfig::Input(true) && !self.output.header.system.gpio_pullup_enable {
            return Err(PiXtendError::GpioPullupNotGloballyEnabled);
        }

        self.output.data.gpio_ctrl.set_gpio_config(index, config)?;
        self.output.data.gpio_out.set_gpio_config(index, config)?;
        *self
            .gpio_configs
            .get_mut(index as usize)
            .ok_or(PiXtendError::InvalidGpioOutputIndex(index))? = config;

        Ok(())
    }

    /// Writes the given value to the GPIO output with the given index.
    /// Valid indexes are `0` to `3`, returns an error if the index is invalid.
    pub fn set_gpio_output(&mut self, index: u8, value: bool) -> Result<(), PiXtendError> {
        // Check if the given index is valid
        if index > 3 {
            return Err(PiXtendError::InvalidGpioOutputIndex(index));
        }

        // Check if the GPIO is configured as an output
        if self.gpio_configs[index as usize] != GpioConfig::Output {
            return Err(PiXtendError::GpioNotConfiguredAsOutput(index));
        }

        self.output.data.gpio_out.set_gpio_output(index, value)
    }

    /// Configures the gpio debounce for the given group. There are 2 groups of two digital
    /// inputs each available:
    /// - Group 0: Digital input 0 and 1
    /// - Group 1: Digital input 2 and 3
    ///
    /// The debounce time is set in cycles. Each cycle has a duration of 30ms.
    pub fn set_gpio_debounce(&mut self, group: u8, value: u8) -> Result<(), PiXtendError> {
        self.output
            .data
            .gpio_debounce
            .set_gpio_debounce(group, value)
    }

    /// Retain data can be used to store at most 64 bytes of data in the PiXtend board. This data
    /// is retained even after a power cycle. The data can be read and written by the Raspberry
    /// Pi. If less than 64 are passed, the remaining bytes are filled with zeros.
    /// Returns an error if the given data length is greater than 64 or if the retain option
    /// is not globally enabled via `set_retain_enable`.
    pub fn set_retain_data(&mut self, data: Vec<u8>) -> Result<(), PiXtendError> {
        // Check if retain is enabled
        if !self.output.header.system.retain_enable {
            return Err(PiXtendError::RetainDataNotGloballyEnabled);
        }

        self.output.data.retain.set_retain_data(data)
    }

    pub fn write(&mut self) -> Result<(), PiXtendError> {
        // Write the output to the PiXtend board
        self.output.update()?;
        self.spi.write(&self.output.to_bytes()?)?;

        // Read the response from the PiXtend board
        let mut buffer = [0u8; 111];
        self.spi.read(&mut buffer)?;
        println!("{:?}", buffer);

        Ok(())
    }
}
