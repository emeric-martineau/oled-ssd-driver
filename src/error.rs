#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayError {
    /// I2C or SPI bus communication error
    BusError,
    /// Invalid parameter value
    InvalidArgument,
}
