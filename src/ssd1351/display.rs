use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::{IntoStorage, Rgb565},
    Pixel,
};

use super::init::{InitConfig, SIZE_128X128};
use crate::common::DisplaySize;
use crate::common::{Buffered, Unbuffered};
use crate::error::DisplayError;
use crate::interface::{AsyncDisplayInterface, DisplayInterface};

/// Maximum buffer: 128 × 128 × 2 bytes (65K color, 16 bits per pixel) = 32768 bytes
const BUFFER_SIZE_1351: usize = 128 * 128 * 2;

pub trait Ssd1351BufferMode {
    type Buffer;
    fn new_buffer() -> Self::Buffer;
}

impl Ssd1351BufferMode for Buffered {
    type Buffer = [u8; BUFFER_SIZE_1351];
    fn new_buffer() -> Self::Buffer {
        [0u8; BUFFER_SIZE_1351]
    }
}

impl Ssd1351BufferMode for Unbuffered {
    type Buffer = ();
    fn new_buffer() -> Self::Buffer {}
}

/// SSD1351 OLED display driver.
///
/// Generic over the bus interface `DI` (SPI only) and buffer mode `MODE`.
/// In `Buffered` mode (default) the driver owns a 32 KB framebuffer and supports all drawing APIs.
/// In `Unbuffered` mode the driver owns no RAM buffer; pixels are sent directly to GDDRAM
/// using per-pixel column/row window commands (slower, but ~32 KB RAM saving).
pub struct Ssd1351<DI, MODE: Ssd1351BufferMode = Buffered> {
    interface: DI,
    buffer: MODE::Buffer,
    size: DisplaySize,
}

impl<DI> Ssd1351<DI, Buffered> {
    /// Create a new SSD1351 driver with default 128x128 display size (buffered).
    pub fn new(interface: DI) -> Self {
        Self {
            interface,
            buffer: [0u8; BUFFER_SIZE_1351],
            size: SIZE_128X128,
        }
    }

    pub fn new_with_size(interface: DI, size: DisplaySize) -> Self {
        Self {
            interface,
            buffer: [0u8; BUFFER_SIZE_1351],
            size,
        }
    }

    pub fn release(self) -> DI {
        self.interface
    }

    /// SSD1351 §8.3 — Set a pixel in the internal framebuffer (big-endian RGB565).
    pub fn set_pixel(&mut self, x: u8, y: u8, color: Rgb565) {
        if u16::from(x) >= self.size.width || y >= self.size.height {
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

    pub fn size_config(&self) -> DisplaySize {
        self.size
    }
}

impl<DI> Ssd1351<DI, Unbuffered> {
    /// Create a new SSD1351 driver in unbuffered mode with default 128x128 display size.
    pub fn new_unbuffered(interface: DI) -> Self {
        Self {
            interface,
            buffer: (),
            size: SIZE_128X128,
        }
    }

    pub fn new_unbuffered_with_size(interface: DI, size: DisplaySize) -> Self {
        Self {
            interface,
            buffer: (),
            size,
        }
    }

    pub fn release(self) -> DI {
        self.interface
    }

    pub fn size_config(&self) -> DisplaySize {
        self.size
    }
}

// --- Synchronous API (Buffered) ---

impl<DI: DisplayInterface> Ssd1351<DI, Buffered> {
    /// SSD1351 §8.9 — Power ON initialization sequence.
    pub fn init(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        ssd1351_init_commands(&mut self.interface, &self.size, config)
    }

    pub fn flush(&mut self) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x15])?;
        self.interface
            .send_data(&[0x00, (self.size.width - 1) as u8])?;
        self.interface.send_command(&[0x75])?;
        self.interface.send_data(&[0x00, self.size.height - 1])?;
        self.interface.send_command(&[0x5C])?;
        let len = self.size.width as usize * self.size.height as usize * 2;
        self.interface.send_data(&self.buffer[..len])
    }

    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.interface.send_command(&[if on { 0xAF } else { 0xAE }])
    }

    pub fn set_contrast_abc(&mut self, a: u8, b: u8, c: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xC1])?;
        self.interface.send_data(&[a, b, c])
    }

    pub fn set_master_contrast(&mut self, current: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xC7])?;
        self.interface.send_data(&[current & 0x0F])
    }

    pub fn set_inverse(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if inverse { 0xA7 } else { 0xA6 }])
    }

    pub fn set_vcomh(&mut self, level: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xBE])?;
        self.interface.send_data(&[level & 0x07])
    }

    pub fn set_precharge_voltage(&mut self, voltage: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xBB])?;
        self.interface.send_data(&[voltage & 0x1F])
    }
}

// --- Synchronous API (Unbuffered) ---

