use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::BinaryColor,
    Pixel,
};

use crate::command::{
    AddressingMode, ComPinConfig, Command, ScrollDirection, ScrollInterval, VcomhLevel,
};
use crate::error::DisplayError;
use crate::interface::{AsyncDisplayInterface, DisplayInterface};
use crate::{HEIGHT, PAGES, WIDTH};

const BUFFER_SIZE: usize = WIDTH as usize * PAGES as usize;

/// SSD1306 OLED display driver.
///
/// Generic over the bus interface `DI` (I2C or SPI).
/// Owns an internal framebuffer (1024 bytes for 128x64).
pub struct Ssd1306<DI> {
    interface: DI,
    buffer: [u8; BUFFER_SIZE],
}

impl<DI> Ssd1306<DI> {
    pub fn new(interface: DI) -> Self {
        Self {
            interface,
            buffer: [0u8; BUFFER_SIZE],
        }
    }

    pub fn release(self) -> DI {
        self.interface
    }

    /// Set a pixel in the internal framebuffer.
    /// SSD1306 §8.7 — Each byte represents 8 vertical pixels (D0=top, D7=bottom) within one page.
    pub fn set_pixel(&mut self, x: u8, y: u8, on: bool) {
        if x >= WIDTH || y >= HEIGHT {
            return;
        }
        let page = (y / 8) as usize;
        let bit = y % 8;
        let idx = page * WIDTH as usize + x as usize;
        if on {
            self.buffer[idx] |= 1 << bit;
        } else {
            self.buffer[idx] &= !(1 << bit);
        }
    }

    pub fn clear_buffer(&mut self) {
        self.buffer = [0u8; BUFFER_SIZE];
    }

    pub fn buffer(&self) -> &[u8; BUFFER_SIZE] {
        &self.buffer
    }
}

// --- Synchronous API ---

impl<DI: DisplayInterface> Ssd1306<DI> {
    fn send_command(&mut self, cmd: Command) -> Result<(), DisplayError> {
        let (bytes, len) = cmd.bytes();
        self.interface.send_command(&bytes[..len])
    }

    /// SSD1306 §8.9 — Power ON initialization sequence.
    /// Configures the display for 128x64, horizontal addressing, charge pump enabled.
    pub fn init(&mut self) -> Result<(), DisplayError> {
        // SSD1306 §10.1.12 — Display OFF during init
        self.send_command(Command::DisplayOn(false))?;
        // SSD1306 §10.1.16 — Clock: default osc freq (0x8), divide ratio 1
        self.send_command(Command::SetDisplayClock {
            divide_ratio: 0,
            osc_freq: 0x8,
        })?;
        // SSD1306 §10.1.11 — MUX ratio = 63 (64 lines)
        self.send_command(Command::SetMultiplexRatio(63))?;
        // SSD1306 §10.1.15 — No display offset
        self.send_command(Command::SetDisplayOffset(0))?;
        // SSD1306 §10.1.6 — Start line = 0
        self.send_command(Command::SetDisplayStartLine(0))?;
        // SSD1306 §8.9 — Enable charge pump (required for most modules)
        self.send_command(Command::SetChargePump(true))?;
        // SSD1306 §10.1.3 — Horizontal addressing mode
        self.send_command(Command::SetAddressingMode(AddressingMode::Horizontal))?;
        // SSD1306 §10.1.8 — Segment remap: column 127 mapped to SEG0
        self.send_command(Command::SetSegmentRemap(true))?;
        // SSD1306 §10.1.14 — COM scan remapped (C8h)
        self.send_command(Command::SetComScanDirection(true))?;
        // SSD1306 §10.1.18 — Alternative COM pin config, no L/R remap
        self.send_command(Command::SetComPinConfig(ComPinConfig::default()))?;
        // SSD1306 §10.1.7 — Contrast = 0x7F (RESET)
        self.send_command(Command::SetContrast(0x7F))?;
        // SSD1306 §10.1.17 — Pre-charge: phase1=1, phase2=15
        self.send_command(Command::SetPreChargePeriod {
            phase1: 0x1,
            phase2: 0xF,
        })?;
        // SSD1306 §10.1.19 — VCOMH deselect ~0.77 x VCC
        self.send_command(Command::SetVcomhDeselect(VcomhLevel::V077))?;
        // SSD1306 §10.1.9 — Output follows RAM content
        self.send_command(Command::EntireDisplayOn(false))?;
        // SSD1306 §10.1.10 — Normal display (not inverted)
        self.send_command(Command::InverseDisplay(false))?;
        // SSD1306 §10.2.3 — Deactivate scrolling
        self.send_command(Command::DeactivateScroll)?;
        // SSD1306 §10.1.12 — Display ON
        self.send_command(Command::DisplayOn(true))?;
        Ok(())
    }

