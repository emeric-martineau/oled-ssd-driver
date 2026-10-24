use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::BinaryColor,
    Pixel,
};

use super::init::{InitConfig, ScrollInterval0303, DEFAULT_DISPLAY_SIZE};
use crate::common::{Buffered, DisplayBufferMode, DisplayCore, DisplaySize, Unbuffered};
use crate::error::DisplayError;
use crate::interface::{AsyncDisplayInterface, DisplayInterface};

/// SSD0303 OLED display driver.
///
/// Generic over the bus interface `DI` (I2C or SPI) and buffer mode `MODE`.
/// In `Buffered` mode (default) the driver owns a framebuffer and supports all drawing APIs.
/// In `Unbuffered` mode the driver owns no RAM buffer; only control commands are available.
pub struct Ssd0303<DI, MODE: DisplayBufferMode = Buffered> {
    core: DisplayCore<DI, MODE>,
}

impl<DI> Ssd0303<DI, Buffered> {
    /// Create a new SSD0303 driver with default 132x64 display size (buffered).
    pub fn new(interface: DI) -> Self {
        Self {
            core: DisplayCore::new(interface, DEFAULT_DISPLAY_SIZE),
        }
    }

    /// Create a new SSD0303 driver with a custom display size (buffered).
    pub fn new_with_size(interface: DI, size: DisplaySize) -> Self {
        Self {
            core: DisplayCore::new(interface, size),
        }
    }

    pub fn release(self) -> DI {
        self.core.interface
    }

    /// SSD0303 §7.9 — Each byte represents 8 vertical pixels (D0=top, D7=bottom) within one page.
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

impl<DI> Ssd0303<DI, Unbuffered> {
    /// Create a new SSD0303 driver in unbuffered mode with default 132x64 display size.
    pub fn new_unbuffered(interface: DI) -> Self {
        Self {
            core: DisplayCore::new(interface, DEFAULT_DISPLAY_SIZE),
        }
    }

    /// Create a new SSD0303 driver in unbuffered mode with a custom display size.
    pub fn new_unbuffered_with_size(interface: DI, size: DisplaySize) -> Self {
        Self {
            core: DisplayCore::new(interface, size),
        }
    }

    pub fn release(self) -> DI {
        self.core.interface
    }

    pub fn size_config(&self) -> DisplaySize {
        self.core.size
    }
}

// --- Synchronous API (Buffered) ---

impl<DI: DisplayInterface> Ssd0303<DI, Buffered> {
    fn send_raw_command(&mut self, bytes: &[u8]) -> Result<(), DisplayError> {
        self.core.interface.send_command(bytes)
    }

    /// SSD0303 §7.2 — Power ON initialization sequence.
    pub fn init(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        ssd0303_init_commands(&mut self.core.interface, config)
    }

    /// SSD0303 — Flush the internal framebuffer to the display GDDRAM (page mode).
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        let pages = self.core.size.pages();
        let width = self.core.size.width;
        for page in 0..pages {
            self.send_raw_command(&[0xB0 | page])?;
            self.send_raw_command(&[0x00])?;
            self.send_raw_command(&[0x10])?;
            let start = page as usize * width as usize;
            let end = start + width as usize;
            self.core
                .interface
                .send_data(&self.core.buffer[start..end])?;
        }
        Ok(())
    }

    /// SSD0303 §8 — Turn the display panel ON or OFF (sleep mode)
    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.send_raw_command(&[if on { 0xAF } else { 0xAE }])
    }

    /// SSD0303 §8 — Set contrast (0-255)
    pub fn set_contrast(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.send_raw_command(&[0x81, contrast])
    }

    /// SSD0303 §8 — Set normal or inverse display
    pub fn set_inverse(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.send_raw_command(&[if inverse { 0xA7 } else { 0xA6 }])
    }

    /// SSD0303 §8 — Set VCOM deselect level (7-bit value, 0-127)
    pub fn set_vcomh(&mut self, level: u8) -> Result<(), DisplayError> {
        self.send_raw_command(&[0xDB, level & 0x7F])
    }

    /// SSD0303 §8 — Set DC-DC on/off
    pub fn set_dc_dc(&mut self, enable: bool) -> Result<(), DisplayError> {
        self.send_raw_command(&[0xAD, if enable { 0x01 } else { 0x00 }])
    }

    /// SSD0303 §8 — Set brightness for color banks (82h)
    pub fn set_brightness(&mut self, brightness: u8) -> Result<(), DisplayError> {
        self.send_raw_command(&[0x82, brightness])
    }

    /// SSD0303 §8 — Configure and activate horizontal scrolling
    pub fn start_horizontal_scroll(
        &mut self,
        start_page: u8,
        end_page: u8,
        num_columns: u8,
        interval: ScrollInterval0303,
    ) -> Result<(), DisplayError> {
        let max_page = self.core.size.pages() - 1;
        if start_page > max_page || end_page > max_page || end_page < start_page {
            return Err(DisplayError::InvalidArgument);
        }
        if num_columns == 0 || num_columns > 4 {
            return Err(DisplayError::InvalidArgument);
        }
        self.send_raw_command(&[0x2E])?;
        self.send_raw_command(&[
            0x26,
            num_columns & 0x07,
            start_page & 0x07,
            interval as u8,
            end_page & 0x07,
        ])?;
        self.send_raw_command(&[0x2F])
    }

    /// SSD0303 §8 — Stop scrolling (2Eh)
    pub fn stop_scroll(&mut self) -> Result<(), DisplayError> {
        self.send_raw_command(&[0x2E])
    }
}