impl<DI: DisplayInterface> Ssd1351<DI, Unbuffered> {
    /// SSD1351 §8.9 — Power ON initialization sequence (unbuffered).
    pub fn init(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        ssd1351_init_commands(&mut self.interface, &self.size, config)
    }

    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.interface.send_command(&[if on { 0xAF } else { 0xAE }])
    }

    pub fn set_contrast_abc(&mut self, a: u8, b: u8, c: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xC1])?;
        self.interface.send_data(&[a, b, c])
    }

    pub fn set_master_contrast(&mut self, current: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xC7])?;
        self.interface.send_data(&[current & 0x0F])
    }

    pub fn set_inverse(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if inverse { 0xA7 } else { 0xA6 }])
    }

    pub fn set_vcomh(&mut self, level: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xBE])?;
        self.interface.send_data(&[level & 0x07])
    }

    pub fn set_precharge_voltage(&mut self, voltage: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xBB])?;
        self.interface.send_data(&[voltage & 0x1F])
    }
}

// --- Async API (Buffered) ---

impl<DI: AsyncDisplayInterface> Ssd1351<DI, Buffered> {
    pub async fn init_async(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        ssd1351_init_commands_async(&mut self.interface, &self.size, config).await
    }

    pub async fn flush_async(&mut self) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x15]).await?;
        self.interface
            .send_data(&[0x00, (self.size.width - 1) as u8])
            .await?;
        self.interface.send_command(&[0x75]).await?;
        self.interface
            .send_data(&[0x00, self.size.height - 1])
            .await?;
        self.interface.send_command(&[0x5C]).await?;
        let len = self.size.width as usize * self.size.height as usize * 2;
        self.interface.send_data(&self.buffer[..len]).await
    }

    pub async fn display_on_async(&mut self, on: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if on { 0xAF } else { 0xAE }])
            .await
    }

    pub async fn set_contrast_abc_async(
        &mut self,
        a: u8,
        b: u8,
        c: u8,
    ) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xC1]).await?;
        self.interface.send_data(&[a, b, c]).await
    }

    pub async fn set_master_contrast_async(&mut self, current: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xC7]).await?;
        self.interface.send_data(&[current & 0x0F]).await
    }

    pub async fn set_inverse_async(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if inverse { 0xA7 } else { 0xA6 }])
            .await
    }
}

// --- Async API (Unbuffered) ---

impl<DI: AsyncDisplayInterface> Ssd1351<DI, Unbuffered> {
    pub async fn init_async(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        ssd1351_init_commands_async(&mut self.interface, &self.size, config).await
    }

    pub async fn display_on_async(&mut self, on: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if on { 0xAF } else { 0xAE }])
            .await
    }

    pub async fn set_contrast_abc_async(
        &mut self,
        a: u8,
        b: u8,
        c: u8,
    ) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xC1]).await?;
        self.interface.send_data(&[a, b, c]).await
    }

    pub async fn set_master_contrast_async(&mut self, current: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xC7]).await?;
        self.interface.send_data(&[current & 0x0F]).await
    }

    pub async fn set_inverse_async(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if inverse { 0xA7 } else { 0xA6 }])
            .await
    }
}

// --- embedded-graphics DrawTarget (both modes for RGB565) ---

impl<DI, MODE: Ssd1351BufferMode> OriginDimensions for Ssd1351<DI, MODE> {
    fn size(&self) -> Size {
        Size::new(self.size.width as u32, self.size.height as u32)
    }
}

impl<DI> DrawTarget for Ssd1351<DI, Buffered> {
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

/// Unbuffered DrawTarget: each pixel is sent as a 1×1 column/row window write directly to GDDRAM.
impl<DI: DisplayInterface> DrawTarget for Ssd1351<DI, Unbuffered> {
    type Color = Rgb565;
    type Error = DisplayError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let w = self.size.width as i32;
        let h = self.size.height as i32;
        for Pixel(coord, color) in pixels {
            if coord.x >= 0 && coord.x < w && coord.y >= 0 && coord.y < h {
                let x = coord.x as u8;
                let y = coord.y as u8;
                // SSD1351 §10.1.1 — Set Column Address (1-pixel window)
                self.interface.send_command(&[0x15])?;
                self.interface.send_data(&[x, x])?;
                // SSD1351 §10.1.2 — Set Row Address (1-pixel window)
                self.interface.send_command(&[0x75])?;
                self.interface.send_data(&[y, y])?;
                // SSD1351 §10.1.3 — Write RAM Command
                self.interface.send_command(&[0x5C])?;
                let raw: u16 = color.into_storage();
                self.interface.send_data(&[(raw >> 8) as u8, raw as u8])?;
            }
        }
        Ok(())
    }
}

