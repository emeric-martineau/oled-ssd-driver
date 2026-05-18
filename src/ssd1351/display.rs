use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::{IntoStorage, Rgb565},
    Pixel,
};

use super::init::{DisplaySize1351, InitConfig, SIZE_128X128};
use crate::error::DisplayError;
use crate::interface::{AsyncDisplayInterface, DisplayInterface};

/// Maximum buffer: 128 × 128 × 2 bytes (65K color, 16 bits per pixel) = 32768 bytes
const BUFFER_SIZE_1351: usize = 128 * 128 * 2;

/// SSD1351 OLED display driver.
///
/// 128x128 dot matrix, 65K color (RGB565) OLED/PLED driver.
/// Each pixel in the buffer occupies 2 bytes in big-endian RGB565 format.
///
/// Features 256-step contrast per color channel and 16-step master current.
///
/// Generic over the bus interface `DI` (SPI only — SSD1351 serial interface is write-only).
/// Owns an internal framebuffer sized for the configured display dimensions.
pub struct Ssd1351<DI> {
    interface: DI,
    buffer: [u8; BUFFER_SIZE_1351],
    size: DisplaySize1351,
}

impl<DI> Ssd1351<DI> {
    /// Create a new SSD1351 driver with default 128x128 display size.
    pub fn new(interface: DI) -> Self {
        Self {
            interface,
            buffer: [0u8; BUFFER_SIZE_1351],
            size: SIZE_128X128,
        }
    }

    /// Create a new SSD1351 driver with a custom display size.
    pub fn new_with_size(interface: DI, size: DisplaySize1351) -> Self {
        Self {
            interface,
            buffer: [0u8; BUFFER_SIZE_1351],
            size,
        }
    }

    pub fn release(self) -> DI {
        self.interface
    }

    /// SSD1351 §8.3 — Set a pixel in the internal framebuffer.
    /// Color is stored as big-endian RGB565 (2 bytes per pixel).
    pub fn set_pixel(&mut self, x: u8, y: u8, color: Rgb565) {
        if x >= self.size.width || y >= self.size.height {
            return;
        }
        let idx = (y as usize * self.size.width as usize + x as usize) * 2;
        let raw: u16 = color.into_storage();
        self.buffer[idx] = (raw >> 8) as u8;
        self.buffer[idx + 1] = raw as u8;
    }

    pub fn clear_buffer(&mut self) {
        let len = self.size.width as usize * self.size.height as usize * 2;
        self.buffer[..len].fill(0);
    }

    pub fn buffer(&self) -> &[u8] {
        let len = self.size.width as usize * self.size.height as usize * 2;
        &self.buffer[..len]
    }

    pub fn size_config(&self) -> DisplaySize1351 {
        self.size
    }
}

// --- Synchronous API ---