// --- Synchronous API (Unbuffered) ---

impl<DI: DisplayInterface> Ssd0303<DI, Unbuffered> {
    fn send_raw_command(&mut self, bytes: &[u8]) -> Result<(), DisplayError> {
        self.core.interface.send_command(bytes)
    }

    /// SSD0303 §7.2 — Power ON initialization sequence (unbuffered).
    pub fn init(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        ssd0303_init_commands(&mut self.core.interface, config)
    }

    /// SSD0303 §8 — Turn the display panel ON or OFF (sleep mode)
    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.send_raw_command(&[if on { 0xAF } else { 0xAE }])
    }

    /// SSD0303 §8 — Set contrast (0-255)
    pub fn set_contrast(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.send_raw_command(&[0x81, contrast])
    }

    /// SSD0303 §8 — Set normal or inverse display
    pub fn set_inverse(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.send_raw_command(&[if inverse { 0xA7 } else { 0xA6 }])
    }

    /// SSD0303 §8 — Set VCOM deselect level (7-bit value, 0-127)
    pub fn set_vcomh(&mut self, level: u8) -> Result<(), DisplayError> {
        self.send_raw_command(&[0xDB, level & 0x7F])
    }

    /// SSD0303 §8 — Set DC-DC on/off
    pub fn set_dc_dc(&mut self, enable: bool) -> Result<(), DisplayError> {
        self.send_raw_command(&[0xAD, if enable { 0x01 } else { 0x00 }])
    }

    /// SSD0303 §8 — Set brightness for color banks (82h)
    pub fn set_brightness(&mut self, brightness: u8) -> Result<(), DisplayError> {
        self.send_raw_command(&[0x82, brightness])
    }

    /// SSD0303 §8 — Configure and activate horizontal scrolling
    pub fn start_horizontal_scroll(
        &mut self,
        start_page: u8,
        end_page: u8,
        num_columns: u8,
        interval: ScrollInterval0303,
    ) -> Result<(), DisplayError> {
        let max_page = self.core.size.pages() - 1;
        if start_page > max_page || end_page > max_page || end_page < start_page {
            return Err(DisplayError::InvalidArgument);
        }
        if num_columns == 0 || num_columns > 4 {
            return Err(DisplayError::InvalidArgument);
        }
        self.send_raw_command(&[0x2E])?;
        self.send_raw_command(&[
            0x26,
            num_columns & 0x07,
            start_page & 0x07,
            interval as u8,
            end_page & 0x07,
        ])?;
        self.send_raw_command(&[0x2F])
    }

    /// SSD0303 §8 — Stop scrolling (2Eh)
    pub fn stop_scroll(&mut self) -> Result<(), DisplayError> {
        self.send_raw_command(&[0x2E])
    }
}

// --- Async API (Buffered) ---

impl<DI: AsyncDisplayInterface> Ssd0303<DI, Buffered> {
    async fn send_raw_command_async(&mut self, bytes: &[u8]) -> Result<(), DisplayError> {
        self.core.interface.send_command(bytes).await
    }

    /// SSD0303 §7.2 — Async power ON initialization sequence.
    pub async fn init_async(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        ssd0303_init_commands_async(&mut self.core.interface, config).await
    }

    /// SSD0303 — Async flush of framebuffer to display GDDRAM (page mode).
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

    /// SSD0303 §8 — Async display ON/OFF
    pub async fn display_on_async(&mut self, on: bool) -> Result<(), DisplayError> {
        self.send_raw_command_async(&[if on { 0xAF } else { 0xAE }])
            .await
    }

    /// SSD0303 §8 — Async set contrast
    pub async fn set_contrast_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.send_raw_command_async(&[0x81, contrast]).await
    }

    /// SSD0303 §8 — Async set inverse
    pub async fn set_inverse_async(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.send_raw_command_async(&[if inverse { 0xA7 } else { 0xA6 }])
            .await
    }
}

// --- Async API (Unbuffered) ---

impl<DI: AsyncDisplayInterface> Ssd0303<DI, Unbuffered> {
    async fn send_raw_command_async(&mut self, bytes: &[u8]) -> Result<(), DisplayError> {
        self.core.interface.send_command(bytes).await
    }

