use deku::{DekuContainerWrite, DekuUpdate};
use error::PiXtendError;
use output::Output;
use rppal::{
    gpio::Gpio,
    spi::{Bus, Mode, SlaveSelect, Spi},
};

mod error;
mod output;

const SPI_ENABLE_PIN: u8 = 24;
const SPI_CLOCK_SPEED: u32 = 700_000;

pub struct PiXtend {
    spi: Spi,
    output: Output,
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

        Ok(Self { spi, output })
    }

    pub fn set_digital_output(&mut self, index: u8, value: bool) -> Result<(), PiXtendError> {
        self.output
            .data
            .digital_out
            .set_digital_output(index, value)
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