impl<DI: DisplayInterface> Ssd1351<DI> {
    /// SSD1351 §8.9 — Power ON initialization sequence.
    pub fn init(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        // SSD1351 §10.1.23 — Unlock OLED driver IC (FDh)
        self.interface.send_command(&[0xFD])?;
        self.interface.send_data(&[0x12])?;
        // SSD1351 §10.1.23 — Unlock commands A2,B1,B3,BB,BE,C1
        self.interface.send_command(&[0xFD])?;
        self.interface.send_data(&[0xB1])?;
        // SSD1351 §10.1.10 — Display OFF (sleep mode)
        self.interface.send_command(&[0xAE])?;
        // SSD1351 §10.1.13 — Set Front Clock Divider / Oscillator Frequency
        self.interface.send_command(&[0xB3])?;
        self.interface
            .send_data(&[(config.osc_freq & 0x0F) << 4 | (config.clock_divider & 0x0F)])?;
        // SSD1351 §10.1.22 — Set Multiplex Ratio
        self.interface.send_command(&[0xCA])?;
        self.interface.send_data(&[self.size.height - 1])?;
        // SSD1351 §10.1.5 — Set Re-map & Color Depth
        self.interface.send_command(&[0xA0])?;
        self.interface.send_data(&[config.remap_byte()])?;
        // SSD1351 §10.1.1 — Set Column Address
        self.interface.send_command(&[0x15])?;
        self.interface.send_data(&[0x00, self.size.width - 1])?;
        // SSD1351 §10.1.2 — Set Row Address
        self.interface.send_command(&[0x75])?;
        self.interface.send_data(&[0x00, self.size.height - 1])?;
        // SSD1351 §10.1.6 — Set Display Start Line
        self.interface.send_command(&[0xA1])?;
        self.interface
            .send_data(&[config.display_start_line & 0x7F])?;
        // SSD1351 §10.1.7 — Set Display Offset
        self.interface.send_command(&[0xA2])?;
        self.interface.send_data(&[config.display_offset & 0x7F])?;
        // SSD1351 §10.1.14 — Set GPIO (disabled)
        self.interface.send_command(&[0xB5])?;
        self.interface.send_data(&[0x00])?;
        // SSD1351 §10.1.9 — Function Selection (internal VDD regulator)
        self.interface.send_command(&[0xAB])?;
        self.interface
            .send_data(&[if config.internal_vdd { 0x01 } else { 0x00 }])?;
        // SSD1351 §10.1.11 — Set Phase Length
        self.interface.send_command(&[0xB1])?;
        self.interface
            .send_data(&[(config.phase2 & 0x0F) << 4 | (config.phase1 & 0x0F)])?;
        // SSD1351 §10.1.12 — Display Enhancement
        self.interface.send_command(&[0xB2])?;
        self.interface.send_data(&[0xA4, 0x00, 0x00])?;
        // SSD1351 §10.1.20 — Set Contrast Current for Color A, B, C
        self.interface.send_command(&[0xC1])?;
        self.interface
            .send_data(&[config.contrast_a, config.contrast_b, config.contrast_c])?;
        // SSD1351 §10.1.21 — Master Contrast Current Control
        self.interface.send_command(&[0xC7])?;
        self.interface.send_data(&[config.master_contrast & 0x0F])?;
        // SSD1351 §10.1.17 — Use Built-in Linear LUT
        self.interface.send_command(&[0xB9])?;
        // SSD1351 §10.1.18 — Set Pre-charge Voltage
        self.interface.send_command(&[0xBB])?;
        self.interface
            .send_data(&[config.precharge_voltage & 0x1F])?;
        // SSD1351 §10.1.19 — Set VCOMH Voltage
        self.interface.send_command(&[0xBE])?;
        self.interface.send_data(&[config.vcomh & 0x07])?;
        // SSD1351 §10.1.15 — Set Second Pre-charge Period
        self.interface.send_command(&[0xB6])?;
        self.interface
            .send_data(&[config.second_precharge & 0x0F])?;
        // SSD1351 §10.1.8 — Set Display Mode (normal/inverse)
        let mode_cmd = if config.inverse_display { 0xA7 } else { 0xA6 };
        self.interface.send_command(&[mode_cmd])?;

        if config.display_on {
            // SSD1351 §10.1.10 — Display ON
            self.interface.send_command(&[0xAF])?;
        }
        Ok(())
    }

    /// Flush the internal framebuffer to the display GDDRAM.
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        // SSD1351 §10.1.1 — Set Column Address: start=0, end=width-1
        self.interface.send_command(&[0x15])?;
        self.interface.send_data(&[0x00, self.size.width - 1])?;
        // SSD1351 §10.1.2 — Set Row Address: start=0, end=height-1
        self.interface.send_command(&[0x75])?;
        self.interface.send_data(&[0x00, self.size.height - 1])?;
        // SSD1351 §10.1.3 — Write RAM Command
        self.interface.send_command(&[0x5C])?;
        let len = self.size.width as usize * self.size.height as usize * 2;
        self.interface.send_data(&self.buffer[..len])
    }

    /// SSD1351 §10.1.10 — Turn the display panel ON or OFF (sleep mode)
    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.interface.send_command(&[if on { 0xAF } else { 0xAE }])
    }

    /// SSD1351 §10.1.20 — Set contrast for color A (0-255)
    pub fn set_contrast_a(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xC1])?;
        self.interface.send_data(&[contrast, 0xFF, 0xFF])
    }

    /// SSD1351 §10.1.20 — Set contrast for color B (0-255)
    pub fn set_contrast_b(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xC1])?;
        self.interface.send_data(&[0xFF, contrast, 0xFF])
    }

    /// SSD1351 §10.1.20 — Set contrast for color C (0-255)
    pub fn set_contrast_c(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xC1])?;
        self.interface.send_data(&[0xFF, 0xFF, contrast])
    }

    /// SSD1351 §10.1.20 — Set contrast for all three color channels at once
    pub fn set_contrast_abc(&mut self, a: u8, b: u8, c: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xC1])?;
        self.interface.send_data(&[a, b, c])
    }

    /// SSD1351 §10.1.21 — Set master contrast current control (0-15)
    pub fn set_master_contrast(&mut self, current: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xC7])?;
        self.interface.send_data(&[current & 0x0F])
    }

    /// SSD1351 §10.1.8 — Set normal or inverse display
    pub fn set_inverse(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if inverse { 0xA7 } else { 0xA6 }])
    }

    /// SSD1351 §10.1.19 — Set VCOMH deselect level (3-bit value, 0-7)
    pub fn set_vcomh(&mut self, level: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xBE])?;
        self.interface.send_data(&[level & 0x07])
    }

    /// SSD1351 §10.1.18 — Set precharge voltage (5-bit value, 0-31)
    pub fn set_precharge_voltage(&mut self, voltage: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xBB])?;
        self.interface.send_data(&[voltage & 0x1F])
    }
}

// --- Async API ---

