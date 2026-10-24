use super::init::DEFAULT_DISPLAY_SIZE;
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::BinaryColor,
    Pixel,
};

use super::command::{Command, ScrollDirection, ScrollInterval};
use crate::common::{
    Buffered, DisplayBufferMode, DisplayCore, DisplaySize, InitConfig, Unbuffered,
};
use crate::error::DisplayError;
use crate::interface::{AsyncDisplayInterface, DisplayInterface};

/// SSD1306 OLED display driver.
///
/// Generic over the bus interface `DI` (I2C or SPI) and buffer mode `MODE`.
/// In `Buffered` mode (default) the driver owns a framebuffer and supports all drawing APIs.
/// In `Unbuffered` mode the driver owns no RAM buffer; only control commands are available.
pub struct Ssd1306<DI, MODE: DisplayBufferMode = Buffered> {
    core: DisplayCore<DI, MODE>,
}

// --- Constructors shared across modes ---

impl<DI> Ssd1306<DI, Buffered> {
    /// Create a new SSD1306 driver with default 128x64 display size (buffered).
    pub fn new(interface: DI) -> Self {
        Self {
            core: DisplayCore::new(interface, DEFAULT_DISPLAY_SIZE),
        }
    }

    /// Create a new SSD1306 driver with a custom display size (buffered).
    pub fn new_with_size(interface: DI, size: DisplaySize) -> Self {
        Self {
            core: DisplayCore::new(interface, size),
        }
    }
}

impl<DI> Ssd1306<DI, Unbuffered> {
    /// Create a new SSD1306 driver in unbuffered mode with default 128x64 display size.
    pub fn new_unbuffered(interface: DI) -> Self {
        Self {
            core: DisplayCore::new(interface, DEFAULT_DISPLAY_SIZE),
        }
    }

    /// Create a new SSD1306 driver in unbuffered mode with a custom display size.
    pub fn new_unbuffered_with_size(interface: DI, size: DisplaySize) -> Self {
        Self {
            core: DisplayCore::new(interface, size),
        }
    }
}

impl<DI> Ssd1306<DI, Buffered> {
    pub fn release(self) -> DI {
        self.core.interface
    }

    /// Set a pixel in the internal framebuffer.
    /// SSD1306 §8.7 — Each byte represents 8 vertical pixels (D0=top, D7=bottom) within one page.
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

impl<DI> Ssd1306<DI, Unbuffered> {
    pub fn release(self) -> DI {
        self.core.interface
    }

    pub fn size_config(&self) -> DisplaySize {
        self.core.size
    }
}

// --- Synchronous API (Buffered) ---

impl<DI: DisplayInterface> Ssd1306<DI, Buffered> {
    fn send_command(&mut self, cmd: Command) -> Result<(), DisplayError> {
        self.core.send_command(cmd)
    }

    /// SSD1306 §8.9 — Power ON initialization sequence.
    pub fn init(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        self.core.init_common(config)?;
        self.send_command(Command::SetChargePump(true))?;
        if config.display_on {
            self.send_command(Command::DisplayOn(true))?;
        }
        Ok(())
    }

    /// Flush the internal framebuffer to the display GDDRAM.
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        self.core.flush()
    }

    /// SSD1306 §10.1.12 — Turn the display panel ON or OFF (sleep mode)
    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.core.display_on(on)
    }

    /// SSD1306 §10.1.7 — Set contrast (0-255)
    pub fn set_contrast(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.core.set_contrast(contrast)
    }

    /// SSD1306 §10.1.10 — Invert the display (swap ON/OFF pixels)
    pub fn set_inverse(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.core.set_inverse(inverse)
    }

    /// SSD1306 §10.2.1 — Configure and activate horizontal scrolling
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

    /// SSD1306 §10.2.3 — Stop scrolling
    pub fn stop_scroll(&mut self) -> Result<(), DisplayError> {
        self.core.stop_scroll()
    }
}

// --- Synchronous API (Unbuffered) ---

impl<DI: DisplayInterface> Ssd1306<DI, Unbuffered> {
    fn send_command(&mut self, cmd: Command) -> Result<(), DisplayError> {
        self.core.send_command(cmd)
    }

    /// SSD1306 §8.9 — Power ON initialization sequence (unbuffered).
    pub fn init(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        self.core.init_common(config)?;
        self.send_command(Command::SetChargePump(true))?;
        if config.display_on {
            self.send_command(Command::DisplayOn(true))?;
        }
        Ok(())
    }

    /// SSD1306 §10.1.12 — Turn the display panel ON or OFF (sleep mode)
    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.core.display_on(on)
    }

    /// SSD1306 §10.1.7 — Set contrast (0-255)
    pub fn set_contrast(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.core.set_contrast(contrast)
    }

    /// SSD1306 §10.1.10 — Invert the display (swap ON/OFF pixels)
    pub fn set_inverse(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.core.set_inverse(inverse)
    }

    /// SSD1306 §10.2.1 — Configure and activate horizontal scrolling
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

    /// SSD1306 §10.2.3 — Stop scrolling
    pub fn stop_scroll(&mut self) -> Result<(), DisplayError> {
        self.core.stop_scroll()
    }
}

// --- Async API (Buffered) ---

impl<DI: AsyncDisplayInterface> Ssd1306<DI, Buffered> {
    /// SSD1306 §8.9 — Async power ON initialization sequence.
    pub async fn init_async(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        self.core.init_common_async(config).await?;
        self.core
            .send_command_async(Command::SetChargePump(true))
            .await?;
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

    /// SSD1306 §10.1.12 — Async display ON/OFF
    pub async fn display_on_async(&mut self, on: bool) -> Result<(), DisplayError> {
        self.core.display_on_async(on).await
    }

    /// SSD1306 §10.1.7 — Async set contrast
    pub async fn set_contrast_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.core.set_contrast_async(contrast).await
    }

    /// SSD1306 §10.1.10 — Async set inverse
    pub async fn set_inverse_async(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.core.set_inverse_async(inverse).await
    }
}

// --- Async API (Unbuffered) ---

impl<DI: AsyncDisplayInterface> Ssd1306<DI, Unbuffered> {
    /// SSD1306 §8.9 — Async power ON initialization sequence (unbuffered).
    pub async fn init_async(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        self.core.init_common_async(config).await?;
        self.core
            .send_command_async(Command::SetChargePump(true))
            .await?;
        if config.display_on {
            self.core
                .send_command_async(Command::DisplayOn(true))
                .await?;
        }
        Ok(())
    }

    /// SSD1306 §10.1.12 — Async display ON/OFF
    pub async fn display_on_async(&mut self, on: bool) -> Result<(), DisplayError> {
        self.core.display_on_async(on).await
    }

    /// SSD1306 §10.1.7 — Async set contrast
    pub async fn set_contrast_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.core.set_contrast_async(contrast).await
    }

    /// SSD1306 §10.1.10 — Async set inverse
    pub async fn set_inverse_async(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.core.set_inverse_async(inverse).await
    }
}

// --- embedded-graphics DrawTarget (Buffered only) ---

impl<DI, MODE: DisplayBufferMode> OriginDimensions for Ssd1306<DI, MODE> {
    fn size(&self) -> Size {
        Size::new(self.core.size.width as u32, self.core.size.height as u32)
    }
}

impl<DI> DrawTarget for Ssd1306<DI, Buffered> {
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
