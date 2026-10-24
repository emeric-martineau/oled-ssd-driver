use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::{IntoStorage, Rgb565},
    Pixel,
};

use super::init::{InitConfig, SIZE_96X64};
use crate::common::DisplaySize;
use crate::common::{Buffered, Unbuffered};
use crate::error::DisplayError;
use crate::interface::{AsyncDisplayInterface, DisplayInterface};

/// Maximum buffer: 96 × 64 × 2 bytes (65K color, 16 bits per pixel) = 12288 bytes
const BUFFER_SIZE_1332: usize = 96 * 64 * 2;

pub trait Ssd1332BufferMode {
    type Buffer;
    fn new_buffer() -> Self::Buffer;
}

impl Ssd1332BufferMode for Buffered {
    type Buffer = [u8; BUFFER_SIZE_1332];
    fn new_buffer() -> Self::Buffer {
        [0u8; BUFFER_SIZE_1332]
    }
}

impl Ssd1332BufferMode for Unbuffered {
    type Buffer = ();
    fn new_buffer() -> Self::Buffer {}
}

/// SSD1332 OLED display driver.
///
/// Generic over the bus interface `DI` (SPI only) and buffer mode `MODE`.
/// In `Buffered` mode (default) the driver owns a framebuffer and supports all drawing APIs.
/// In `Unbuffered` mode the driver owns no RAM buffer; pixels are sent directly to GDDRAM
/// using per-pixel column/row window commands (slower, but ~12 KB RAM saving).
pub struct Ssd1332<DI, MODE: Ssd1332BufferMode = Buffered> {
    interface: DI,
    buffer: MODE::Buffer,
    size: DisplaySize,
}

impl<DI> Ssd1332<DI, Buffered> {
    /// Create a new SSD1332 driver with default 96x64 display size (buffered).
    pub fn new(interface: DI) -> Self {
        Self {
            interface,
            buffer: [0u8; BUFFER_SIZE_1332],
            size: SIZE_96X64,
        }
    }

    pub fn new_with_size(interface: DI, size: DisplaySize) -> Self {
        Self {
            interface,
            buffer: [0u8; BUFFER_SIZE_1332],
            size,
        }
    }

    pub fn release(self) -> DI {
        self.interface
    }

    /// SSD1332 §7 — Set a pixel in the internal framebuffer (big-endian RGB565).
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

impl<DI> Ssd1332<DI, Unbuffered> {
    /// Create a new SSD1332 driver in unbuffered mode with default 96x64 display size.
    pub fn new_unbuffered(interface: DI) -> Self {
        Self {
            interface,
            buffer: (),
            size: SIZE_96X64,
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

impl<DI: DisplayInterface> Ssd1332<DI, Buffered> {
    pub fn init(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        ssd1332_init_commands(&mut self.interface, &self.size, config)
    }

    pub fn flush(&mut self) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[0x15, 0x00, (self.size.width - 1) as u8])?;
        self.interface
            .send_command(&[0x75, 0x00, self.size.height - 1])?;
        let len = self.size.width as usize * self.size.height as usize * 2;
        self.interface.send_data(&self.buffer[..len])
    }

    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.interface.send_command(&[if on { 0xAF } else { 0xAE }])
    }

    pub fn set_contrast_a(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x81, contrast])
    }

    pub fn set_contrast_b(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x82, contrast])
    }

    pub fn set_contrast_c(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x83, contrast])
    }

    pub fn set_master_current(&mut self, current: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x87, current & 0x0F])
    }

    pub fn set_inverse(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if inverse { 0xA7 } else { 0xA4 }])
    }

    pub fn set_vcomh(&mut self, level: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xBE, level & 0x3F])
    }

    pub fn set_vpa(&mut self, voltage: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xBB, voltage])
    }

    pub fn set_vpb(&mut self, voltage: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xBC, voltage])
    }

    pub fn set_vpc(&mut self, voltage: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xBD, voltage])
    }

    pub fn set_power_save(&mut self, enabled: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[0xB0, if enabled { 0x12 } else { 0x00 }])
    }
}

// --- Synchronous API (Unbuffered) ---

impl<DI: DisplayInterface> Ssd1332<DI, Unbuffered> {
    pub fn init(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        ssd1332_init_commands(&mut self.interface, &self.size, config)
    }

    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.interface.send_command(&[if on { 0xAF } else { 0xAE }])
    }

    pub fn set_contrast_a(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x81, contrast])
    }

    pub fn set_contrast_b(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x82, contrast])
    }

    pub fn set_contrast_c(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x83, contrast])
    }

    pub fn set_master_current(&mut self, current: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x87, current & 0x0F])
    }

    pub fn set_inverse(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if inverse { 0xA7 } else { 0xA4 }])
    }

    pub fn set_vcomh(&mut self, level: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0xBE, level & 0x3F])
    }

    pub fn set_power_save(&mut self, enabled: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[0xB0, if enabled { 0x12 } else { 0x00 }])
    }
}

// --- Async API (Buffered) ---

impl<DI: AsyncDisplayInterface> Ssd1332<DI, Buffered> {
    pub async fn init_async(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        ssd1332_init_commands_async(&mut self.interface, &self.size, config).await
    }

    pub async fn flush_async(&mut self) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[0x15, 0x00, (self.size.width - 1) as u8])
            .await?;
        self.interface
            .send_command(&[0x75, 0x00, self.size.height - 1])
            .await?;
        let len = self.size.width as usize * self.size.height as usize * 2;
        self.interface.send_data(&self.buffer[..len]).await
    }

    pub async fn display_on_async(&mut self, on: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if on { 0xAF } else { 0xAE }])
            .await
    }

    pub async fn set_contrast_a_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x81, contrast]).await
    }

    pub async fn set_contrast_b_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x82, contrast]).await
    }

    pub async fn set_contrast_c_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x83, contrast]).await
    }

    pub async fn set_inverse_async(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if inverse { 0xA7 } else { 0xA4 }])
            .await
    }
}

