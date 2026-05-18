use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::BinaryColor,
    Pixel,
};

use super::init::{InitConfig, ScrollInterval1300, SIZE_104X48};
use crate::common::{DisplayCore, DisplaySize};
use crate::error::DisplayError;
use crate::interface::{AsyncDisplayInterface, DisplayInterface};

/// SSD1300 OLED display driver.
///
/// 104x48 dot matrix, monochrome OLED/PLED driver with internal DC-DC booster.
/// Uses page-mode addressing (no horizontal/vertical addressing mode command).
/// Supports 4 area colour selections and horizontal scrolling.
///
/// Generic over the bus interface `DI` (SPI — SSD1300 serial interface is write-only).
/// Owns an internal framebuffer sized for the configured display dimensions.
pub struct Ssd1300<DI> {
    core: DisplayCore<DI>,
}

impl<DI> Ssd1300<DI> {
    /// Create a new SSD1300 driver with default 104x48 display size.
    pub fn new(interface: DI) -> Self {
        Self {
            core: DisplayCore::new(interface, SIZE_104X48),
        }
    }

    /// Create a new SSD1300 driver with a custom display size.
    pub fn new_with_size(interface: DI, size: DisplaySize) -> Self {
        Self {
            core: DisplayCore::new(interface, size),
        }
    }

    pub fn release(self) -> DI {
        self.core.interface
    }

    /// SSD1300 §7 — Each byte represents 8 vertical pixels (D0=top, D7=bottom) within one page.
    pub fn set_pixel(&mut self, x: u8, y: u8, on: bool) {
        self.core.set_pixel(x, y, on);
    }

    pub fn clear_buffer(&mut self) {
        self.core.clear_buffer();
    }

    pub fn buffer(&self) -> &[u8] {
        self.core.active_buffer()
    }

    pub fn size_config(&self) -> DisplaySize {
        self.core.size
    }
}

// --- Synchronous API ---

impl<DI: DisplayInterface> Ssd1300<DI> {
    fn send_raw_command(&mut self, bytes: &[u8]) -> Result<(), DisplayError> {
        self.core.interface.send_command(bytes)
    }

    /// SSD1300 §7 — Power ON initialization sequence.
    pub fn init(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        // SSD1300 §8 — Display OFF
        self.send_raw_command(&[0xAE])?;
        // SSD1300 §8 — Set Display Clock Divide Ratio / Oscillator Frequency (D5h)
        self.send_raw_command(&[
            0xD5,
            (config.osc_freq & 0x0F) << 4 | (config.clock_divide_ratio & 0x0F),
        ])?;
        // SSD1300 §8 — Set Multiplex Ratio (A8h)
        self.send_raw_command(&[0xA8, (config.multiplex_ratio - 1) & 0x3F])?;
        // SSD1300 §8 — Set Display Offset (D3h)
        self.send_raw_command(&[0xD3, config.display_offset & 0x3F])?;
        // SSD1300 §8 — Set Display Start Line (40h-6Fh)
        self.send_raw_command(&[0x40 | (config.display_start_line & 0x3F)])?;
        // SSD1300 §8 — Set DC-DC on/off (ADh)
        let dc_dc_byte = if config.dc_dc_enable { 0x01 } else { 0x00 }
            | if config.internal_vcomh { 0x02 } else { 0x00 };
        self.send_raw_command(&[0xAD, dc_dc_byte])?;
        // SSD1300 §8 — Set Segment Re-map (A0h/A1h)
        self.send_raw_command(&[if config.segment_remap { 0xA1 } else { 0xA0 }])?;
        // SSD1300 §8 — Set COM Output Scan Direction (C0h/C8h)
        self.send_raw_command(&[if config.com_scan_reversed { 0xC8 } else { 0xC0 }])?;
        // SSD1300 §8 — Set COM Pins Hardware Configuration (DAh)
        let com_pin_byte = if config.com_pin_alternative {
            0x12
        } else {
            0x02
        };
        self.send_raw_command(&[0xDA, com_pin_byte])?;
        // SSD1300 §8 — Set Contrast Control Register (81h)
        self.send_raw_command(&[0x81, config.contrast])?;
        // SSD1300 §8 — Set Pre-charge Period (D9h)
        self.send_raw_command(&[0xD9, (config.phase2 & 0x0F) << 4 | (config.phase1 & 0x0F)])?;
        // SSD1300 §8 — Set VCOMH Deselect Level (DBh)
        self.send_raw_command(&[0xDB, config.vcomh & 0x7F])?;
        // SSD1300 §8 — Set Entire Display On/Off (A4h/A5h)
        self.send_raw_command(&[if config.entire_display_on { 0xA5 } else { 0xA4 }])?;
        // SSD1300 §8 — Set Normal/Inverse Display (A6h/A7h)
        self.send_raw_command(&[if config.inverse_display { 0xA7 } else { 0xA6 }])?;
        // SSD1300 §8 — Deactivate Horizontal Scroll (2Eh)
        if config.deactivate_scroll {
            self.send_raw_command(&[0x2E])?;
        }
        if config.display_on {
            // SSD1300 §8 — Display ON (AEh/AFh)
            self.send_raw_command(&[0xAF])?;
        }
        Ok(())
    }

