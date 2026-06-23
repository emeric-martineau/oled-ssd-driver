use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::BinaryColor,
    Pixel,
};

use super::command::{ChargePumpVoltage1312, IrefSetting1312, ScrollInterval1312, VcomhLevel1312};
use crate::common::{
    Buffered, DisplayBufferMode, DisplayCore, DisplaySize, InitConfig, Unbuffered, SIZE_128X64,
};
use crate::error::DisplayError;
use crate::interface::{AsyncDisplayInterface, DisplayInterface};
use crate::ssd1306::command::{Command, ScrollDirection};

/// SSD1312 OLED display driver.
///
/// Generic over the bus interface `DI` (I2C or SPI) and buffer mode `MODE`.
/// In `Buffered` mode (default) the driver owns a framebuffer and supports all drawing APIs.
/// In `Unbuffered` mode the driver owns no RAM buffer; only control commands are available.
pub struct Ssd1312<DI, MODE: DisplayBufferMode = Buffered> {
    core: DisplayCore<DI, MODE>,
}

impl<DI> Ssd1312<DI, Buffered> {
    /// Create a new SSD1312 driver with default 128x64 display size (buffered).
    pub fn new(interface: DI) -> Self {
        Self {
            core: DisplayCore::new(interface, SIZE_128X64),
        }
    }

    /// Create a new SSD1312 driver with a custom display size (buffered).
    pub fn new_with_size(interface: DI, size: DisplaySize) -> Self {
        Self {
            core: DisplayCore::new(interface, size),
        }
    }

    pub fn release(self) -> DI {
        self.core.interface
    }

    /// SSD1312 §6.7 — Each byte represents 8 vertical pixels (D0=top, D7=bottom) within one page.
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

impl<DI> Ssd1312<DI, Unbuffered> {
    /// Create a new SSD1312 driver in unbuffered mode with default 128x64 display size.
    pub fn new_unbuffered(interface: DI) -> Self {
        Self {
            core: DisplayCore::new(interface, SIZE_128X64),
        }
    }

    /// Create a new SSD1312 driver in unbuffered mode with a custom display size.
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

impl<DI: DisplayInterface> Ssd1312<DI, Buffered> {
    fn send_command(&mut self, cmd: Command) -> Result<(), DisplayError> {
        self.core.send_command(cmd)
    }

    fn send_raw_command(&mut self, bytes: &[u8]) -> Result<(), DisplayError> {
        self.core.interface.send_command(bytes)
    }

    /// SSD1312 §6.9.2 — Power ON initialization sequence with charge pump.
    pub fn init(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        self.core.init_common(config)?;
        self.send_raw_command(&[0xAD, 0x10])?;
        self.send_raw_command(&[0x8D, ChargePumpVoltage1312::V7_5 as u8])?;
        if config.display_on {
            self.send_command(Command::DisplayOn(true))?;
        }
        Ok(())
    }

    /// Flush the internal framebuffer to the display GDDRAM.
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        self.core.flush()
    }

    /// SSD1312 §6.9 — Turn the display panel ON or OFF (sleep mode)
    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.core.display_on(on)
    }

    /// SSD1312 §6.6 — Set contrast (0-255)
    pub fn set_contrast(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.core.set_contrast(contrast)
    }

    /// SSD1312 — Invert the display (swap ON/OFF pixels)
    pub fn set_inverse(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.core.set_inverse(inverse)
    }

    /// SSD1312 §6.10 — Set charge pump output voltage
    pub fn set_charge_pump_voltage(
        &mut self,
        voltage: ChargePumpVoltage1312,
    ) -> Result<(), DisplayError> {
        self.send_raw_command(&[0x8D, voltage as u8])
    }

    /// SSD1312 §6.8 — Set IREF selection
    pub fn set_iref(&mut self, setting: IrefSetting1312) -> Result<(), DisplayError> {
        let byte = match setting {
            IrefSetting1312::External => 0x00,
            IrefSetting1312::Internal => 0x10,
        };
        self.send_raw_command(&[0xAD, byte])
    }

    /// SSD1312 §6.8 — Set VCOMH deselect level (4 levels)
    pub fn set_vcomh_deselect(&mut self, level: VcomhLevel1312) -> Result<(), DisplayError> {
        self.send_raw_command(&[0xDB, level as u8])
    }

    /// SSD1312 — Configure and activate horizontal scrolling with column bounds
    pub fn start_horizontal_scroll(
        &mut self,
        direction: ScrollDirection,
        start_page: u8,
        end_page: u8,
        interval: ScrollInterval1312,
        start_column: u8,
        end_column: u8,
    ) -> Result<(), DisplayError> {
        let max_page = self.core.size.pages() - 1;
        let max_col = self.core.size.width - 1;
        if start_page > max_page || end_page > max_page || end_page < start_page {
            return Err(DisplayError::InvalidArgument);
        }
        if start_column > max_col || end_column > max_col || end_column < start_column {
            return Err(DisplayError::InvalidArgument);
        }
        self.core.stop_scroll()?;
        let cmd_byte = match direction {
            ScrollDirection::Right => 0x26,
            ScrollDirection::Left => 0x27,
        };
        self.send_raw_command(&[
            cmd_byte,
            0x00,
            start_page,
            interval as u8,
            end_page,
            start_column,
            end_column,
        ])?;
        self.send_raw_command(&[0x2F])
    }

