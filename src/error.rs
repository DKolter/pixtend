use deku::DekuError;
use rppal::{gpio::Error as GpioError, spi::Error as SpiError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PiXtendError {
    #[error("GPIO error: {0}")]
    GpioError(#[from] GpioError),
    #[error("SPI error: {0}")]
    SpiError(#[from] SpiError),
    #[error("Binary frame error: {0}")]
    BinaryFrameReadWriteError(#[from] DekuError),
    #[error("Invalid digital output index: {0}")]
    InvalidDigitalOutputIndex(u8),
    #[error("Invalid digital debounce group: {0}")]
    InvalidDigitalDebounceGroup(u8),
    #[error("Invalid relay output index: {0}")]
    InvalidRelayOutputIndex(u8),
    #[error("Invalid gpio output index: {0}")]
    InvalidGpioOutputIndex(u8),
    #[error("Cannot enable GPIO pullup resistor without enabling it globally")]
    GpioPullupNotGloballyEnabled,
    #[error("GPIO not configured as output: {0}")]
    GpioNotConfiguredAsOutput(u8),
    #[error("GPIO not configured as input: {0}")]
    GpioNotConfiguredAsInput(u8),
    #[error("Invalid gpio debounce group: {0}")]
    InvalidGpioDebounceGroup(u8),
    #[error("Invalid retain data length: {0}")]
    InvalidRetainDataLength(usize),
    #[error("Cannot write retain data without enabling it globally")]
    RetainDataNotGloballyEnabled,
    #[error("Invalid SPI response length: {0}")]
    InvalidSpiResponseLength(usize),
    #[error("Invalid PiXtend model")]
    PiXtendModelMismatch,
    #[error("CRC Error occured in input data")]
    InputCrcError,
    #[error("PiXtend reports crc error in output data")]
    OutputCrcError,
    #[error("PiXtend reports invalid SPI frequency")]
    SPIFrequencyTooHigh,
    #[error("PiXtend reports incoming datablock too short")]
    DataBlockTooShort,
    #[error("PiXtend not ready for communication, maybe it is in safe mode")]
    NotReadyForCommunication,
    #[error("No input data available, fetch using `read_write` first")]
    NoInputDataAvailable,
    #[error("Invalid digital input index: {0}")]
    InvalidDigitalInputIndex(u8),
    #[error("Invalid analog input index: {0}")]
    InvalidAnalogVoltageInputIndex(u8),
    #[error("Invalid analog current index: {0}")]
    InvalidAnalogCurrentInputIndex(u8),
    #[error("Invalid gpio input index: {0}")]
    InvalidGpioInputIndex(u8),
}
