use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::{Gray4, GrayColor},
    Pixel,
};

use crate::command_1325::{CurrentRange1325, DisplaySize1325, InitConfig1325, SIZE_128X80};
use crate::error::DisplayError;
use crate::interface::{AsyncDisplayInterface, DisplayInterface};

/// Maximum buffer: 128 pixels / 2 × 80 rows = 5120 bytes
const BUFFER_SIZE_1325: usize = 64 * 80;

/// SSD1325 OLED display driver.
///
/// 128x80 dot matrix, 16 grayscale (4-bit per pixel) OLED/PLED driver.
/// Each byte in the buffer stores 2 horizontal pixels (high nibble = left pixel,
/// low nibble = right pixel in default non-remapped mode).
///
/// Requires external VCC (8-16V) and external IREF via resistor.
///
/// Generic over the bus interface `DI` (I2C or SPI).
/// Owns an internal framebuffer sized for the configured display dimensions.
pub struct Ssd1325<DI> {
    interface: DI,
    buffer: [u8; BUFFER_SIZE_1325],
    size: DisplaySize1325,
}

impl<DI> Ssd1325<DI> {
    /// Create a new SSD1325 driver with default 128x80 display size.
    pub fn new(interface: DI) -> Self {
        Self {
            interface,
            buffer: [0u8; BUFFER_SIZE_1325],
            size: SIZE_128X80,
        }
    }

    /// Create a new SSD1325 driver with a custom display size.
    pub fn new_with_size(interface: DI, size: DisplaySize1325) -> Self {
        Self {
            interface,
            buffer: [0u8; BUFFER_SIZE_1325],
            size,
        }
    }

    pub fn release(self) -> DI {
        self.interface
    }

    /// SSD1325 §8.7 — Set a pixel in the internal framebuffer.
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

    pub fn size_config(&self) -> DisplaySize1325 {
        self.size
    }
}

// --- Synchronous API ---

impl<DI: DisplayInterface> Ssd1325<DI> {
    /// SSD1325 §8.9 — Power ON initialization sequence.
    pub fn init(&mut self, config: &InitConfig1325) -> Result<(), DisplayError> {
        // SSD1325 §10.1.11 — Display OFF
        self.interface.send_command(&[0xAE])?;
        // SSD1325 §10.1.16 — Set display clock divide ratio / oscillator frequency
        self.interface.send_command(&[
            0xB3,
            (config.osc_freq << 4) | (config.clock_divide_ratio & 0x0F),
        ])?;
        // SSD1325 §10.1.9 — Set multiplex ratio (height - 1)
        self.interface.send_command(&[0xA8, self.size.height - 1])?;
        // SSD1325 §10.1.7 — Set display offset
        self.interface
            .send_command(&[0xA2, config.display_offset])?;
        // SSD1325 §10.1.6 — Set display start line
        self.interface
            .send_command(&[0xA1, config.display_start_line])?;
        // SSD1325 §10.1.5 — Set re-map
        self.interface.send_command(&[0xA0, config.remap_byte()])?;
        // SSD1325 §10.1.3 — Set contrast current
        self.interface.send_command(&[0x81, config.contrast])?;
        // SSD1325 §10.1.4 — Set current range
        self.interface.send_command(&[config.current_range as u8])?;
        // SSD1325 §10.1.14 — Set phase length
        self.interface
            .send_command(&[0xB1, (config.phase2 << 4) | (config.phase1 & 0x0F)])?;
        // SSD1325 §10.1.13 — Set precharge voltage
        self.interface
            .send_command(&[0xBC, config.precharge_voltage])?;
        // SSD1325 §10.1.12 — Set VCOMH voltage
        self.interface.send_command(&[0xBE, config.vcomh])?;
        // SSD1325 §10.1.10 — Set master configuration (external VCC)
        self.interface.send_command(&[0xAD, 0x02])?;
        // SSD1325 §10.1.8 — Set display mode
        let mode_cmd = if config.entire_display_on {
            0xA5
        } else if config.inverse_display {
            0xA7
        } else {
            0xA4
        };
        self.interface.send_command(&[mode_cmd])?;

        if config.display_on {
            // SSD1325 §10.1.11 — Display ON
            self.interface.send_command(&[0xAF])?;
        }
        Ok(())
    }