impl<DI: AsyncDisplayInterface> Ssd1351<DI> {
    /// Async power ON initialization sequence.
    pub async fn init_async(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xFD]).await?;
        self.interface.send_data(&[0x12]).await?;
        self.interface.send_command(&[0xFD]).await?;
        self.interface.send_data(&[0xB1]).await?;
        self.interface.send_command(&[0xAE]).await?;
        self.interface.send_command(&[0xB3]).await?;
        self.interface
            .send_data(&[(config.osc_freq & 0x0F) << 4 | (config.clock_divider & 0x0F)])
            .await?;
        self.interface.send_command(&[0xCA]).await?;
        self.interface.send_data(&[self.size.height - 1]).await?;
        self.interface.send_command(&[0xA0]).await?;
        self.interface.send_data(&[config.remap_byte()]).await?;
        self.interface.send_command(&[0x15]).await?;
        self.interface
            .send_data(&[0x00, self.size.width - 1])
            .await?;
        self.interface.send_command(&[0x75]).await?;
        self.interface
            .send_data(&[0x00, self.size.height - 1])
            .await?;
        self.interface.send_command(&[0xA1]).await?;
        self.interface
            .send_data(&[config.display_start_line & 0x7F])
            .await?;
        self.interface.send_command(&[0xA2]).await?;
        self.interface
            .send_data(&[config.display_offset & 0x7F])
            .await?;
        self.interface.send_command(&[0xB5]).await?;
        self.interface.send_data(&[0x00]).await?;
        self.interface.send_command(&[0xAB]).await?;
        self.interface
            .send_data(&[if config.internal_vdd { 0x01 } else { 0x00 }])
            .await?;
        self.interface.send_command(&[0xB1]).await?;
        self.interface
            .send_data(&[(config.phase2 & 0x0F) << 4 | (config.phase1 & 0x0F)])
            .await?;
        self.interface.send_command(&[0xB2]).await?;
        self.interface.send_data(&[0xA4, 0x00, 0x00]).await?;
        self.interface.send_command(&[0xC1]).await?;
        self.interface
            .send_data(&[config.contrast_a, config.contrast_b, config.contrast_c])
            .await?;
        self.interface.send_command(&[0xC7]).await?;
        self.interface
            .send_data(&[config.master_contrast & 0x0F])
            .await?;
        self.interface.send_command(&[0xB9]).await?;
        self.interface.send_command(&[0xBB]).await?;
        self.interface
            .send_data(&[config.precharge_voltage & 0x1F])
            .await?;
        self.interface.send_command(&[0xBE]).await?;
        self.interface.send_data(&[config.vcomh & 0x07]).await?;
        self.interface.send_command(&[0xB6]).await?;
        self.interface
            .send_data(&[config.second_precharge & 0x0F])
            .await?;
        let mode_cmd = if config.inverse_display { 0xA7 } else { 0xA6 };
        self.interface.send_command(&[mode_cmd]).await?;

        if config.display_on {
            self.interface.send_command(&[0xAF]).await?;
        }
        Ok(())
    }

    /// Async flush of framebuffer to display GDDRAM.
    pub async fn flush_async(&mut self) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x15]).await?;
        self.interface
            .send_data(&[0x00, self.size.width - 1])
            .await?;
        self.interface.send_command(&[0x75]).await?;
        self.interface
            .send_data(&[0x00, self.size.height - 1])
            .await?;
        self.interface.send_command(&[0x5C]).await?;
        let len = self.size.width as usize * self.size.height as usize * 2;
        self.interface.send_data(&self.buffer[..len]).await
    }

    /// Async display ON/OFF
    pub async fn display_on_async(&mut self, on: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if on { 0xAF } else { 0xAE }])
            .await
    }

    /// Async set contrast for all three color channels
    pub async fn set_contrast_abc_async(
        &mut self,
        a: u8,
        b: u8,
        c: u8,
    ) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xC1]).await?;
        self.interface.send_data(&[a, b, c]).await
    }

    /// Async set master contrast
    pub async fn set_master_contrast_async(&mut self, current: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xC7]).await?;
        self.interface.send_data(&[current & 0x0F]).await
    }

    /// Async set inverse
    pub async fn set_inverse_async(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if inverse { 0xA7 } else { 0xA6 }])
            .await
    }
}

// --- embedded-graphics DrawTarget ---

impl<DI> OriginDimensions for Ssd1351<DI> {
    fn size(&self) -> Size {
        Size::new(self.size.width as u32, self.size.height as u32)
    }
}

impl<DI> DrawTarget for Ssd1351<DI> {
    type Color = Rgb565;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let w = self.size.width as i32;
        let h = self.size.height as i32;
        for Pixel(coord, color) in pixels {
            if coord.x >= 0 && coord.x < w && coord.y >= 0 && coord.y < h {
                self.set_pixel(coord.x as u8, coord.y as u8, color);
            }
        }
        Ok(())
    }
}
