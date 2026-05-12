use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::BinaryColor,
    Pixel,
};

use crate::command::{Command, ScrollDirection, ScrollInterval};
use crate::display_common::{DisplayCore, BUFFER_SIZE};
use crate::error::DisplayError;
use crate::interface::{AsyncDisplayInterface, DisplayInterface};
use crate::{HEIGHT, WIDTH};

/// SSD1306 OLED display driver.
///
/// Generic over the bus interface `DI` (I2C or SPI).
/// Owns an internal framebuffer (1024 bytes for 128x64).
pub struct Ssd1306<DI> {
    core: DisplayCore<DI>,
}

impl<DI> Ssd1306<DI> {
    pub fn new(interface: DI) -> Self {
        Self {
            core: DisplayCore::new(interface),
        }
    }

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

    pub fn buffer(&self) -> &[u8; BUFFER_SIZE] {
        &self.core.buffer
    }
}

// --- Synchronous API ---

impl<DI: DisplayInterface> Ssd1306<DI> {
    fn send_command(&mut self, cmd: Command) -> Result<(), DisplayError> {
        self.core.send_command(cmd)
    }

    /// SSD1306 §8.9 — Power ON initialization sequence.
    pub fn init(&mut self) -> Result<(), DisplayError> {
        self.core.init_common()?;
        // SSD1306 §8.9 — Enable charge pump
        self.send_command(Command::SetChargePump(true))?;
        // SSD1306 §10.1.12 — Display ON
        self.send_command(Command::DisplayOn(true))?;
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
        if start_page > 7 || end_page > 7 || end_page < start_page {
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
        // SSD1306 §10.2.4 — Activate scroll (2Fh)
        self.core.interface.send_command(&[0x2F])
    }

    /// SSD1306 §10.2.3 — Stop scrolling
    pub fn stop_scroll(&mut self) -> Result<(), DisplayError> {
        self.core.stop_scroll()
    }
}

// --- Async API ---

impl<DI: AsyncDisplayInterface> Ssd1306<DI> {
    /// SSD1306 §8.9 — Async power ON initialization sequence.
    pub async fn init_async(&mut self) -> Result<(), DisplayError> {
        self.core.init_common_async().await?;
        self.core
            .send_command_async(Command::SetChargePump(true))
            .await?;
        self.core
            .send_command_async(Command::DisplayOn(true))
            .await?;
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

// --- embedded-graphics DrawTarget ---

impl<DI> OriginDimensions for Ssd1306<DI> {
    fn size(&self) -> Size {
        Size::new(WIDTH as u32, HEIGHT as u32)
    }
}

impl<DI> DrawTarget for Ssd1306<DI> {
    type Color = BinaryColor;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels {
            if coord.x >= 0 && coord.x < WIDTH as i32 && coord.y >= 0 && coord.y < HEIGHT as i32 {
                self.set_pixel(coord.x as u8, coord.y as u8, color.is_on());
            }
        }
        Ok(())
    }
}
