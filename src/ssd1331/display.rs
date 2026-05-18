use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::{IntoStorage, Rgb565},
    Pixel,
};

use super::init::{DisplaySize1331, InitConfig, SIZE_96X64};
use crate::error::DisplayError;
use crate::interface::{AsyncDisplayInterface, DisplayInterface};

/// Maximum buffer: 96 × 64 × 2 bytes (65K color, 16 bits per pixel) = 12288 bytes
const BUFFER_SIZE_1331: usize = 96 * 64 * 2;

/// SSD1331 OLED display driver.
///
/// 96x64 dot matrix, 65K color (RGB565) OLED/PLED driver.
/// Each pixel in the buffer occupies 2 bytes in big-endian RGB565 format.
///
/// Features 256-step contrast per color channel and 16-step master current.
///
/// Generic over the bus interface `DI` (SPI only — SSD1331 serial interface is write-only).
/// Owns an internal framebuffer sized for the configured display dimensions.
pub struct Ssd1331<DI> {
    interface: DI,
    buffer: [u8; BUFFER_SIZE_1331],
    size: DisplaySize1331,
}

impl<DI> Ssd1331<DI> {
    /// Create a new SSD1331 driver with default 96x64 display size.
    pub fn new(interface: DI) -> Self {
        Self {
            interface,
            buffer: [0u8; BUFFER_SIZE_1331],
            size: SIZE_96X64,
        }
    }

    /// Create a new SSD1331 driver with a custom display size.
    pub fn new_with_size(interface: DI, size: DisplaySize1331) -> Self {
        Self {
            interface,
            buffer: [0u8; BUFFER_SIZE_1331],
            size,
        }
    }

    pub fn release(self) -> DI {
        self.interface
    }

    /// SSD1331 §7.5 — Set a pixel in the internal framebuffer.
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

    pub fn size_config(&self) -> DisplaySize1331 {
        self.size
    }
}

// --- Synchronous API ---

impl<DI: DisplayInterface> Ssd1331<DI> {
    /// SSD1331 §7.9 — Power ON initialization sequence.
    pub fn init(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        // SSD1331 §9.1.13 — Display OFF (sleep mode)
        self.interface.send_command(&[0xAE])?;
        // SSD1331 §9.1.6 — Set Re-map & Color Depth
        self.interface.send_command(&[0xA0, config.remap_byte()])?;
        // SSD1331 §9.1.7 — Set Display Start Line
        self.interface
            .send_command(&[0xA1, config.display_start_line & 0x3F])?;
        // SSD1331 §9.1.8 — Set Display Offset
        self.interface
            .send_command(&[0xA2, config.display_offset & 0x3F])?;
        // SSD1331 §9.1.9 — Set Display Mode (normal/inverse)
        let mode_cmd = if config.inverse_display { 0xA7 } else { 0xA4 };
        self.interface.send_command(&[mode_cmd])?;
        // SSD1331 §9.1.10 — Set Multiplex Ratio
        self.interface.send_command(&[0xA8, self.size.height - 1])?;
        // SSD1331 §9.1.12 — Set Master Configuration (bit A[0] must be 0b after RESET)
        self.interface.send_command(&[0xAD, 0x8E])?;
        // SSD1331 §9.1.15 — Set Phase 1 and 2 Period
        self.interface
            .send_command(&[0xB1, (config.phase2 & 0x0F) << 4 | (config.phase1 & 0x0F)])?;
        // SSD1331 §9.1.16 — Set Display Clock Divide Ratio / Oscillator Frequency
        self.interface.send_command(&[
            0xB3,
            (config.osc_freq & 0x0F) << 4 | (config.clock_divide_ratio & 0x0F),
        ])?;
        // SSD1331 §9.1.5 — Set Second Pre-charge Speed for Color A, B, C
        self.interface.send_command(&[0x8A, config.contrast_a])?;
        self.interface.send_command(&[0x8B, config.contrast_b])?;
        self.interface.send_command(&[0x8C, config.contrast_c])?;
        // SSD1331 §9.1.3 — Set Contrast for Color A
        self.interface.send_command(&[0x81, config.contrast_a])?;
        // SSD1331 §9.1.3 — Set Contrast for Color B
        self.interface.send_command(&[0x82, config.contrast_b])?;
        // SSD1331 §9.1.3 — Set Contrast for Color C
        self.interface.send_command(&[0x83, config.contrast_c])?;
        // SSD1331 §9.1.4 — Set Master Current Control
        self.interface
            .send_command(&[0x87, config.master_current & 0x0F])?;
        // SSD1331 §9.1.18 — Enable Linear Gray Scale Table
        self.interface.send_command(&[0xB9])?;
        // SSD1331 §9.1.19 — Set Pre-charge Voltage
        self.interface
            .send_command(&[0xBB, config.precharge_voltage & 0x3F])?;
        // SSD1331 §9.1.20 — Set VCOMH Voltage
        self.interface.send_command(&[0xBE, config.vcomh & 0x3F])?;

        if config.display_on {
            // SSD1331 §9.1.13 — Display ON in normal mode
            self.interface.send_command(&[0xAF])?;
        }
        Ok(())
    }

