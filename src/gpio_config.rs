/// GPIO configuration options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpioConfig {
    /// GPIO Input with optional pullup resistor
    Input(bool),
    /// GPIO Output
    Output,
    /// 1 Wire sensor mode for DHT11, DHT22, AM2302
    Sensor,
}

impl Default for GpioConfig {
    fn default() -> Self {
        GpioConfig::Input(false)
    }
}