// --- Async API (Unbuffered) ---

impl<DI: AsyncDisplayInterface> Ssd1332<DI, Unbuffered> {
    pub async fn init_async(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        ssd1332_init_commands_async(&mut self.interface, &self.size, config).await
    }

    pub async fn display_on_async(&mut self, on: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if on { 0xAF } else { 0xAE }])
            .await
    }

    pub async fn set_contrast_a_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x81, contrast]).await
    }

    pub async fn set_contrast_b_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x82, contrast]).await
    }

    pub async fn set_contrast_c_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.interface.send_command(&[0x83, contrast]).await
    }

    pub async fn set_inverse_async(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(&[if inverse { 0xA7 } else { 0xA4 }])
            .await
    }
}

// --- embedded-graphics DrawTarget (both modes for RGB565) ---

impl<DI, MODE: Ssd1332BufferMode> OriginDimensions for Ssd1332<DI, MODE> {
    fn size(&self) -> Size {
        Size::new(self.size.width as u32, self.size.height as u32)
    }
}

impl<DI> DrawTarget for Ssd1332<DI, Buffered> {
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

/// Unbuffered DrawTarget: each pixel is sent as a 1×1 window write directly to GDDRAM.
impl<DI: DisplayInterface> DrawTarget for Ssd1332<DI, Unbuffered> {
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
                // SSD1332 §9 — Set Column Address (1-pixel window)
                self.interface.send_command(&[0x15, x, x])?;
                // SSD1332 §9 — Set Row Address (1-pixel window)
                self.interface.send_command(&[0x75, y, y])?;
                let raw: u16 = color.into_storage();
                self.interface.send_data(&[(raw >> 8) as u8, raw as u8])?;
            }
        }
        Ok(())
    }
}

fn ssd1332_init_commands(
    interface: &mut impl DisplayInterface,
    size: &DisplaySize,
    config: &InitConfig,
) -> Result<(), DisplayError> {
    interface.send_command(&[0xAE])?;
    interface.send_command(&[0xA0, config.remap_byte()])?;
    interface.send_command(&[0xA1, config.display_start_line & 0x3F])?;
    interface.send_command(&[0xA2, config.display_offset & 0x3F])?;
    interface.send_command(&[if config.inverse_display { 0xA7 } else { 0xA4 }])?;
    interface.send_command(&[0xA8, size.height - 1])?;
    interface.send_command(&[0xAD, config.master_config_byte()])?;
    interface.send_command(&[0xB0, 0x00])?;
    interface.send_command(&[0xB1, (config.phase2 & 0x0F) << 4 | (config.phase1 & 0x0F)])?;
    interface.send_command(&[
        0xB3,
        (config.osc_freq & 0x0F) << 4 | (config.clock_divide_ratio & 0x0F),
    ])?;
    interface.send_command(&[0x81, config.contrast_a])?;
    interface.send_command(&[0x82, config.contrast_b])?;
    interface.send_command(&[0x83, config.contrast_c])?;
    interface.send_command(&[0x87, config.master_current & 0x0F])?;
    interface.send_command(&[0xB9])?;
    interface.send_command(&[0xBB, config.vpa])?;
    interface.send_command(&[0xBC, config.vpb])?;
    interface.send_command(&[0xBD, config.vpc])?;
    interface.send_command(&[0xBE, config.vcomh & 0x3F])?;
    if config.display_on {
        interface.send_command(&[0xAF])?;
    }
    Ok(())
}

async fn ssd1332_init_commands_async(
    interface: &mut impl AsyncDisplayInterface,
    size: &DisplaySize,
    config: &InitConfig,
) -> Result<(), DisplayError> {
    interface.send_command(&[0xAE]).await?;
    interface.send_command(&[0xA0, config.remap_byte()]).await?;
    interface
        .send_command(&[0xA1, config.display_start_line & 0x3F])
        .await?;
    interface
        .send_command(&[0xA2, config.display_offset & 0x3F])
        .await?;
    interface
        .send_command(&[if config.inverse_display { 0xA7 } else { 0xA4 }])
        .await?;
    interface.send_command(&[0xA8, size.height - 1]).await?;
    interface
        .send_command(&[0xAD, config.master_config_byte()])
        .await?;
    interface.send_command(&[0xB0, 0x00]).await?;
    interface
        .send_command(&[0xB1, (config.phase2 & 0x0F) << 4 | (config.phase1 & 0x0F)])
        .await?;
    interface
        .send_command(&[
            0xB3,
            (config.osc_freq & 0x0F) << 4 | (config.clock_divide_ratio & 0x0F),
        ])
        .await?;
    interface.send_command(&[0x81, config.contrast_a]).await?;
    interface.send_command(&[0x82, config.contrast_b]).await?;
    interface.send_command(&[0x83, config.contrast_c]).await?;
    interface
        .send_command(&[0x87, config.master_current & 0x0F])
        .await?;
    interface.send_command(&[0xB9]).await?;
    interface.send_command(&[0xBB, config.vpa]).await?;
    interface.send_command(&[0xBC, config.vpb]).await?;
    interface.send_command(&[0xBD, config.vpc]).await?;
    interface.send_command(&[0xBE, config.vcomh & 0x3F]).await?;
    if config.display_on {
        interface.send_command(&[0xAF]).await?;
    }
    Ok(())
}
