use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::{IntoStorage, Rgb565},
    Pixel,
};

use super::init::{DisplaySize1332, InitConfig, SIZE_96X64};
use crate::error::DisplayError;
use crate::interface::{AsyncDisplayInterface, DisplayInterface};

/// Maximum buffer: 96 × 64 × 2 bytes (65K color, 16 bits per pixel) = 12288 bytes
const BUFFER_SIZE_1332: usize = 96 * 64 * 2;

/// SSD1332 OLED display driver.
///
/// 96x64 dot matrix, 65K color (RGB565) OLED/PLED driver.
/// Each pixel in the buffer occupies 2 bytes in big-endian RGB565 format.
///
/// Features 256-step contrast per color channel, 16-step master current,
/// internal DC-DC booster, and separate VPA/VPB/VPC pre-charge voltages.
///
/// Generic over the bus interface `DI` (SPI only — SSD1332 serial interface is write-only).
/// Owns an internal framebuffer sized for the configured display dimensions.
pub struct Ssd1332<DI> {
    interface: DI,
    buffer: [u8; BUFFER_SIZE_1332],
    size: DisplaySize1332,
}

impl<DI> Ssd1332<DI> {
    /// Create a new SSD1332 driver with default 96x64 display size.
    pub fn new(interface: DI) -> Self {
        Self {
            interface,
            buffer: [0u8; BUFFER_SIZE_1332],
            size: SIZE_96X64,
        }
    }

    /// Create a new SSD1332 driver with a custom display size.
    pub fn new_with_size(interface: DI, size: DisplaySize1332) -> Self {
        Self {
            interface,
            buffer: [0u8; BUFFER_SIZE_1332],
            size,
        }
    }

    pub fn release(self) -> DI {
        self.interface
    }

    /// SSD1332 §7 — Set a pixel in the internal framebuffer.
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

    pub fn size_config(&self) -> DisplaySize1332 {
        self.size
    }
}

// --- Synchronous API ---

impl<DI: DisplayInterface> Ssd1332<DI> {
    /// SSD1332 §7 — Power ON initialization sequence.
    pub fn init(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        // SSD1332 §8 — Display OFF (sleep mode)
        self.interface.send_command(&[0xAE])?;
        // SSD1332 §9 — Set Re-map & Data Format
        self.interface.send_command(&[0xA0, config.remap_byte()])?;
        // SSD1332 §9 — Set Display Start Line
        self.interface
            .send_command(&[0xA1, config.display_start_line & 0x3F])?;
        // SSD1332 §9 — Set Display Offset
        self.interface
            .send_command(&[0xA2, config.display_offset & 0x3F])?;
        // SSD1332 §9 — Set Display Mode (normal/inverse)
        let mode_cmd = if config.inverse_display { 0xA7 } else { 0xA4 };
        self.interface.send_command(&[mode_cmd])?;
        // SSD1332 §9 — Set Multiplex Ratio
        self.interface.send_command(&[0xA8, self.size.height - 1])?;
        // SSD1332 §9 — Set Master Configuration (booster + VP selection)
        self.interface
            .send_command(&[0xAD, config.master_config_byte()])?;
        // SSD1332 §9 — Set Power Save Mode (disabled)
        self.interface.send_command(&[0xB0, 0x00])?;
        // SSD1332 §9 — Set Phase 1 and 2 Period
        self.interface
            .send_command(&[0xB1, (config.phase2 & 0x0F) << 4 | (config.phase1 & 0x0F)])?;
        // SSD1332 §9 — Set Display Clock Divide Ratio / Oscillator Frequency
        self.interface.send_command(&[
            0xB3,
            (config.osc_freq & 0x0F) << 4 | (config.clock_divide_ratio & 0x0F),
        ])?;
        // SSD1332 §9 — Set Contrast for Color A
        self.interface.send_command(&[0x81, config.contrast_a])?;
        // SSD1332 §9 — Set Contrast for Color B
        self.interface.send_command(&[0x82, config.contrast_b])?;
        // SSD1332 §9 — Set Contrast for Color C
        self.interface.send_command(&[0x83, config.contrast_c])?;
        // SSD1332 §9 — Set Master Current Control
        self.interface
            .send_command(&[0x87, config.master_current & 0x0F])?;
        // SSD1332 §9 — Enable Linear Gray Scale Table
        self.interface.send_command(&[0xB9])?;
        // SSD1332 §9 — Set VPA Pre-charge Voltage for Color A
        self.interface.send_command(&[0xBB, config.vpa])?;
        // SSD1332 §9 — Set VPB Pre-charge Voltage for Color B
        self.interface.send_command(&[0xBC, config.vpb])?;
        // SSD1332 §9 — Set VPC Pre-charge Voltage for Color C
        self.interface.send_command(&[0xBD, config.vpc])?;
        // SSD1332 §9 — Set VCOMH Voltage
        self.interface.send_command(&[0xBE, config.vcomh & 0x3F])?;

        if config.display_on {
            // SSD1332 §8 — Display ON in normal mode
            self.interface.send_command(&[0xAF])?;
        }
        Ok(())
    }

