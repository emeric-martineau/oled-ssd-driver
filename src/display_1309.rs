use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::BinaryColor,
    Pixel,
};

use crate::command_1306::{Command, ScrollDirection, ScrollInterval};
use crate::command_1309::VcomhLevel1309;
use crate::display_common::{DisplayCore, DisplaySize, InitConfig, SIZE_128X64};
use crate::error::DisplayError;
use crate::interface::{AsyncDisplayInterface, DisplayInterface};

/// SSD1309 OLED display driver.
///
/// 128x64 dot matrix OLED/PLED driver. Unlike SSD1306, the SSD1309 does not have
/// an internal charge pump — it requires external VCC (7-18V).
///
/// Generic over the bus interface `DI` (I2C or SPI).
/// Owns an internal framebuffer sized for the configured display dimensions.
pub struct Ssd1309<DI> {
    core: DisplayCore<DI>,
}

impl<DI> Ssd1309<DI> {
    /// Create a new SSD1309 driver with default 128x64 display size.
    pub fn new(interface: DI) -> Self {
        Self {
            core: DisplayCore::new(interface, SIZE_128X64),
        }
    }

    /// Create a new SSD1309 driver with a custom display size.
    pub fn new_with_size(interface: DI, size: DisplaySize) -> Self {
        Self {
            core: DisplayCore::new(interface, size),
        }
    }

    pub fn release(self) -> DI {
        self.core.interface
    }

    /// SSD1309 §8.7 — Each byte represents 8 vertical pixels (D0=top, D7=bottom) within one page.
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

impl<DI: DisplayInterface> Ssd1309<DI> {
    fn send_command(&mut self, cmd: Command) -> Result<(), DisplayError> {
        self.core.send_command(cmd)
    }

    /// SSD1309 §8.8 — Power ON initialization sequence.
    /// No charge pump (external VCC required).
    pub fn init(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        self.core.init_common(config)?;
        if config.display_on {
            self.send_command(Command::DisplayOn(true))?;
        }
        Ok(())
    }

    /// Flush the internal framebuffer to the display GDDRAM.
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        self.core.flush()
    }

    /// SSD1309 §9.1 — Turn the display panel ON or OFF (sleep mode)
    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.core.display_on(on)
    }

    /// SSD1309 §10.7 — Set contrast (0-255)
    pub fn set_contrast(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.core.set_contrast(contrast)
    }

    /// SSD1309 §10.10 — Invert the display (swap ON/OFF pixels)
    pub fn set_inverse(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.core.set_inverse(inverse)
    }

    /// SSD1309 §9.2 — Configure and activate horizontal scrolling
    pub fn start_horizontal_scroll(
        &mut self,
        direction: ScrollDirection,
        start_page: u8,
        end_page: u8,
        interval: ScrollInterval,
    ) -> Result<(), DisplayError> {
        let max_page = self.core.size.pages() - 1;
        if start_page > max_page || end_page > max_page || end_page < start_page {
            return Err(DisplayError::InvalidArgument);
        }
        self.core.stop_scroll()?;
        let cmd_byte = match direction {
            ScrollDirection::Right => 0x26,
            ScrollDirection::Left => 0x27,
        };
        self.core.interface.send_command(&[
            cmd_byte,
            0x00,
            start_page,
            interval as u8,
            end_page,
            0x00,
            0xFF,
        ])?;
        self.core.interface.send_command(&[0x2F])
    }

    /// SSD1309 §9.2 — Stop scrolling
    pub fn stop_scroll(&mut self) -> Result<(), DisplayError> {
        self.core.stop_scroll()
    }

    /// SSD1309 §9.4 — Set VCOMH deselect level (DBh).
    /// SSD1309 has different VCOMH levels than SSD1306.
    pub fn set_vcomh_deselect(&mut self, level: VcomhLevel1309) -> Result<(), DisplayError> {
        self.core.interface.send_command(&[0xDB, level as u8])
    }
}

// --- Async API ---

impl<DI: AsyncDisplayInterface> Ssd1309<DI> {
    /// SSD1309 §8.8 — Async power ON initialization sequence.
    pub async fn init_async(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        self.core.init_common_async(config).await?;
        if config.display_on {
            self.core
                .send_command_async(Command::DisplayOn(true))
                .await?;
        }
        Ok(())
    }

    /// Async flush of framebuffer to display GDDRAM.
    pub async fn flush_async(&mut self) -> Result<(), DisplayError> {
        self.core.flush_async().await
    }

    /// SSD1309 §9.1 — Async display ON/OFF
    pub async fn display_on_async(&mut self, on: bool) -> Result<(), DisplayError> {
        self.core.display_on_async(on).await
    }

    /// SSD1309 §10.7 — Async set contrast
    pub async fn set_contrast_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.core.set_contrast_async(contrast).await
    }

    /// SSD1309 §10.10 — Async set inverse
    pub async fn set_inverse_async(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.core.set_inverse_async(inverse).await
    }
}

// --- embedded-graphics DrawTarget ---

impl<DI> OriginDimensions for Ssd1309<DI> {
    fn size(&self) -> Size {
        Size::new(self.core.size.width as u32, self.core.size.height as u32)
    }
}

impl<DI> DrawTarget for Ssd1309<DI> {
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