    /// SSD1312 — Content scroll with column address bounds
    pub fn start_content_scroll(
        &mut self,
        direction: ScrollDirection,
        start_page: u8,
        end_page: u8,
        interval: ScrollInterval1312,
        start_column: u8,
        end_column: u8,
    ) -> Result<(), DisplayError> {
        let max_page = self.core.size.pages() - 1;
        let max_col = self.core.size.width - 1;
        if start_page > max_page || end_page > max_page || end_page < start_page {
            return Err(DisplayError::InvalidArgument);
        }
        if start_column > max_col || end_column > max_col || end_column < start_column {
            return Err(DisplayError::InvalidArgument);
        }
        self.core.stop_scroll()?;
        let cmd_byte = match direction {
            ScrollDirection::Right => 0x2C,
            ScrollDirection::Left => 0x2D,
        };
        self.send_raw_command(&[
            cmd_byte,
            0x00,
            start_page,
            interval as u8,
            end_page,
            start_column,
            end_column,
        ])?;
        self.send_raw_command(&[0x2F])
    }

    /// SSD1312 — Stop scrolling
    pub fn stop_scroll(&mut self) -> Result<(), DisplayError> {
        self.core.stop_scroll()
    }
}

// --- Synchronous API (Unbuffered) ---

impl<DI: DisplayInterface> Ssd1312<DI, Unbuffered> {
    fn send_raw_command(&mut self, bytes: &[u8]) -> Result<(), DisplayError> {
        self.core.interface.send_command(bytes)
    }

    /// SSD1312 §6.9.2 — Power ON initialization sequence (unbuffered).
    pub fn init(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        self.core.init_common(config)?;
        self.send_raw_command(&[0xAD, 0x10])?;
        self.send_raw_command(&[0x8D, ChargePumpVoltage1312::V7_5 as u8])?;
        if config.display_on {
            self.core.send_command(Command::DisplayOn(true))?;
        }
        Ok(())
    }

    /// SSD1312 §6.9 — Turn the display panel ON or OFF (sleep mode)
    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.core.display_on(on)
    }

    /// SSD1312 §6.6 — Set contrast (0-255)
    pub fn set_contrast(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.core.set_contrast(contrast)
    }

    /// SSD1312 — Invert the display (swap ON/OFF pixels)
    pub fn set_inverse(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.core.set_inverse(inverse)
    }

    /// SSD1312 §6.10 — Set charge pump output voltage
    pub fn set_charge_pump_voltage(
        &mut self,
        voltage: ChargePumpVoltage1312,
    ) -> Result<(), DisplayError> {
        self.send_raw_command(&[0x8D, voltage as u8])
    }

    /// SSD1312 §6.8 — Set IREF selection
    pub fn set_iref(&mut self, setting: IrefSetting1312) -> Result<(), DisplayError> {
        let byte = match setting {
            IrefSetting1312::External => 0x00,
            IrefSetting1312::Internal => 0x10,
        };
        self.send_raw_command(&[0xAD, byte])
    }

    /// SSD1312 §6.8 — Set VCOMH deselect level
    pub fn set_vcomh_deselect(&mut self, level: VcomhLevel1312) -> Result<(), DisplayError> {
        self.send_raw_command(&[0xDB, level as u8])
    }

    /// SSD1312 — Configure and activate horizontal scrolling
    pub fn start_horizontal_scroll(
        &mut self,
        direction: ScrollDirection,
        start_page: u8,
        end_page: u8,
        interval: ScrollInterval1312,
        start_column: u8,
        end_column: u8,
    ) -> Result<(), DisplayError> {
        let max_page = self.core.size.pages() - 1;
        let max_col = self.core.size.width - 1;
        if start_page > max_page || end_page > max_page || end_page < start_page {
            return Err(DisplayError::InvalidArgument);
        }
        if start_column > max_col || end_column > max_col || end_column < start_column {
            return Err(DisplayError::InvalidArgument);
        }
        self.core.stop_scroll()?;
        let cmd_byte = match direction {
            ScrollDirection::Right => 0x26,
            ScrollDirection::Left => 0x27,
        };
        self.send_raw_command(&[
            cmd_byte,
            0x00,
            start_page,
            interval as u8,
            end_page,
            start_column,
            end_column,
        ])?;
        self.send_raw_command(&[0x2F])
    }

    /// SSD1312 — Content scroll with column address bounds
    pub fn start_content_scroll(
        &mut self,
        direction: ScrollDirection,
        start_page: u8,
        end_page: u8,
        interval: ScrollInterval1312,
        start_column: u8,
        end_column: u8,
    ) -> Result<(), DisplayError> {
        let max_page = self.core.size.pages() - 1;
        let max_col = self.core.size.width - 1;
        if start_page > max_page || end_page > max_page || end_page < start_page {
            return Err(DisplayError::InvalidArgument);
        }
        if start_column > max_col || end_column > max_col || end_column < start_column {
            return Err(DisplayError::InvalidArgument);
        }
        self.core.stop_scroll()?;
        let cmd_byte = match direction {
            ScrollDirection::Right => 0x2C,
            ScrollDirection::Left => 0x2D,
        };
        self.send_raw_command(&[
            cmd_byte,
            0x00,
            start_page,
            interval as u8,
            end_page,
            start_column,
            end_column,
        ])?;
        self.send_raw_command(&[0x2F])
    }

    /// SSD1312 — Stop scrolling
    pub fn stop_scroll(&mut self) -> Result<(), DisplayError> {
        self.core.stop_scroll()
    }
}

