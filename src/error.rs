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
}