    /// Flush the internal framebuffer to the display GDDRAM.
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        // SSD1331 §9.1.1 — Set Column Address: start=0, end=width-1
        self.interface
            .send_command(&[0x15, 0x00, self.size.width - 1])?;
        // SSD1331 §9.1.2 — Set Row Address: start=0, end=height-1
        self.interface
            .send_command(&[0x75, 0x00, self.size.height - 1])?;
        let len = self.size.width as usize * self.size.height as usize * 2;
        self.interface.send_data(&self.buffer[..len])
    }

    /// SSD1331 §9.1.13 — Turn the display panel ON or OFF (sleep mode)
    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.interface.send_command(&[if on { 0xAF } else { 0xAE }])
    }

    /// SSD1331 §9.1.3 — Set contrast for color A (0-255)
    pub fn set_contrast_a(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x81, contrast])
    }

    /// SSD1331 §9.1.3 — Set contrast for color B (0-255)
    pub fn set_contrast_b(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x82, contrast])
    }

    /// SSD1331 §9.1.3 — Set contrast for color C (0-255)
    pub fn set_contrast_c(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x83, contrast])
    }

    /// SSD1331 §9.1.4 — Set master current attenuation factor (0-15)
    pub fn set_master_current(&mut self, current: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x87, current & 0x0F])
    }

    /// SSD1331 §9.1.9 — Set normal or inverse display
    pub fn set_inverse(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if inverse { 0xA7 } else { 0xA4 }])
    }

    /// SSD1331 §9.1.20 — Set VCOMH deselect level (6-bit value, 0-63)
    pub fn set_vcomh(&mut self, level: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xBE, level & 0x3F])
    }

    /// SSD1331 §9.1.19 — Set precharge voltage (6-bit value, 0-63)
    pub fn set_precharge_voltage(&mut self, voltage: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xBB, voltage & 0x3F])
    }
}

// --- Async API ---

impl<DI: AsyncDisplayInterface> Ssd1331<DI> {
    /// Async power ON initialization sequence.
    pub async fn init_async(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xAE]).await?;
        self.interface
            .send_command(&[0xA0, config.remap_byte()])
            .await?;
        self.interface
            .send_command(&[0xA1, config.display_start_line & 0x3F])
            .await?;
        self.interface
            .send_command(&[0xA2, config.display_offset & 0x3F])
            .await?;
        let mode_cmd = if config.inverse_display { 0xA7 } else { 0xA4 };
        self.interface.send_command(&[mode_cmd]).await?;
        self.interface
            .send_command(&[0xA8, self.size.height - 1])
            .await?;
        self.interface.send_command(&[0xAD, 0x8E]).await?;
        self.interface
            .send_command(&[0xB1, (config.phase2 & 0x0F) << 4 | (config.phase1 & 0x0F)])
            .await?;
        self.interface
            .send_command(&[
                0xB3,
                (config.osc_freq & 0x0F) << 4 | (config.clock_divide_ratio & 0x0F),
            ])
            .await?;
        self.interface
            .send_command(&[0x8A, config.contrast_a])
            .await?;
        self.interface
            .send_command(&[0x8B, config.contrast_b])
            .await?;
        self.interface
            .send_command(&[0x8C, config.contrast_c])
            .await?;
        self.interface
            .send_command(&[0x81, config.contrast_a])
            .await?;
        self.interface
            .send_command(&[0x82, config.contrast_b])
            .await?;
        self.interface
            .send_command(&[0x83, config.contrast_c])
            .await?;
        self.interface
            .send_command(&[0x87, config.master_current & 0x0F])
            .await?;
        self.interface.send_command(&[0xB9]).await?;
        self.interface
            .send_command(&[0xBB, config.precharge_voltage & 0x3F])
            .await?;
        self.interface
            .send_command(&[0xBE, config.vcomh & 0x3F])
            .await?;

        if config.display_on {
            self.interface.send_command(&[0xAF]).await?;
        }
        Ok(())
    }

    /// Async flush of framebuffer to display GDDRAM.
    pub async fn flush_async(&mut self) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[0x15, 0x00, self.size.width - 1])
            .await?;
        self.interface
            .send_command(&[0x75, 0x00, self.size.height - 1])
            .await?;
        let len = self.size.width as usize * self.size.height as usize * 2;
        self.interface.send_data(&self.buffer[..len]).await
    }

    /// Async display ON/OFF
    pub async fn display_on_async(&mut self, on: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if on { 0xAF } else { 0xAE }])
            .await
    }

    /// Async set contrast for color A
    pub async fn set_contrast_a_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x81, contrast]).await
    }

    /// Async set contrast for color B
    pub async fn set_contrast_b_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x82, contrast]).await
    }

    /// Async set contrast for color C
    pub async fn set_contrast_c_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x83, contrast]).await
    }

    /// Async set inverse
    pub async fn set_inverse_async(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if inverse { 0xA7 } else { 0xA4 }])
            .await
    }
}

// --- embedded-graphics DrawTarget ---

impl<DI> OriginDimensions for Ssd1331<DI> {
    fn size(&self) -> Size {
        Size::new(self.size.width as u32, self.size.height as u32)
    }
}

impl<DI> DrawTarget for Ssd1331<DI> {
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
