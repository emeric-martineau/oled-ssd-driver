use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::{Gray4, GrayColor},
    Pixel,
};

use crate::command_1320::{DisplaySize1320, InitConfig1320, IrefSetting1320, SIZE_160X160};
use crate::error::DisplayError;
use crate::interface::{AsyncDisplayInterface, DisplayInterface};

/// Maximum buffer: 160 columns × 160 rows × 4bpp = 160/2 × 160 = 12800 bytes
const BUFFER_SIZE_1320: usize = 80 * 160;

/// SSD1320 OLED display driver.
///
/// 160x160 dot matrix, 16 grayscale (4-bit per pixel) OLED/PLED driver.
/// Each byte in the buffer stores 2 horizontal pixels (high nibble = left pixel,
/// low nibble = right pixel in default non-remapped mode).
///
/// Requires external VCC (8-18V).
///
/// Generic over the bus interface `DI` (I2C or SPI).
/// Owns an internal framebuffer sized for the configured display dimensions.
pub struct Ssd1320<DI> {
    interface: DI,
    buffer: [u8; BUFFER_SIZE_1320],
    size: DisplaySize1320,
}

impl<DI> Ssd1320<DI> {
    /// Create a new SSD1320 driver with default 160x160 display size.
    pub fn new(interface: DI) -> Self {
        Self {
            interface,
            buffer: [0u8; BUFFER_SIZE_1320],
            size: SIZE_160X160,
        }
    }

    /// Create a new SSD1320 driver with a custom display size.
    pub fn new_with_size(interface: DI, size: DisplaySize1320) -> Self {
        Self {
            interface,
            buffer: [0u8; BUFFER_SIZE_1320],
            size,
        }
    }

    pub fn release(self) -> DI {
        self.interface
    }

    /// SSD1320 §6.7 — Set a pixel in the internal framebuffer.
    /// Each byte holds 2 pixels (4 bits each). Gray value 0-15.
    pub fn set_pixel(&mut self, x: u8, y: u8, gray: u8) {
        if x >= self.size.width || y >= self.size.height {
            return;
        }
        let col = (x / 2) as usize;
        let row = y as usize;
        let columns = self.size.columns() as usize;
        let idx = row * columns + col;
        let value = gray & 0x0F;
        if x.is_multiple_of(2) {
            self.buffer[idx] = (self.buffer[idx] & 0x0F) | (value << 4);
        } else {
            self.buffer[idx] = (self.buffer[idx] & 0xF0) | value;
        }
    }

    pub fn clear_buffer(&mut self) {
        let len = self.size.columns() as usize * self.size.height as usize;
        self.buffer[..len].fill(0);
    }

    pub fn buffer(&self) -> &[u8] {
        let len = self.size.columns() as usize * self.size.height as usize;
        &self.buffer[..len]
    }

    pub fn size_config(&self) -> DisplaySize1320 {
        self.size
    }
}

// --- Synchronous API ---

impl<DI: DisplayInterface> Ssd1320<DI> {
    /// SSD1320 §6.9 — Power ON initialization sequence.
    pub fn init(&mut self, config: &InitConfig1320) -> Result<(), DisplayError> {
        // SSD1320 §6.9 — Display OFF
        self.interface.send_command(&[0xAE])?;
        // Set display clock divide ratio / oscillator frequency (D5h)
        self.interface.send_command(&[
            0xD5,
            (config.osc_freq << 4) | (config.clock_divide_ratio & 0x0F),
        ])?;
        // Set multiplex ratio (A8h) — height - 1
        self.interface.send_command(&[0xA8, self.size.height - 1])?;
        // Set display offset (D3h)
        self.interface
            .send_command(&[0xD3, config.display_offset])?;
        // Set display start line (40h)
        self.interface.send_command(&[0x40])?;
        // Set addressing mode (20h)
        self.interface
            .send_command(&[0x20, config.addressing_mode as u8])?;
        // Set segment remap (A0h/A1h)
        self.interface
            .send_command(&[if config.column_remap { 0xA1 } else { 0xA0 }])?;
        // Set COM output scan direction (C0h/C8h)
        self.interface
            .send_command(&[if config.com_remap { 0xC8 } else { 0xC0 }])?;
        // Set contrast (81h)
        self.interface.send_command(&[0x81, config.contrast])?;
        // Set pre-charge period (D9h)
        self.interface
            .send_command(&[0xD9, (config.phase2 << 4) | (config.phase1 & 0x0F)])?;
        // Set IREF selection (ADh)
        self.interface.send_command(&[0xAD, config.iref as u8])?;
        // Entire display ON (A4h/A5h)
        self.interface
            .send_command(&[if config.entire_display_on { 0xA5 } else { 0xA4 }])?;
        // Normal/inverse display (A6h/A7h)
        self.interface
            .send_command(&[if config.inverse_display { 0xA7 } else { 0xA6 }])?;

        if config.display_on {
            self.interface.send_command(&[0xAF])?;
        }
        Ok(())
    }