    /// SSD0303 §7.2 — Async power ON initialization sequence (unbuffered).
    pub async fn init_async(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        ssd0303_init_commands_async(&mut self.core.interface, config).await
    }

    /// SSD0303 §8 — Async display ON/OFF
    pub async fn display_on_async(&mut self, on: bool) -> Result<(), DisplayError> {
        self.send_raw_command_async(&[if on { 0xAF } else { 0xAE }])
            .await
    }

    /// SSD0303 §8 — Async set contrast
    pub async fn set_contrast_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.send_raw_command_async(&[0x81, contrast]).await
    }

    /// SSD0303 §8 — Async set inverse
    pub async fn set_inverse_async(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.send_raw_command_async(&[if inverse { 0xA7 } else { 0xA6 }])
            .await
    }
}

// --- embedded-graphics DrawTarget (Buffered only) ---

impl<DI, MODE: DisplayBufferMode> OriginDimensions for Ssd0303<DI, MODE> {
    fn size(&self) -> Size {
        Size::new(self.core.size.width as u32, self.core.size.height as u32)
    }
}

impl<DI> DrawTarget for Ssd0303<DI, Buffered> {
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

fn ssd0303_init_commands(
    interface: &mut impl DisplayInterface,
    config: &InitConfig,
) -> Result<(), DisplayError> {
    interface.send_command(&[0xAE])?;
    interface.send_command(&[
        0xD5,
        (config.osc_freq & 0x0F) << 4 | (config.clock_divide_ratio & 0x0F),
    ])?;
    interface.send_command(&[0xA8, (config.multiplex_ratio - 1) & 0x3F])?;
    interface.send_command(&[0xD3, config.display_offset & 0x3F])?;
    interface.send_command(&[0x40 | (config.display_start_line & 0x3F)])?;
    interface.send_command(&[0xAD, if config.dc_dc_enable { 0x01 } else { 0x00 }])?;
    interface.send_command(&[if config.segment_remap { 0xA1 } else { 0xA0 }])?;
    interface.send_command(&[if config.com_scan_reversed { 0xC8 } else { 0xC0 }])?;
    interface.send_command(&[
        0xDA,
        if config.com_pin_alternative {
            0x12
        } else {
            0x02
        },
    ])?;
    interface.send_command(&[0x81, config.contrast])?;
    interface.send_command(&[0xD9, (config.phase2 & 0x0F) << 4 | (config.phase1 & 0x0F)])?;
    interface.send_command(&[0xDB, config.vcomh & 0x7F])?;
    interface.send_command(&[if config.entire_display_on { 0xA5 } else { 0xA4 }])?;
    interface.send_command(&[if config.inverse_display { 0xA7 } else { 0xA6 }])?;
    if config.deactivate_scroll {
        interface.send_command(&[0x2E])?;
    }
    if config.display_on {
        interface.send_command(&[0xAF])?;
    }
    Ok(())
}

async fn ssd0303_init_commands_async(
    interface: &mut impl AsyncDisplayInterface,
    config: &InitConfig,
) -> Result<(), DisplayError> {
    interface.send_command(&[0xAE]).await?;
    interface
        .send_command(&[
            0xD5,
            (config.osc_freq & 0x0F) << 4 | (config.clock_divide_ratio & 0x0F),
        ])
        .await?;
    interface
        .send_command(&[0xA8, (config.multiplex_ratio - 1) & 0x3F])
        .await?;
    interface
        .send_command(&[0xD3, config.display_offset & 0x3F])
        .await?;
    interface
        .send_command(&[0x40 | (config.display_start_line & 0x3F)])
        .await?;
    interface
        .send_command(&[0xAD, if config.dc_dc_enable { 0x01 } else { 0x00 }])
        .await?;
    interface
        .send_command(&[if config.segment_remap { 0xA1 } else { 0xA0 }])
        .await?;
    interface
        .send_command(&[if config.com_scan_reversed { 0xC8 } else { 0xC0 }])
        .await?;
    interface
        .send_command(&[
            0xDA,
            if config.com_pin_alternative {
                0x12
            } else {
                0x02
            },
        ])
        .await?;
    interface.send_command(&[0x81, config.contrast]).await?;
    interface
        .send_command(&[0xD9, (config.phase2 & 0x0F) << 4 | (config.phase1 & 0x0F)])
        .await?;
    interface.send_command(&[0xDB, config.vcomh & 0x7F]).await?;
    interface
        .send_command(&[if config.entire_display_on { 0xA5 } else { 0xA4 }])
        .await?;
    interface
        .send_command(&[if config.inverse_display { 0xA7 } else { 0xA6 }])
        .await?;
    if config.deactivate_scroll {
        interface.send_command(&[0x2E]).await?;
    }
    if config.display_on {
        interface.send_command(&[0xAF]).await?;
    }
    Ok(())
}