    /// Flush the internal framebuffer to the display GDDRAM.
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        // SSD1332 §9 — Set Column Address: start=0, end=width-1
        self.interface
            .send_command(&[0x15, 0x00, self.size.width - 1])?;
        // SSD1332 §9 — Set Row Address: start=0, end=height-1
        self.interface
            .send_command(&[0x75, 0x00, self.size.height - 1])?;
        let len = self.size.width as usize * self.size.height as usize * 2;
        self.interface.send_data(&self.buffer[..len])
    }

    /// SSD1332 §8 — Turn the display panel ON or OFF (sleep mode)
    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.interface.send_command(&[if on { 0xAF } else { 0xAE }])
    }

    /// SSD1332 §9 — Set contrast for color A (0-255)
    pub fn set_contrast_a(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x81, contrast])
    }

    /// SSD1332 §9 — Set contrast for color B (0-255)
    pub fn set_contrast_b(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x82, contrast])
    }

    /// SSD1332 §9 — Set contrast for color C (0-255)
    pub fn set_contrast_c(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x83, contrast])
    }

    /// SSD1332 §9 — Set master current attenuation factor (0-15)
    pub fn set_master_current(&mut self, current: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x87, current & 0x0F])
    }

    /// SSD1332 §9 — Set normal or inverse display
    pub fn set_inverse(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if inverse { 0xA7 } else { 0xA4 }])
    }

    /// SSD1332 §9 — Set VCOMH deselect level (6-bit value, 0-63)
    pub fn set_vcomh(&mut self, level: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xBE, level & 0x3F])
    }

    /// SSD1332 §9 — Set VPA pre-charge voltage for color A (0-255)
    pub fn set_vpa(&mut self, voltage: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xBB, voltage])
    }

    /// SSD1332 §9 — Set VPB pre-charge voltage for color B (0-255)
    pub fn set_vpb(&mut self, voltage: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xBC, voltage])
    }

    /// SSD1332 §9 — Set VPC pre-charge voltage for color C (0-255)
    pub fn set_vpc(&mut self, voltage: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xBD, voltage])
    }

    /// SSD1332 §8 — Set power save mode (true = enabled, false = disabled)
    pub fn set_power_save(&mut self, enabled: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[0xB0, if enabled { 0x12 } else { 0x00 }])
    }
}

// --- Async API ---

impl<DI: AsyncDisplayInterface> Ssd1332<DI> {
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
        self.interface
            .send_command(&[0xAD, config.master_config_byte()])
            .await?;
        self.interface.send_command(&[0xB0, 0x00]).await?;
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
        self.interface.send_command(&[0xBB, config.vpa]).await?;
        self.interface.send_command(&[0xBC, config.vpb]).await?;
        self.interface.send_command(&[0xBD, config.vpc]).await?;
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

impl<DI> OriginDimensions for Ssd1332<DI> {
    fn size(&self) -> Size {
        Size::new(self.size.width as u32, self.size.height as u32)
    }
}

impl<DI> DrawTarget for Ssd1332<DI> {
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