    /// Flush the internal framebuffer to the display GDDRAM.
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        // SSD1320 — Set column address (15h): start=0, end=columns-1
        self.interface
            .send_command(&[0x15, 0x00, self.size.columns() - 1])?;
        // SSD1320 — Set row address (75h): start=0, end=height-1
        self.interface
            .send_command(&[0x75, 0x00, self.size.height - 1])?;
        let len = self.size.columns() as usize * self.size.height as usize;
        self.interface.send_data(&self.buffer[..len])
    }

    /// Turn the display panel ON or OFF (sleep mode)
    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.interface.send_command(&[if on { 0xAF } else { 0xAE }])
    }

    /// Set contrast (0-255)
    pub fn set_contrast(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x81, contrast])
    }

    /// Set inverse display
    pub fn set_inverse(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if inverse { 0xA7 } else { 0xA6 }])
    }

    /// Set IREF selection (external or internal)
    pub fn set_iref(&mut self, setting: IrefSetting1320) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xAD, setting as u8])
    }
}

// --- Async API ---

impl<DI: AsyncDisplayInterface> Ssd1320<DI> {
    /// Async power ON initialization sequence.
    pub async fn init_async(&mut self, config: &InitConfig1320) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xAE]).await?;
        self.interface
            .send_command(&[
                0xD5,
                (config.osc_freq << 4) | (config.clock_divide_ratio & 0x0F),
            ])
            .await?;
        self.interface
            .send_command(&[0xA8, self.size.height - 1])
            .await?;
        self.interface
            .send_command(&[0xD3, config.display_offset])
            .await?;
        self.interface.send_command(&[0x40]).await?;
        self.interface
            .send_command(&[0x20, config.addressing_mode as u8])
            .await?;
        self.interface
            .send_command(&[if config.column_remap { 0xA1 } else { 0xA0 }])
            .await?;
        self.interface
            .send_command(&[if config.com_remap { 0xC8 } else { 0xC0 }])
            .await?;
        self.interface
            .send_command(&[0x81, config.contrast])
            .await?;
        self.interface
            .send_command(&[0xD9, (config.phase2 << 4) | (config.phase1 & 0x0F)])
            .await?;
        self.interface
            .send_command(&[0xAD, config.iref as u8])
            .await?;
        self.interface
            .send_command(&[if config.entire_display_on { 0xA5 } else { 0xA4 }])
            .await?;
        self.interface
            .send_command(&[if config.inverse_display { 0xA7 } else { 0xA6 }])
            .await?;

        if config.display_on {
            self.interface.send_command(&[0xAF]).await?;
        }
        Ok(())
    }

    /// Async flush of framebuffer to display GDDRAM.
    pub async fn flush_async(&mut self) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[0x15, 0x00, self.size.columns() - 1])
            .await?;
        self.interface
            .send_command(&[0x75, 0x00, self.size.height - 1])
            .await?;
        let len = self.size.columns() as usize * self.size.height as usize;
        self.interface.send_data(&self.buffer[..len]).await
    }

    /// Async display ON/OFF
    pub async fn display_on_async(&mut self, on: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if on { 0xAF } else { 0xAE }])
            .await
    }

    /// Async set contrast
    pub async fn set_contrast_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x81, contrast]).await
    }

    /// Async set inverse
    pub async fn set_inverse_async(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if inverse { 0xA7 } else { 0xA6 }])
            .await
    }
}

// --- embedded-graphics DrawTarget ---

impl<DI> OriginDimensions for Ssd1320<DI> {
    fn size(&self) -> Size {
        Size::new(self.size.width as u32, self.size.height as u32)
    }
}

impl<DI> DrawTarget for Ssd1320<DI> {
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