    /// Flush the internal framebuffer to the display GDDRAM.
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        // SSD1325 §10.1.1 — Set column address: start=0, end=columns-1
        self.interface
            .send_command(&[0x15, 0x00, self.size.columns() - 1])?;
        // SSD1325 §10.1.2 — Set row address: start=0, end=height-1
        self.interface
            .send_command(&[0x75, 0x00, self.size.height - 1])?;
        let len = self.size.columns() as usize * self.size.height as usize;
        self.interface.send_data(&self.buffer[..len])
    }

    /// SSD1325 §10.1.11 — Turn the display panel ON or OFF (sleep mode)
    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.interface.send_command(&[if on { 0xAF } else { 0xAE }])
    }

    /// SSD1325 §10.1.3 — Set contrast current (0-127)
    pub fn set_contrast(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x81, contrast & 0x7F])
    }

    /// SSD1325 §10.1.8 — Set normal or inverse display
    pub fn set_inverse(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if inverse { 0xA7 } else { 0xA4 }])
    }

    /// SSD1325 §10.1.4 — Set current range (quarter, half, full)
    pub fn set_current_range(&mut self, range: CurrentRange1325) -> Result<(), DisplayError> {
        self.interface.send_command(&[range as u8])
    }

    /// SSD1325 §10.1.12 — Set VCOMH deselect level (5-bit value, 0-31)
    pub fn set_vcomh(&mut self, level: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xBE, level & 0x1F])
    }

    /// SSD1325 §10.1.13 — Set precharge voltage (8-bit value)
    pub fn set_precharge_voltage(&mut self, voltage: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xBC, voltage])
    }
}

// --- Async API ---

impl<DI: AsyncDisplayInterface> Ssd1325<DI> {
    /// Async power ON initialization sequence.
    pub async fn init_async(&mut self, config: &InitConfig1325) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xAE]).await?;
        self.interface
            .send_command(&[
                0xB3,
                (config.osc_freq << 4) | (config.clock_divide_ratio & 0x0F),
            ])
            .await?;
        self.interface
            .send_command(&[0xA8, self.size.height - 1])
            .await?;
        self.interface
            .send_command(&[0xA2, config.display_offset])
            .await?;
        self.interface
            .send_command(&[0xA1, config.display_start_line])
            .await?;
        self.interface
            .send_command(&[0xA0, config.remap_byte()])
            .await?;
        self.interface
            .send_command(&[0x81, config.contrast])
            .await?;
        self.interface
            .send_command(&[config.current_range as u8])
            .await?;
        self.interface
            .send_command(&[0xB1, (config.phase2 << 4) | (config.phase1 & 0x0F)])
            .await?;
        self.interface
            .send_command(&[0xBC, config.precharge_voltage])
            .await?;
        self.interface.send_command(&[0xBE, config.vcomh]).await?;
        self.interface.send_command(&[0xAD, 0x02]).await?;
        let mode_cmd = if config.entire_display_on {
            0xA5
        } else if config.inverse_display {
            0xA7
        } else {
            0xA4
        };
        self.interface.send_command(&[mode_cmd]).await?;

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
        self.interface.send_command(&[0x81, contrast & 0x7F]).await
    }

    /// Async set inverse
    pub async fn set_inverse_async(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if inverse { 0xA7 } else { 0xA4 }])
            .await
    }
}

// --- embedded-graphics DrawTarget ---

impl<DI> OriginDimensions for Ssd1325<DI> {
    fn size(&self) -> Size {
        Size::new(self.size.width as u32, self.size.height as u32)
    }
}

impl<DI> DrawTarget for Ssd1325<DI> {
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
