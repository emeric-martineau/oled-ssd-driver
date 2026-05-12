use crate::error::DisplayError;

/// SSD1306 §8.1.5 — I2C control byte: Co=0, D/C#=0 for commands
const I2C_COMMAND_BYTE: u8 = 0x00;
/// SSD1306 §8.1.5 — I2C control byte: Co=0, D/C#=1 for data
const I2C_DATA_BYTE: u8 = 0x40;

/// SSD1306 §8.1.5 — Default I2C slave address (SA0=0 → 0x3C, SA0=1 → 0x3D)
pub const DEFAULT_I2C_ADDRESS: u8 = 0x3C;

pub trait DisplayInterface {
    fn send_command(&mut self, cmd: &[u8]) -> Result<(), DisplayError>;
    fn send_data(&mut self, data: &[u8]) -> Result<(), DisplayError>;
}

/// SSD1306 §8.1.5 — I2C interface wrapper
pub struct I2cInterface<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C> I2cInterface<I2C> {
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self { i2c, address }
    }

    pub fn release(self) -> I2C {
        self.i2c
    }
}

impl<I2C> DisplayInterface for I2cInterface<I2C>
where
    I2C: embedded_hal::i2c::I2c,
{
    fn send_command(&mut self, cmd: &[u8]) -> Result<(), DisplayError> {
        // SSD1306 §8.1.5.1 — Control byte 0x00: Co=0, D/C#=0 (command stream)
        for &byte in cmd {
            self.i2c
                .write(self.address, &[I2C_COMMAND_BYTE, byte])
                .map_err(|_| DisplayError::BusError)?;
        }
        Ok(())
    }

    fn send_data(&mut self, data: &[u8]) -> Result<(), DisplayError> {
        // SSD1306 §8.1.5.1 — Control byte 0x40: Co=0, D/C#=1 (data stream)
        // Send in chunks to avoid needing a large buffer
        for chunk in data.chunks(32) {
            let mut buf = [0u8; 33];
            buf[0] = I2C_DATA_BYTE;
            buf[1..=chunk.len()].copy_from_slice(chunk);
            self.i2c
                .write(self.address, &buf[..=chunk.len()])
                .map_err(|_| DisplayError::BusError)?;
        }
        Ok(())
    }
}

/// SSD1306 §8.1.3 — 4-wire SPI interface wrapper
pub struct SpiInterface<SPI, DC> {
    spi: SPI,
    dc: DC,
}

impl<SPI, DC> SpiInterface<SPI, DC> {
    pub fn new(spi: SPI, dc: DC) -> Self {
        Self { spi, dc }
    }

    pub fn release(self) -> (SPI, DC) {
        (self.spi, self.dc)
    }
}

impl<SPI, DC> DisplayInterface for SpiInterface<SPI, DC>
where
    SPI: embedded_hal::spi::SpiDevice,
    DC: embedded_hal::digital::OutputPin,
{
    fn send_command(&mut self, cmd: &[u8]) -> Result<(), DisplayError> {
        // SSD1306 §8.1.3 — D/C# LOW for command
        self.dc.set_low().map_err(|_| DisplayError::BusError)?;
        self.spi.write(cmd).map_err(|_| DisplayError::BusError)?;
        Ok(())
    }

    fn send_data(&mut self, data: &[u8]) -> Result<(), DisplayError> {
        // SSD1306 §8.1.3 — D/C# HIGH for data
        self.dc.set_high().map_err(|_| DisplayError::BusError)?;
        self.spi.write(data).map_err(|_| DisplayError::BusError)?;
        Ok(())
    }
}

/// Async variant of the display interface
pub trait AsyncDisplayInterface {
    fn send_command(
        &mut self,
        cmd: &[u8],
    ) -> impl core::future::Future<Output = Result<(), DisplayError>>;
    fn send_data(
        &mut self,
        data: &[u8],
    ) -> impl core::future::Future<Output = Result<(), DisplayError>>;
}

impl<I2C> AsyncDisplayInterface for I2cInterface<I2C>
where
    I2C: embedded_hal_async::i2c::I2c,
{
    async fn send_command(&mut self, cmd: &[u8]) -> Result<(), DisplayError> {
        for &byte in cmd {
            self.i2c
                .write(self.address, &[I2C_COMMAND_BYTE, byte])
                .await
                .map_err(|_| DisplayError::BusError)?;
        }
        Ok(())
    }

    async fn send_data(&mut self, data: &[u8]) -> Result<(), DisplayError> {
        for chunk in data.chunks(32) {
            let mut buf = [0u8; 33];
            buf[0] = I2C_DATA_BYTE;
            buf[1..=chunk.len()].copy_from_slice(chunk);
            self.i2c
                .write(self.address, &buf[..=chunk.len()])
                .await
                .map_err(|_| DisplayError::BusError)?;
        }
        Ok(())
    }
}

impl<SPI, DC> AsyncDisplayInterface for SpiInterface<SPI, DC>
where
    SPI: embedded_hal_async::spi::SpiDevice,
    DC: embedded_hal::digital::OutputPin,
{
    async fn send_command(&mut self, cmd: &[u8]) -> Result<(), DisplayError> {
        self.dc.set_low().map_err(|_| DisplayError::BusError)?;
        self.spi
            .write(cmd)
            .await
            .map_err(|_| DisplayError::BusError)?;
        Ok(())
    }

    async fn send_data(&mut self, data: &[u8]) -> Result<(), DisplayError> {
        self.dc.set_high().map_err(|_| DisplayError::BusError)?;
        self.spi
            .write(data)
            .await
            .map_err(|_| DisplayError::BusError)?;
        Ok(())
    }
}
