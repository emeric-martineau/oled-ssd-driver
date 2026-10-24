use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::{Gray4, GrayColor},
    Pixel,
};

use super::init::{InitConfig, DEFAULT_DISPLAY_SIZE};
use crate::common::DisplaySize;
use crate::common::{Buffered, Unbuffered};
use crate::error::DisplayError;
use crate::interface::{AsyncDisplayInterface, DisplayInterface};

/// Maximum buffer: 128 pixels / 2 × 128 rows = 8192 bytes
const BUFFER_SIZE_1327: usize = 64 * 128;

pub trait Ssd1327BufferMode {
    type Buffer;
    fn new_buffer() -> Self::Buffer;
}

impl Ssd1327BufferMode for Buffered {
    type Buffer = [u8; BUFFER_SIZE_1327];
    fn new_buffer() -> Self::Buffer {
        [0u8; BUFFER_SIZE_1327]
    }
}

impl Ssd1327BufferMode for Unbuffered {
    type Buffer = ();
    fn new_buffer() -> Self::Buffer {}
}

/// SSD1327 OLED display driver.
///
/// Generic over the bus interface `DI` (I2C or SPI) and buffer mode `MODE`.
/// In `Buffered` mode (default) the driver owns a framebuffer and supports all drawing APIs.
/// In `Unbuffered` mode the driver owns no RAM buffer; only control commands are available.
pub struct Ssd1327<DI, MODE: Ssd1327BufferMode = Buffered> {
    interface: DI,
    buffer: MODE::Buffer,
    size: DisplaySize,
}

impl<DI> Ssd1327<DI, Buffered> {
    /// Create a new SSD1327 driver with default 128x128 display size (buffered).
    pub fn new(interface: DI) -> Self {
        Self {
            interface,
            buffer: [0u8; BUFFER_SIZE_1327],
            size: DEFAULT_DISPLAY_SIZE,
        }
    }

    pub fn new_with_size(interface: DI, size: DisplaySize) -> Self {
        Self {
            interface,
            buffer: [0u8; BUFFER_SIZE_1327],
            size,
        }
    }

    pub fn release(self) -> DI {
        self.interface
    }

    /// SSD1327 §7.7 — Set a pixel in the internal framebuffer. Gray value 0-15.
    pub fn set_pixel(&mut self, x: u8, y: u8, gray: u8) {
        if u16::from(x) >= self.size.width || y >= self.size.height {
            return;
        }
        let col = (x / 2) as usize;
        let row = y as usize;
        let columns = (self.size.width / 2) as u8 as usize;
        let idx = row * columns + col;
        let value = gray & 0x0F;
        if x.is_multiple_of(2) {
            self.buffer[idx] = (self.buffer[idx] & 0x0F) | (value << 4);
        } else {
            self.buffer[idx] = (self.buffer[idx] & 0xF0) | value;
        }
    }

    pub fn clear_buffer(&mut self) {
        let len = (self.size.width / 2) as u8 as usize * self.size.height as usize;
        self.buffer[..len].fill(0);
    }

    pub fn buffer(&self) -> &[u8] {
        let len = (self.size.width / 2) as u8 as usize * self.size.height as usize;
        &self.buffer[..len]
    }

    pub fn size_config(&self) -> DisplaySize {
        self.size
    }
}

impl<DI> Ssd1327<DI, Unbuffered> {
    /// Create a new SSD1327 driver in unbuffered mode with default 128x128 display size.
    pub fn new_unbuffered(interface: DI) -> Self {
        Self {
            interface,
            buffer: (),
            size: DEFAULT_DISPLAY_SIZE,
        }
    }

    pub fn new_unbuffered_with_size(interface: DI, size: DisplaySize) -> Self {
        Self {
            interface,
            buffer: (),
            size,
        }
    }

    pub fn release(self) -> DI {
        self.interface
    }

    pub fn size_config(&self) -> DisplaySize {
        self.size
    }
}

// --- Synchronous API (Buffered) ---

impl<DI: DisplayInterface> Ssd1327<DI, Buffered> {
    /// SSD1327 §7.9 — Power ON initialization sequence.
    pub fn init(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        ssd1327_init_commands(&mut self.interface, &self.size, config)
    }

    pub fn flush(&mut self) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[0x15, 0x00, (self.size.width / 2) as u8 - 1])?;
        self.interface
            .send_command(&[0x75, 0x00, self.size.height - 1])?;
        let len = (self.size.width / 2) as u8 as usize * self.size.height as usize;
        self.interface.send_data(&self.buffer[..len])
    }

    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.interface.send_command(&[if on { 0xAF } else { 0xAE }])
    }

    pub fn set_contrast(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x81, contrast])
    }

    pub fn set_inverse(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if inverse { 0xA7 } else { 0xA4 }])
    }

    pub fn set_vcomh(&mut self, level: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xBE, level & 0x07])
    }

    pub fn set_precharge_voltage(&mut self, voltage: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xBC, voltage & 0x0F])
    }

    pub fn set_internal_vdd(&mut self, enable: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[0xAB, if enable { 0x01 } else { 0x00 }])
    }
}

// --- Synchronous API (Unbuffered) ---

impl<DI: DisplayInterface> Ssd1327<DI, Unbuffered> {
    /// SSD1327 §7.9 — Power ON initialization sequence (unbuffered).
    pub fn init(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        ssd1327_init_commands(&mut self.interface, &self.size, config)
    }

    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.interface.send_command(&[if on { 0xAF } else { 0xAE }])
    }

    pub fn set_contrast(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x81, contrast])
    }

    pub fn set_inverse(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if inverse { 0xA7 } else { 0xA4 }])
    }

    pub fn set_vcomh(&mut self, level: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xBE, level & 0x07])
    }

    pub fn set_precharge_voltage(&mut self, voltage: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xBC, voltage & 0x0F])
    }

    pub fn set_internal_vdd(&mut self, enable: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[0xAB, if enable { 0x01 } else { 0x00 }])
    }
}

