use deku::prelude::*;
use error::PiXtendError;
use input::{ErrorCode, Input};
use output::{Dac, Output};
use rppal::{
    gpio::Gpio,
    spi::{Bus, Mode, SlaveSelect, Spi},
};
use std::time::{Duration, Instant};

mod error;
mod gpio_config;
mod input;
mod output;
mod pwm_config;
mod utils;

pub use gpio_config::GpioConfig;
pub use input::{ReferenceVoltage, SensorKind, Warnings};
pub use output::{PwmPrescaler, Watchdog};
pub use pwm_config::PwmConfig;

const SPI_ENABLE_PIN: u8 = 24;
const SPI_CLOCK_SPEED: u32 = 700_000;
const COMMUNICATION_DELAY: Duration = Duration::from_millis(30);

pub struct PiXtend {
    spi_pixtend: Spi,
    spi_dac: Spi,
    input: Option<Input>,
    output: Output,
    gpio_configs: [GpioConfig; 4],
    pwm_configs: [PwmConfig; 3],
    dac_configs: [Dac; 2],
    last_read: Instant,
}

impl PiXtend {
    pub fn new() -> Result<Self, PiXtendError> {
        // Setting the SPI_ENABLE_PIN to high enables the communication with the PiXtend board
        Gpio::new()?
            .get(SPI_ENABLE_PIN)?
            .into_output_high()
            .set_reset_on_drop(false);

        // Create the SPI instances for communication with the PiXtend board
        let spi_pixtend = Spi::new(Bus::Spi0, SlaveSelect::Ss0, SPI_CLOCK_SPEED, Mode::Mode0)?;
        let spi_dac = Spi::new(Bus::Spi0, SlaveSelect::Ss1, SPI_CLOCK_SPEED, Mode::Mode0)?;

        // Create a default Output instance
        let output = Output::default();

        // Create default configurations
        let gpio_configs = [GpioConfig::default(); 4];
        let pwm_configs = [PwmConfig::default(); 3];
        let dac_configs = [Dac::new(Channel::A, 0.0), Dac::new(Channel::B, 0.0)];

        Ok(Self {
            spi_pixtend,
            spi_dac,
            input: None,
            output,
            gpio_configs,
            pwm_configs,
            dac_configs,
            last_read: Instant::now(),
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
    /// Returns an error in the following cases:
    /// - Index not in the valid range of `0` to `3`
    /// - Trying to configure a GPIO sensor input while a PWM output is already configured
    /// - Trying to configure a GPIO pullup resistor without first enabling it globally via
    /// `set_gpio_pullup_enable`
    pub fn set_gpio_config(&mut self, index: u8, config: GpioConfig) -> Result<(), PiXtendError> {
        // Check if a PWM output is configured at the same time
        if config == GpioConfig::Sensor
            && self
                .pwm_configs
                .iter()
                .any(|config| *config != PwmConfig::Deactivated)
        {
            return Err(PiXtendError::PwmAndDhtExclusive);
        }

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
    /// Returns an error if the given index is invalid (0 to 3) or if the GPIO is not configured
    /// as an output.
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

    /// Configures the PWM output for the group with the given index. Each group has two channels
    /// (A and B). The configuration can be one of the following:
    /// - `PwmConfig::Deactivated`: The PWM output is deactivated
    /// - `PwmConfig::Servo`: The PWM output is configured for servos with a frequency of 50Hz
    /// - `PwmConfig::DutyCycle`: The PWM output group can set individual duty cycles for channel A
    /// and B, but they share the same frequency, which is set via the prescaler and frequency
    /// - `PwmConfig::Universal`: The PWM output group can only configure frequency and duty
    /// cycle of channel A, while channel B always has 50% duty cycle and half the frequency of A
    /// - `PwmConfig::Frequency`: The PWM output group can set individual frequencies for channel
    /// A and B, but they both have a duty cycle of 50%
    /// Valid indexes are `0` to `2`, returns an error if the index is invalid.
    pub fn set_pwm_config(&mut self, index: u8, config: PwmConfig) -> Result<(), PiXtendError> {
        // Check if any DHT sensors are configured, which is not allowed
        if self
            .gpio_configs
            .iter()
            .any(|config| *config == GpioConfig::Sensor)
        {
            return Err(PiXtendError::PwmAndDhtExclusive);
        }

        // Set the PWM configuration
        self.output.data.pwm.set_pwm_config(index, config)?;
        *self
            .pwm_configs
            .get_mut(index as usize)
            .ok_or(PiXtendError::InvalidPwmOutputGroupIndex(index))? = config;

        Ok(())
    }

    /// Sets the PWM servo position for the given index and channel as a value between `0` and
    /// `16000`. The value is linearly mapped to the pulse width between `1ms` and `2ms`, where
    /// 1ms is the minimum position and 2ms is the maximum position. The frequency is always 50Hz.
    /// Returns an error if the given index is invalid (0 to 2) or if the PWM is not configured
    /// as a servo.
    ///
    /// # Example
    /// We want to set the servo position of PWM 0A to half of the maximum position:
    /// ```no_run
    /// # use pixtend::{PiXtend, PwmConfig, Channel};
    /// # let mut pixtend = PiXtend::new().unwrap();
    /// pixtend.set_pwm_config(0, PwmConfig::Servo { channel_a: true, channel_b: true });
    /// pixtend.set_pwm_servo(0, Channel::A, 8000).unwrap();
    /// ```
    pub fn set_pwm_servo(
        &mut self,
        index: u8,
        channel: Channel,
        value: u16,
    ) -> Result<(), PiXtendError> {
        // Check if the given index is valid
        if index > 2 {
            return Err(PiXtendError::InvalidPwmOutputGroupIndex(index));
        }

        // Check if the pwm is configured as a servo
        if !matches!(self.pwm_configs[index as usize], PwmConfig::Servo { .. }) {
            return Err(PiXtendError::PwmNotConfiguredAsServo(index));
        }

        self.output
            .data
            .pwm
            .set_channel_value(index, channel, value)
    }

    /// Sets the PWM duty cycle for the given index and channel as a value between `0` and
    /// the configured `frequency`, where `0` is 0% duty cycle and the configured frequency is
    /// 100% duty cycle.
    /// Returns an error if the given index is invalid (0 to 2) or if the PWM is not configured
    /// for DutyCycleMode or if the channel is set to B for a Universal mode (only channel A is
    /// configurable in Universal mode).
    ///
    /// # Example
    /// We want to set the duty cycle of PWM 0A to `50%` with 1 Hz:
    /// ```no_run
    /// # use pixtend::{PiXtend, PwmConfig, Channel, PwmPrescaler};
    /// # let mut pixtend = PiXtend::new().unwrap();
    /// pixtend.set_pwm_config(0, PwmConfig::DutyCycle {
    ///    prescaler: PwmPrescaler::Prescale62_5kHz,
    ///    frequency: 31250,
    ///    channel_a: true,
    ///    channel_b: true,
    /// });
    /// pixtend.set_pwm_duty_cycle(0, Channel::A, 15625).unwrap();
    /// ```
    pub fn set_pwm_duty_cycle(
        &mut self,
        index: u8,
        channel: Channel,
        value: u16,
    ) -> Result<(), PiXtendError> {
        // Check if the given index is valid
        if index > 2 {
            return Err(PiXtendError::InvalidPwmOutputGroupIndex(index));
        }

        // The duty cycle is only configurable for both channels in DutyCycle mode
        // and for channel A in Universal mode
        if !matches!(
            (self.pwm_configs[index as usize], channel),
            (PwmConfig::DutyCycle { .. }, _) | (PwmConfig::Universal { .. }, Channel::A)
        ) {
            return Err(PiXtendError::PwmNotConfiguredForDutyCycle(index));
        }

        self.output
            .data
            .pwm
            .set_channel_value(index, channel, value)
    }

    /// Sets the PWM frequency for the given index. The final frequency of the channel is
    /// calculated with the following formula:
    /// `frequency = prescaler / 2 / value`
    ///
    /// # Example
    /// We want to set the frequency of PWM 0A to `1 Hz`:
    /// 1 Hz = PwmPrescaler::Prescale62_5kHz / 2 / 31250
    /// ```no_run
    /// # use pixtend::{PiXtend, PwmConfig, Channel, PwmPrescaler};
    /// # let mut pixtend = PiXtend::new().unwrap();
    /// pixtend.set_pwm_config(0, PwmConfig::Frequency {
    ///     prescaler: PwmPrescaler::Prescale62_5kHz,
    ///     channel_a: true,
    ///     channel_b: false,
    /// }).unwrap();
    ///
    /// pixtend.set_pwm_frequency(0, Channel::A, 31250).unwrap();
    /// ```
    pub fn set_pwm_frequency(
        &mut self,
        index: u8,
        channel: Channel,
        value: u16,
    ) -> Result<(), PiXtendError> {
        // Check if the given index is valid
        if index > 2 {
            return Err(PiXtendError::InvalidPwmOutputGroupIndex(index));
        }

        // Check if the pwm is configured for frequency
        if !matches!(
            self.pwm_configs[index as usize],
            PwmConfig::Frequency { .. }
        ) {
            return Err(PiXtendError::PwmNotConfiguredAsFrequency(index));
        }

        self.output
            .data
            .pwm
            .set_channel_value(index, channel, value)
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

    /// Writes the given voltage to the analog output with the given channel. The voltage is
    /// clamped between `0V` and `10V`.
    pub fn set_analog_output(&mut self, channel: Channel, voltage: f64) {
        let dac = Dac::new(channel, voltage);
        self.dac_configs[channel as usize] = dac;
    }

    /// Reads the firmware version of the PiXtend board.
    /// Returns an error if the input data has not been read yet via `read_write`.
    pub fn get_firmware_version(&self) -> Result<u8, PiXtendError> {
        self.input
            .as_ref()
            .map(|input| input.header.firmware)
            .ok_or(PiXtendError::NoInputDataAvailable)
    }

    /// Reads the hardware version of the PiXtend board.
    /// Returns an error if the input data has not been read yet via `read_write`.
    pub fn get_hardware_version(&self) -> Result<u8, PiXtendError> {
        self.input
            .as_ref()
            .map(|input| input.header.hardware)
            .ok_or(PiXtendError::NoInputDataAvailable)
    }

    /// Returns the warnings that the PiXtend board reports. The warnings are:
    /// - `i2c_error`: An I2C error occurred between the PiXtend board and the Raspberry Pi
    /// - `voltage_error`: The voltage supply of the PiXtend board dropped below 19V. As a result,
    /// the retain memory functionality is not available
    /// - `retain_crc_error`: The CRC check of the retain memory failed
    pub fn get_warnings(&self) -> Result<Warnings, PiXtendError> {
        self.input
            .as_ref()
            .map(|input| input.header.warnings)
            .ok_or(PiXtendError::NoInputDataAvailable)
    }

    /// Reads the digital input at the given index.
    /// Valid indexes are `0` to `15`, returns an error if the index is invalid.
    /// Returns an error if the input data has not been read yet via `read_write`.
    pub fn get_digital_input(&self, index: u8) -> Result<bool, PiXtendError> {
        self.input
            .as_ref()
            .ok_or(PiXtendError::NoInputDataAvailable)?
            .data
            .digital_in
            .get_digital_input(index)
    }

    /// Reads the analog voltage input at the given index in volts. The reference voltage can be
    /// set to either `ReferenceVoltage::V5` for a 0V to 5V range or `ReferenceVoltage::V10` for a 0V
    /// to 10V range. This range is set via jumpers on the PiXtend board. The default is 0V to 10V.
    /// Valid indexes are `0` to `3`, returns an error if the index is invalid.
    /// Returns an error if the input data has not been read yet via `read_write`.
    pub fn get_analog_voltage_input(
        &self,
        index: u8,
        reference_voltage: ReferenceVoltage,
    ) -> Result<f64, PiXtendError> {
        self.input
            .as_ref()
            .ok_or(PiXtendError::NoInputDataAvailable)?
            .data
            .analog_in
            .get_analog_voltage_input(index, reference_voltage)
    }

    /// Reads the analog current input at the given index in Amperes.
    /// Valid indexes are `4` and `5`, returns an error if the index is invalid.
    /// Returns an error if the input data has not been read yet via `read_write`.
    pub fn get_analog_current_input(&self, index: u8) -> Result<f64, PiXtendError> {
        self.input
            .as_ref()
            .ok_or(PiXtendError::NoInputDataAvailable)?
            .data
            .analog_in
            .get_analog_current_input(index)
    }

    /// Reads the GPIO input at the given index.
    /// If the GPIO is not configured as an input, an error is returned.
    /// Valid indexes are `0` to `3`, returns an error if the index is invalid.
    /// Returns an error if the input data has not been read yet via `read_write`.
    pub fn get_gpio_input(&self, index: u8) -> Result<bool, PiXtendError> {
        // Check if the gpio is configured as an input
        if !matches!(
            self.gpio_configs.get(index as usize),
            Some(GpioConfig::Input(_)),
        ) {
            return Err(PiXtendError::GpioNotConfiguredAsInput(index));
        }

        self.input
            .as_ref()
            .ok_or(PiXtendError::NoInputDataAvailable)?
            .data
            .gpio_in
            .get_gpio_input(index)
    }

    /// Reads the temperature from a DHT11/DHT22 onewire sensor connected to the given GPIO
    /// index. The sensor type must be specified to return the calculated temperature in Celsius.
    /// Valid indexes are `0` to `3`, returns an error if the index is invalid.
    /// Returns an error if the input data has not been read yet via `read_write`.
    pub fn get_gpio_temperature(&self, index: u8, sensor: SensorKind) -> Result<f64, PiXtendError> {
        // Check if the gpio is configured as a sensor
        if !matches!(
            self.gpio_configs.get(index as usize),
            Some(GpioConfig::Sensor),
        ) {
            return Err(PiXtendError::GpioNotConfiguredAsInput(index));
        }

        self.input
            .as_ref()
            .ok_or(PiXtendError::NoInputDataAvailable)?
            .data
            .sensor_in
            .get_temperature_input(index, sensor)
    }

    /// Reads the humidity from a DHT11/DHT22 onewire sensor connected to the given GPIO
    /// index. The sensor type must be specified to return the calculated humidity as a percentage
    /// from 0.0 to 1.0.
    /// Valid indexes are `0` to `3`, returns an error if the index is invalid.
    /// Returns an error if the input data has not been read yet via `read_write`.
    pub fn get_gpio_humidity(&self, index: u8, sensor: SensorKind) -> Result<f64, PiXtendError> {
        // Check if the gpio is configured as a sensor
        if !matches!(
            self.gpio_configs.get(index as usize),
            Some(GpioConfig::Sensor),
        ) {
            return Err(PiXtendError::GpioNotConfiguredAsInput(index));
        }

        self.input
            .as_ref()
            .ok_or(PiXtendError::NoInputDataAvailable)?
            .data
            .sensor_in
            .get_humidity_input(index, sensor)
    }

    /// Reads the retain data that the PiXtend board returns. Depending on the value of
    /// `set_retain_copy`, this can be the last saved data or the last data sent by the Raspberry Pi.
    /// Returns an error if the input data has not been read yet via `read_write`.
    pub fn get_retain_data(&self) -> Result<Vec<u8>, PiXtendError> {
        Ok(self
            .input
            .as_ref()
            .ok_or(PiXtendError::NoInputDataAvailable)?
            .data
            .retain
            .clone())
    }

    /// This function does the actual communication with the PiXtend board over SPI. Previous
    /// commands are collected in a frame and then sent to the PiXtend board. The response is read
    /// and stored for easy read access. Before sending a new command, an optional delay of 30ms is
    /// applied, if the last command was sent less than 30ms ago to conform with the PiXtend
    /// documentation on timing.
    ///
    /// This function can fail with a variety of errors, some of the most common ones are:
    /// - `PiXtendError::NotReadyForCommunication`: The PiXtend board is i.e. in safe mode and
    /// not ready for communication, a restart is required
    /// - `PiXtendError::InvalidSpiResponseLength`: The response from the PiXtend board didn't
    /// return the expected number of bytes, this is likely a wiring / connection issue
    /// - `PiXtendError::InputCrcError`: The input data from the PiXtend board is corrupted
    /// - `PiXtendError::PiXtendModelMismatch`: The connected PiXtend board is not a PiXtend L
    /// - `PiXtendError::OutputCrcError`: The output data sent to the PiXtend board is corrupted
    pub fn read_write(&mut self) -> Result<(), PiXtendError> {
        // Check if the PiXtend board is ready
        if let Some(input) = &self.input {
            if !input.header.state.run {
                return Err(PiXtendError::NotReadyForCommunication);
            }
        }

        // Wait for the communication delay to be passed
        let elapsed = self.last_read.elapsed();
        if elapsed < COMMUNICATION_DELAY {
            std::thread::sleep(COMMUNICATION_DELAY - elapsed);
        }

        // Calculate the CRC values
        self.output.update()?;

        // Transfer the data and read the response
        let mut buffer = [0u8; 111];
        let bytes_read = self
            .spi_pixtend
            .transfer(&mut buffer, &self.output.to_bytes()?)?;
        if bytes_read != 111 {
            return Err(PiXtendError::InvalidSpiResponseLength(bytes_read));
        }

        // Parse the response
        let (_, input) = Input::from_bytes((&buffer, 0))?;

        // Check the input CRC
        if !input.check_crc_valid() {
            return Err(PiXtendError::InputCrcError);
        }

        // Check if the returned model matches the PiXtend L
        if input.header.model != b'L' {
            return Err(PiXtendError::PiXtendModelMismatch);
        }

        // Check if there is an error in the state
        match input.header.state.error_code {
            ErrorCode::NoError => {}
            ErrorCode::DataCrcError => return Err(PiXtendError::OutputCrcError),
            ErrorCode::DataBlockTooShort => return Err(PiXtendError::DataBlockTooShort),
            ErrorCode::PiXtendModelMismatch => return Err(PiXtendError::PiXtendModelMismatch),
            ErrorCode::HeaderCrcError => return Err(PiXtendError::OutputCrcError),
            ErrorCode::SPIFrequencyTooHigh => return Err(PiXtendError::SPIFrequencyTooHigh),
        }

        // Store the input for read access
        self.input = Some(input);

        // Write the two DAC values to the DAC SPI
        for dac in self.dac_configs {
            self.spi_dac.write(&dac.to_bytes()?)?;
        }

        Ok(())
    }

    /// Resets the PiXtend instance to its default state. This includes resetting the output,
    /// input, GPIO configurations and PWM configurations.
    pub fn reset(&mut self) {
        self.output = Output::default();
        self.input = None;
        self.gpio_configs = [GpioConfig::default(); 4];
        self.pwm_configs = [PwmConfig::default(); 3];
        self.dac_configs = [Dac::default(); 2];
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Channel {
    A,
    B,
}