    /// SSD1300 — Flush the internal framebuffer to the display GDDRAM.
    /// Uses page-mode addressing: for each page, set page address then column address, then send data.
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        let pages = self.core.size.pages();
        let width = self.core.size.width;

        for page in 0..pages {
            // SSD1300 §8 — Set Page Address (B0h-B5h)
            self.send_raw_command(&[0xB0 | page])?;
            // SSD1300 §8 — Set Lower Column Address (00h-0Fh)
            self.send_raw_command(&[0x00])?;
            // SSD1300 §8 — Set Higher Column Address (10h-1Fh)
            self.send_raw_command(&[0x10])?;

            let start = page as usize * width as usize;
            let end = start + width as usize;
            self.core
                .interface
                .send_data(&self.core.buffer[start..end])?;
        }
        Ok(())
    }

    /// SSD1300 §8 — Turn the display panel ON or OFF (sleep mode)
    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.send_raw_command(&[if on { 0xAF } else { 0xAE }])
    }

    /// SSD1300 §8 — Set contrast (0-255)
    pub fn set_contrast(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.send_raw_command(&[0x81, contrast])
    }

    /// SSD1300 §8 — Set normal or inverse display
    pub fn set_inverse(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.send_raw_command(&[if inverse { 0xA7 } else { 0xA6 }])
    }

    /// SSD1300 §8 — Set VCOMH deselect level (7-bit value, 0-127)
    pub fn set_vcomh(&mut self, level: u8) -> Result<(), DisplayError> {
        self.send_raw_command(&[0xDB, level & 0x7F])
    }

    /// SSD1300 §8 — Set DC-DC on/off and VCOMH source
    pub fn set_dc_dc(
        &mut self,
        enable: bool,
        internal_vcomh: bool,
    ) -> Result<(), DisplayError> {
        let byte = if enable { 0x01 } else { 0x00 }
            | if internal_vcomh { 0x02 } else { 0x00 };
        self.send_raw_command(&[0xAD, byte])
    }

    /// SSD1300 §8 — Configure and activate horizontal scrolling
    pub fn start_horizontal_scroll(
        &mut self,
        start_page: u8,
        end_page: u8,
        num_columns: u8,
        interval: ScrollInterval1300,
    ) -> Result<(), DisplayError> {
        let max_page = self.core.size.pages() - 1;
        if start_page > max_page || end_page > max_page || end_page < start_page {
            return Err(DisplayError::InvalidArgument);
        }
        if num_columns == 0 || num_columns > 0x0F {
            return Err(DisplayError::InvalidArgument);
        }
        // SSD1300 §8 — Deactivate scroll first (2Eh)
        self.send_raw_command(&[0x2E])?;
        // SSD1300 §8 — Horizontal Scroll Setup (26h)
        self.send_raw_command(&[
            0x26,
            num_columns & 0x0F,
            start_page & 0x07,
            interval as u8,
            end_page & 0x07,
        ])?;
        // SSD1300 §8 — Activate Horizontal Scroll (2Fh)
        self.send_raw_command(&[0x2F])
    }

    /// SSD1300 §8 — Stop scrolling (2Eh)
    pub fn stop_scroll(&mut self) -> Result<(), DisplayError> {
        self.send_raw_command(&[0x2E])
    }
}

// --- Async API ---