// --- Async API (Buffered) ---

impl<DI: AsyncDisplayInterface> Ssd1327<DI, Buffered> {
    pub async fn init_async(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        ssd1327_init_commands_async(&mut self.interface, &self.size, config).await
    }

    pub async fn flush_async(&mut self) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[0x15, 0x00, (self.size.width / 2) as u8 - 1])
            .await?;
        self.interface
            .send_command(&[0x75, 0x00, self.size.height - 1])
            .await?;
        let len = (self.size.width / 2) as u8 as usize * self.size.height as usize;
        self.interface.send_data(&self.buffer[..len]).await
    }

    pub async fn display_on_async(&mut self, on: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if on { 0xAF } else { 0xAE }])
            .await
    }

    pub async fn set_contrast_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x81, contrast]).await
    }

    pub async fn set_inverse_async(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if inverse { 0xA7 } else { 0xA4 }])
            .await
    }
}

// --- Async API (Unbuffered) ---

impl<DI: AsyncDisplayInterface> Ssd1327<DI, Unbuffered> {
    pub async fn init_async(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        ssd1327_init_commands_async(&mut self.interface, &self.size, config).await
    }

    pub async fn display_on_async(&mut self, on: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if on { 0xAF } else { 0xAE }])
            .await
    }

    pub async fn set_contrast_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x81, contrast]).await
    }

    pub async fn set_inverse_async(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if inverse { 0xA7 } else { 0xA4 }])
            .await
    }
}

// --- embedded-graphics DrawTarget (Buffered only for 4-bit gray) ---

impl<DI, MODE: Ssd1327BufferMode> OriginDimensions for Ssd1327<DI, MODE> {
    fn size(&self) -> Size {
        Size::new(self.size.width as u32, self.size.height as u32)
    }
}

impl<DI> DrawTarget for Ssd1327<DI, Buffered> {
    type Color = Gray4;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let w = self.size.width as i32;
        let h = self.size.height as i32;
        for Pixel(coord, color) in pixels {
            if coord.x >= 0 && coord.x < w && coord.y >= 0 && coord.y < h {
                self.set_pixel(coord.x as u8, coord.y as u8, color.luma());
            }
        }
        Ok(())
    }
}

fn ssd1327_init_commands(
    interface: &mut impl DisplayInterface,
    size: &DisplaySize,
    config: &InitConfig,
) -> Result<(), DisplayError> {
    interface.send_command(&[0xAE])?;
    interface.send_command(&[
        0xB3,
        (config.osc_freq << 4) | (config.clock_divide_ratio & 0x0F),
    ])?;
    interface.send_command(&[0xA8, size.height - 1])?;
    interface.send_command(&[0xA2, config.display_offset])?;
    interface.send_command(&[0xA1, config.display_start_line])?;
    interface.send_command(&[0xA0, config.remap_byte()])?;
    interface.send_command(&[0x81, config.contrast])?;
    interface.send_command(&[0xAB, if config.internal_vdd { 0x01 } else { 0x00 }])?;
    interface.send_command(&[0xB1, (config.phase2 << 4) | (config.phase1 & 0x0F)])?;
    interface.send_command(&[0xBC, config.precharge_voltage & 0x0F])?;
    interface.send_command(&[0xBE, config.vcomh & 0x07])?;
    interface.send_command(&[0xD5, config.function_b_byte()])?;
    interface.send_command(&[0xB6, config.second_precharge_period & 0x0F])?;
    interface.send_command(&[0xB9])?;
    let mode_cmd = if config.entire_display_on {
        0xA5
    } else if config.inverse_display {
        0xA7
    } else {
        0xA4
    };
    interface.send_command(&[mode_cmd])?;
    if config.display_on {
        interface.send_command(&[0xAF])?;
    }
    Ok(())
}

async fn ssd1327_init_commands_async(
    interface: &mut impl AsyncDisplayInterface,
    size: &DisplaySize,
    config: &InitConfig,
) -> Result<(), DisplayError> {
    interface.send_command(&[0xAE]).await?;
    interface
        .send_command(&[
            0xB3,
            (config.osc_freq << 4) | (config.clock_divide_ratio & 0x0F),
        ])
        .await?;
    interface.send_command(&[0xA8, size.height - 1]).await?;
    interface
        .send_command(&[0xA2, config.display_offset])
        .await?;
    interface
        .send_command(&[0xA1, config.display_start_line])
        .await?;
    interface.send_command(&[0xA0, config.remap_byte()]).await?;
    interface.send_command(&[0x81, config.contrast]).await?;
    interface
        .send_command(&[0xAB, if config.internal_vdd { 0x01 } else { 0x00 }])
        .await?;
    interface
        .send_command(&[0xB1, (config.phase2 << 4) | (config.phase1 & 0x0F)])
        .await?;
    interface
        .send_command(&[0xBC, config.precharge_voltage & 0x0F])
        .await?;
    interface.send_command(&[0xBE, config.vcomh & 0x07]).await?;
    interface
        .send_command(&[0xD5, config.function_b_byte()])
        .await?;
    interface
        .send_command(&[0xB6, config.second_precharge_period & 0x0F])
        .await?;
    interface.send_command(&[0xB9]).await?;
    let mode_cmd = if config.entire_display_on {
        0xA5
    } else if config.inverse_display {
        0xA7
    } else {
        0xA4
    };
    interface.send_command(&[mode_cmd]).await?;
    if config.display_on {
        interface.send_command(&[0xAF]).await?;
    }
    Ok(())
}