    /// Flush the internal framebuffer to the display GDDRAM.
    /// SSD1306 §10.1.4/10.1.5 — Sets column and page address to cover the full display,
    /// then writes the entire buffer.
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        self.send_command(Command::SetColumnAddress {
            start: 0,
            end: WIDTH - 1,
        })?;
        self.send_command(Command::SetPageAddress {
            start: 0,
            end: PAGES - 1,
        })?;
        self.interface.send_data(&self.buffer)
    }

    /// SSD1306 §10.1.12 — Turn the display panel ON or OFF (sleep mode)
    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.send_command(Command::DisplayOn(on))
    }

    /// SSD1306 §10.1.7 — Set contrast (0-255)
    pub fn set_contrast(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.send_command(Command::SetContrast(contrast))
    }

    /// SSD1306 §10.1.10 — Invert the display (swap ON/OFF pixels)
    pub fn set_inverse(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.send_command(Command::InverseDisplay(inverse))
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
        self.send_command(Command::DeactivateScroll)?;
        let cmd_byte = match direction {
            ScrollDirection::Right => 0x26,
            ScrollDirection::Left => 0x27,
        };
        self.interface.send_command(&[
            cmd_byte,
            0x00,
            start_page,
            interval as u8,
            end_page,
            0x00,
            0xFF,
        ])?;
        // SSD1306 §10.2.4 — Activate scroll (2Fh)
        self.interface.send_command(&[0x2F])
    }

    /// SSD1306 §10.2.3 — Stop scrolling
    pub fn stop_scroll(&mut self) -> Result<(), DisplayError> {
        self.send_command(Command::DeactivateScroll)
    }
}

// --- Async API ---

impl<DI: AsyncDisplayInterface> Ssd1306<DI> {
    async fn send_command_async(&mut self, cmd: Command) -> Result<(), DisplayError> {
        let (bytes, len) = cmd.bytes();
        self.interface.send_command(&bytes[..len]).await
    }

    /// SSD1306 §8.9 — Async power ON initialization sequence.
    pub async fn init_async(&mut self) -> Result<(), DisplayError> {
        self.send_command_async(Command::DisplayOn(false)).await?;
        self.send_command_async(Command::SetDisplayClock {
            divide_ratio: 0,
            osc_freq: 0x8,
        })
        .await?;
        self.send_command_async(Command::SetMultiplexRatio(63))
            .await?;
        self.send_command_async(Command::SetDisplayOffset(0))
            .await?;
        self.send_command_async(Command::SetDisplayStartLine(0))
            .await?;
        self.send_command_async(Command::SetChargePump(true))
            .await?;
        self.send_command_async(Command::SetAddressingMode(AddressingMode::Horizontal))
            .await?;
        self.send_command_async(Command::SetSegmentRemap(true))
            .await?;
        self.send_command_async(Command::SetComScanDirection(true))
            .await?;
        self.send_command_async(Command::SetComPinConfig(ComPinConfig::default()))
            .await?;
        self.send_command_async(Command::SetContrast(0x7F)).await?;
        self.send_command_async(Command::SetPreChargePeriod {
            phase1: 0x1,
            phase2: 0xF,
        })
        .await?;
        self.send_command_async(Command::SetVcomhDeselect(VcomhLevel::V077))
            .await?;
        self.send_command_async(Command::EntireDisplayOn(false))
            .await?;
        self.send_command_async(Command::InverseDisplay(false))
            .await?;
        self.send_command_async(Command::DeactivateScroll).await?;
        self.send_command_async(Command::DisplayOn(true)).await?;
        Ok(())
    }

    /// Async flush of framebuffer to display GDDRAM.
    pub async fn flush_async(&mut self) -> Result<(), DisplayError> {
        self.send_command_async(Command::SetColumnAddress {
            start: 0,
            end: WIDTH - 1,
        })
        .await?;
        self.send_command_async(Command::SetPageAddress {
            start: 0,
            end: PAGES - 1,
        })
        .await?;
        self.interface.send_data(&self.buffer).await
    }

    /// SSD1306 §10.1.12 — Async display ON/OFF
    pub async fn display_on_async(&mut self, on: bool) -> Result<(), DisplayError> {
        self.send_command_async(Command::DisplayOn(on)).await
    }

    /// SSD1306 §10.1.7 — Async set contrast
    pub async fn set_contrast_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.send_command_async(Command::SetContrast(contrast))
            .await
    }

    /// SSD1306 §10.1.10 — Async set inverse
    pub async fn set_inverse_async(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.send_command_async(Command::InverseDisplay(inverse))
            .await
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