// --- Async API (Buffered) ---

impl<DI: AsyncDisplayInterface> Ssd1312<DI, Buffered> {
    async fn send_raw_command_async(&mut self, bytes: &[u8]) -> Result<(), DisplayError> {
        self.core.interface.send_command(bytes).await
    }

    /// SSD1312 §6.9.2 — Async power ON initialization sequence.
    pub async fn init_async(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        self.core.init_common_async(config).await?;
        self.send_raw_command_async(&[0xAD, 0x10]).await?;
        self.send_raw_command_async(&[0x8D, ChargePumpVoltage1312::V7_5 as u8])
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

    /// SSD1312 §6.9 — Async display ON/OFF
    pub async fn display_on_async(&mut self, on: bool) -> Result<(), DisplayError> {
        self.core.display_on_async(on).await
    }

    /// SSD1312 §6.6 — Async set contrast
    pub async fn set_contrast_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.core.set_contrast_async(contrast).await
    }

    /// SSD1312 — Async set inverse
    pub async fn set_inverse_async(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.core.set_inverse_async(inverse).await
    }

    /// SSD1312 §6.10 — Async set charge pump output voltage
    pub async fn set_charge_pump_voltage_async(
        &mut self,
        voltage: ChargePumpVoltage1312,
    ) -> Result<(), DisplayError> {
        self.send_raw_command_async(&[0x8D, voltage as u8]).await
    }

    /// SSD1312 §6.8 — Async set IREF selection
    pub async fn set_iref_async(&mut self, setting: IrefSetting1312) -> Result<(), DisplayError> {
        let byte = match setting {
            IrefSetting1312::External => 0x00,
            IrefSetting1312::Internal => 0x10,
        };
        self.send_raw_command_async(&[0xAD, byte]).await
    }
}

// --- Async API (Unbuffered) ---

impl<DI: AsyncDisplayInterface> Ssd1312<DI, Unbuffered> {
    async fn send_raw_command_async(&mut self, bytes: &[u8]) -> Result<(), DisplayError> {
        self.core.interface.send_command(bytes).await
    }

    /// SSD1312 §6.9.2 — Async power ON initialization sequence (unbuffered).
    pub async fn init_async(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        self.core.init_common_async(config).await?;
        self.send_raw_command_async(&[0xAD, 0x10]).await?;
        self.send_raw_command_async(&[0x8D, ChargePumpVoltage1312::V7_5 as u8])
            .await?;
        if config.display_on {
            self.core
                .send_command_async(Command::DisplayOn(true))
                .await?;
        }
        Ok(())
    }

    /// SSD1312 §6.9 — Async display ON/OFF
    pub async fn display_on_async(&mut self, on: bool) -> Result<(), DisplayError> {
        self.core.display_on_async(on).await
    }

    /// SSD1312 §6.6 — Async set contrast
    pub async fn set_contrast_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.core.set_contrast_async(contrast).await
    }

    /// SSD1312 — Async set inverse
    pub async fn set_inverse_async(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.core.set_inverse_async(inverse).await
    }

    /// SSD1312 §6.10 — Async set charge pump output voltage
    pub async fn set_charge_pump_voltage_async(
        &mut self,
        voltage: ChargePumpVoltage1312,
    ) -> Result<(), DisplayError> {
        self.send_raw_command_async(&[0x8D, voltage as u8]).await
    }

    /// SSD1312 §6.8 — Async set IREF selection
    pub async fn set_iref_async(&mut self, setting: IrefSetting1312) -> Result<(), DisplayError> {
        let byte = match setting {
            IrefSetting1312::External => 0x00,
            IrefSetting1312::Internal => 0x10,
        };
        self.send_raw_command_async(&[0xAD, byte]).await
    }
}

// --- embedded-graphics DrawTarget (Buffered only) ---

impl<DI, MODE: DisplayBufferMode> OriginDimensions for Ssd1312<DI, MODE> {
    fn size(&self) -> Size {
        Size::new(self.core.size.width as u32, self.core.size.height as u32)
    }
}

impl<DI> DrawTarget for Ssd1312<DI, Buffered> {
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