fn ssd1351_init_commands(
    interface: &mut impl DisplayInterface,
    size: &DisplaySize,
    config: &InitConfig,
) -> Result<(), DisplayError> {
    interface.send_command(&[0xFD])?;
    interface.send_data(&[0x12])?;
    interface.send_command(&[0xFD])?;
    interface.send_data(&[0xB1])?;
    interface.send_command(&[0xAE])?;
    interface.send_command(&[0xB3])?;
    interface.send_data(&[(config.osc_freq & 0x0F) << 4 | (config.clock_divider & 0x0F)])?;
    interface.send_command(&[0xCA])?;
    interface.send_data(&[size.height - 1])?;
    interface.send_command(&[0xA0])?;
    interface.send_data(&[config.remap_byte()])?;
    interface.send_command(&[0x15])?;
    interface.send_data(&[0x00, (size.width - 1) as u8])?;
    interface.send_command(&[0x75])?;
    interface.send_data(&[0x00, size.height - 1])?;
    interface.send_command(&[0xA1])?;
    interface.send_data(&[config.display_start_line & 0x7F])?;
    interface.send_command(&[0xA2])?;
    interface.send_data(&[config.display_offset & 0x7F])?;
    interface.send_command(&[0xB5])?;
    interface.send_data(&[0x00])?;
    interface.send_command(&[0xAB])?;
    interface.send_data(&[if config.internal_vdd { 0x01 } else { 0x00 }])?;
    interface.send_command(&[0xB1])?;
    interface.send_data(&[(config.phase2 & 0x0F) << 4 | (config.phase1 & 0x0F)])?;
    interface.send_command(&[0xB2])?;
    interface.send_data(&[0xA4, 0x00, 0x00])?;
    interface.send_command(&[0xC1])?;
    interface.send_data(&[config.contrast_a, config.contrast_b, config.contrast_c])?;
    interface.send_command(&[0xC7])?;
    interface.send_data(&[config.master_contrast & 0x0F])?;
    interface.send_command(&[0xB9])?;
    interface.send_command(&[0xBB])?;
    interface.send_data(&[config.precharge_voltage & 0x1F])?;
    interface.send_command(&[0xBE])?;
    interface.send_data(&[config.vcomh & 0x07])?;
    interface.send_command(&[0xB6])?;
    interface.send_data(&[config.second_precharge & 0x0F])?;
    interface.send_command(&[if config.inverse_display { 0xA7 } else { 0xA6 }])?;
    if config.display_on {
        interface.send_command(&[0xAF])?;
    }
    Ok(())
}

async fn ssd1351_init_commands_async(
    interface: &mut impl AsyncDisplayInterface,
    size: &DisplaySize,
    config: &InitConfig,
) -> Result<(), DisplayError> {
    interface.send_command(&[0xFD]).await?;
    interface.send_data(&[0x12]).await?;
    interface.send_command(&[0xFD]).await?;
    interface.send_data(&[0xB1]).await?;
    interface.send_command(&[0xAE]).await?;
    interface.send_command(&[0xB3]).await?;
    interface
        .send_data(&[(config.osc_freq & 0x0F) << 4 | (config.clock_divider & 0x0F)])
        .await?;
    interface.send_command(&[0xCA]).await?;
    interface.send_data(&[size.height - 1]).await?;
    interface.send_command(&[0xA0]).await?;
    interface.send_data(&[config.remap_byte()]).await?;
    interface.send_command(&[0x15]).await?;
    interface.send_data(&[0x00, (size.width - 1) as u8]).await?;
    interface.send_command(&[0x75]).await?;
    interface.send_data(&[0x00, size.height - 1]).await?;
    interface.send_command(&[0xA1]).await?;
    interface
        .send_data(&[config.display_start_line & 0x7F])
        .await?;
    interface.send_command(&[0xA2]).await?;
    interface.send_data(&[config.display_offset & 0x7F]).await?;
    interface.send_command(&[0xB5]).await?;
    interface.send_data(&[0x00]).await?;
    interface.send_command(&[0xAB]).await?;
    interface
        .send_data(&[if config.internal_vdd { 0x01 } else { 0x00 }])
        .await?;
    interface.send_command(&[0xB1]).await?;
    interface
        .send_data(&[(config.phase2 & 0x0F) << 4 | (config.phase1 & 0x0F)])
        .await?;
    interface.send_command(&[0xB2]).await?;
    interface.send_data(&[0xA4, 0x00, 0x00]).await?;
    interface.send_command(&[0xC1]).await?;
    interface
        .send_data(&[config.contrast_a, config.contrast_b, config.contrast_c])
        .await?;
    interface.send_command(&[0xC7]).await?;
    interface
        .send_data(&[config.master_contrast & 0x0F])
        .await?;
    interface.send_command(&[0xB9]).await?;
    interface.send_command(&[0xBB]).await?;
    interface
        .send_data(&[config.precharge_voltage & 0x1F])
        .await?;
    interface.send_command(&[0xBE]).await?;
    interface.send_data(&[config.vcomh & 0x07]).await?;
    interface.send_command(&[0xB6]).await?;
    interface
        .send_data(&[config.second_precharge & 0x0F])
        .await?;
    interface
        .send_command(&[if config.inverse_display { 0xA7 } else { 0xA6 }])
        .await?;
    if config.display_on {
        interface.send_command(&[0xAF]).await?;
    }
    Ok(())
}