impl<DI: AsyncDisplayInterface> Ssd1300<DI> {
    async fn send_raw_command_async(&mut self, bytes: &[u8]) -> Result<(), DisplayError> {
        self.core.interface.send_command(bytes).await
    }

    /// SSD1300 §7 — Async power ON initialization sequence.
    pub async fn init_async(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        self.send_raw_command_async(&[0xAE]).await?;
        self.send_raw_command_async(&[
            0xD5,
            (config.osc_freq & 0x0F) << 4 | (config.clock_divide_ratio & 0x0F),
        ])
        .await?;
        self.send_raw_command_async(&[0xA8, (config.multiplex_ratio - 1) & 0x3F])
            .await?;
        self.send_raw_command_async(&[0xD3, config.display_offset & 0x3F])
            .await?;
        self.send_raw_command_async(&[0x40 | (config.display_start_line & 0x3F)])
            .await?;
        let dc_dc_byte = if config.dc_dc_enable { 0x01 } else { 0x00 }
            | if config.internal_vcomh { 0x02 } else { 0x00 };
        self.send_raw_command_async(&[0xAD, dc_dc_byte]).await?;
        self.send_raw_command_async(&[if config.segment_remap { 0xA1 } else { 0xA0 }])
            .await?;
        self.send_raw_command_async(&[if config.com_scan_reversed { 0xC8 } else { 0xC0 }])
            .await?;
        let com_pin_byte = if config.com_pin_alternative {
            0x12
        } else {
            0x02
        };
        self.send_raw_command_async(&[0xDA, com_pin_byte]).await?;
        self.send_raw_command_async(&[0x81, config.contrast])
            .await?;
        self.send_raw_command_async(&[0xD9, (config.phase2 & 0x0F) << 4 | (config.phase1 & 0x0F)])
            .await?;
        self.send_raw_command_async(&[0xDB, config.vcomh & 0x7F])
            .await?;
        self.send_raw_command_async(&[if config.entire_display_on { 0xA5 } else { 0xA4 }])
            .await?;
        self.send_raw_command_async(&[if config.inverse_display { 0xA7 } else { 0xA6 }])
            .await?;
        if config.deactivate_scroll {
            self.send_raw_command_async(&[0x2E]).await?;
        }
        if config.display_on {
            self.send_raw_command_async(&[0xAF]).await?;
        }
        Ok(())
    }

    /// SSD1300 — Async flush of framebuffer to display GDDRAM (page mode).
    pub async fn flush_async(&mut self) -> Result<(), DisplayError> {
        let pages = self.core.size.pages();
        let width = self.core.size.width;

        for page in 0..pages {
            self.send_raw_command_async(&[0xB0 | page]).await?;
            self.send_raw_command_async(&[0x00]).await?;
            self.send_raw_command_async(&[0x10]).await?;

            let start = page as usize * width as usize;
            let end = start + width as usize;
            self.core
                .interface
                .send_data(&self.core.buffer[start..end])
                .await?;
        }
        Ok(())
    }

    /// SSD1300 §8 — Async display ON/OFF
    pub async fn display_on_async(&mut self, on: bool) -> Result<(), DisplayError> {
        self.send_raw_command_async(&[if on { 0xAF } else { 0xAE }])
            .await
    }

    /// SSD1300 §8 — Async set contrast
    pub async fn set_contrast_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.send_raw_command_async(&[0x81, contrast]).await
    }

    /// SSD1300 §8 — Async set inverse
    pub async fn set_inverse_async(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.send_raw_command_async(&[if inverse { 0xA7 } else { 0xA6 }])
            .await
    }
}

// --- embedded-graphics DrawTarget ---

impl<DI> OriginDimensions for Ssd1300<DI> {
    fn size(&self) -> Size {
        Size::new(self.core.size.width as u32, self.core.size.height as u32)
    }
}

impl<DI> DrawTarget for Ssd1300<DI> {
    type Color = BinaryColor;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let w = self.core.size.width as i32;
        let h = self.core.size.height as i32;
        for Pixel(coord, color) in pixels {
            if coord.x >= 0 && coord.x < w && coord.y >= 0 && coord.y < h {
                self.set_pixel(coord.x as u8, coord.y as u8, color.is_on());
            }
        }
        Ok(())
    }
}
