use deku::prelude::*;
use error::PiXtendError;
use input::{ErrorCode, Input};
use output::Output;
use rppal::{
    gpio::Gpio,
    spi::{Bus, Mode, SlaveSelect, Spi},
};
use std::time::{Duration, Instant};

mod error;
mod gpio_config;
mod input;
mod output;
mod utils;

pub use gpio_config::GpioConfig;
pub use input::{ReferenceVoltage, SensorKind, Warnings};
pub use output::Watchdog;

const SPI_ENABLE_PIN: u8 = 24;
const SPI_CLOCK_SPEED: u32 = 700_000;
const COMMUNICATION_DELAY: Duration = Duration::from_millis(30);

pub struct PiXtend {
    spi: Spi,
    input: Option<Input>,
    output: Output,
    gpio_configs: [GpioConfig; 4],
    last_read: Instant,
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
            input: None,
            output,
            gpio_configs,
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

        // Write the output to the PiXtend board
        self.output.update()?;
        self.spi.write(&self.output.to_bytes()?)?;

        // Read the response from the PiXtend board
        let mut buffer = [0u8; 111];
        let bytes_read = self.spi.read(&mut buffer)?;
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

        Ok(())
    }
}
