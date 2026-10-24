use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::BinaryColor,
    Pixel,
};

use super::command::{ChargePumpVoltage, FadeMode, IrefSetting, ScrollIntervalB, VcomhLevelB};
use crate::common::{
    Buffered, DisplayBufferMode, DisplayCore, DisplaySize, InitConfig, Unbuffered, SIZE_128X64,
};
use crate::error::DisplayError;
use crate::interface::{AsyncDisplayInterface, DisplayInterface};
use crate::ssd1306::command::{Command, ScrollDirection};

/// SSD1306B OLED display driver.
///
/// Generic over the bus interface `DI` (I2C or SPI) and buffer mode `MODE`.
/// In `Buffered` mode (default) the driver owns a framebuffer and supports all drawing APIs.
/// In `Unbuffered` mode the driver owns no RAM buffer; only control commands are available.
pub struct Ssd1306b<DI, MODE: DisplayBufferMode = Buffered> {
    core: DisplayCore<DI, MODE>,
}

impl<DI> Ssd1306b<DI, Buffered> {
    /// Create a new SSD1306B driver with default 128x64 display size (buffered).
    pub fn new(interface: DI) -> Self {
        Self {
            core: DisplayCore::new(interface, SIZE_128X64),
        }
    }

    /// Create a new SSD1306B driver with a custom display size (buffered).
    pub fn new_with_size(interface: DI, size: DisplaySize) -> Self {
        Self {
            core: DisplayCore::new(interface, size),
        }
    }

    pub fn release(self) -> DI {
        self.core.interface
    }

    /// SSD1306B §8 — Each byte represents 8 vertical pixels (D0=top, D7=bottom) within one page.
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

impl<DI> Ssd1306b<DI, Unbuffered> {
    /// Create a new SSD1306B driver in unbuffered mode with default 128x64 display size.
    pub fn new_unbuffered(interface: DI) -> Self {
        Self {
            core: DisplayCore::new(interface, SIZE_128X64),
        }
    }

    /// Create a new SSD1306B driver in unbuffered mode with a custom display size.
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

impl<DI: DisplayInterface> Ssd1306b<DI, Buffered> {
    fn send_command(&mut self, cmd: Command) -> Result<(), DisplayError> {
        self.core.send_command(cmd)
    }

    fn send_raw_command(&mut self, bytes: &[u8]) -> Result<(), DisplayError> {
        self.core.interface.send_command(bytes)
    }

    /// SSD1306B §8 — Power ON initialization sequence.
    pub fn init(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        self.core.init_common(config)?;
        self.send_raw_command(&[0xAD, 0x10])?;
        self.send_raw_command(&[0x8D, ChargePumpVoltage::V7_5 as u8])?;
        if config.display_on {
            self.send_command(Command::DisplayOn(true))?;
        }
        Ok(())
    }

    /// Flush the internal framebuffer to the display GDDRAM.
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        self.core.flush()
    }

    /// SSD1306B §8 — Turn the display panel ON or OFF (sleep mode)
    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.core.display_on(on)
    }

    /// SSD1306B §9.1.7 — Set contrast (0-255)
    pub fn set_contrast(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.core.set_contrast(contrast)
    }

    /// SSD1306B §9.1.10 — Invert the display (swap ON/OFF pixels)
    pub fn set_inverse(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.core.set_inverse(inverse)
    }

    /// SSD1306B §8 — Set charge pump output voltage
    pub fn set_charge_pump_voltage(
        &mut self,
        voltage: ChargePumpVoltage,
    ) -> Result<(), DisplayError> {
        self.send_raw_command(&[0x8D, voltage as u8])
    }

    /// SSD1306B §8 — Set internal IREF
    pub fn set_iref(&mut self, setting: IrefSetting) -> Result<(), DisplayError> {
        let byte = match setting {
            IrefSetting::Low => 0x10,
            IrefSetting::High => 0x30,
        };
        self.send_raw_command(&[0xAD, byte])
    }

    /// SSD1306B §8 — Set VCOMH deselect level (4 levels)
    pub fn set_vcomh_deselect(&mut self, level: VcomhLevelB) -> Result<(), DisplayError> {
        self.send_raw_command(&[0xDB, level as u8])
    }

    /// SSD1306B §8 — Set fade out or blinking mode
    pub fn set_fade_blink(&mut self, mode: FadeMode, interval: u8) -> Result<(), DisplayError> {
        if interval > 0x0F {
            return Err(DisplayError::InvalidArgument);
        }
        self.send_raw_command(&[0x23, (mode as u8) | (interval & 0x0F)])
    }

    /// SSD1306B §8 — Enable/disable zoom in mode
    pub fn set_zoom(&mut self, enable: bool) -> Result<(), DisplayError> {
        self.send_raw_command(&[0xD6, if enable { 0x01 } else { 0x00 }])
    }

    /// SSD1306B §8 — Configure and activate horizontal scrolling with column bounds
    pub fn start_horizontal_scroll(
        &mut self,
        direction: ScrollDirection,
        start_page: u8,
        end_page: u8,
        interval: ScrollIntervalB,
        start_column: u8,
        end_column: u8,
    ) -> Result<(), DisplayError> {
        let max_page = self.core.size.pages() - 1;
        let max_col = (self.core.size.width - 1) as u8;
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

    /// SSD1306B §8 — Content scroll with column address bounds
    pub fn start_content_scroll(
        &mut self,
        direction: ScrollDirection,
        start_page: u8,
        end_page: u8,
        interval: ScrollIntervalB,
        start_column: u8,
        end_column: u8,
    ) -> Result<(), DisplayError> {
        let max_page = self.core.size.pages() - 1;
        let max_col = (self.core.size.width - 1) as u8;
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

    /// SSD1306B §8 — Stop scrolling
    pub fn stop_scroll(&mut self) -> Result<(), DisplayError> {
        self.core.stop_scroll()
    }
}

// --- Synchronous API (Unbuffered) ---

impl<DI: DisplayInterface> Ssd1306b<DI, Unbuffered> {
    fn send_raw_command(&mut self, bytes: &[u8]) -> Result<(), DisplayError> {
        self.core.interface.send_command(bytes)
    }

    /// SSD1306B §8 — Power ON initialization sequence (unbuffered).
    pub fn init(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        self.core.init_common(config)?;
        self.send_raw_command(&[0xAD, 0x10])?;
        self.send_raw_command(&[0x8D, ChargePumpVoltage::V7_5 as u8])?;
        if config.display_on {
            self.core.send_command(Command::DisplayOn(true))?;
        }
        Ok(())
    }

    /// SSD1306B §8 — Turn the display panel ON or OFF (sleep mode)
    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.core.display_on(on)
    }

    /// SSD1306B §9.1.7 — Set contrast (0-255)
    pub fn set_contrast(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.core.set_contrast(contrast)
    }

    /// SSD1306B §9.1.10 — Invert the display (swap ON/OFF pixels)
    pub fn set_inverse(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.core.set_inverse(inverse)
    }

    /// SSD1306B §8 — Set charge pump output voltage
    pub fn set_charge_pump_voltage(
        &mut self,
        voltage: ChargePumpVoltage,
    ) -> Result<(), DisplayError> {
        self.send_raw_command(&[0x8D, voltage as u8])
    }

    /// SSD1306B §8 — Set internal IREF
    pub fn set_iref(&mut self, setting: IrefSetting) -> Result<(), DisplayError> {
        let byte = match setting {
            IrefSetting::Low => 0x10,
            IrefSetting::High => 0x30,
        };
        self.send_raw_command(&[0xAD, byte])
    }

    /// SSD1306B §8 — Set VCOMH deselect level (4 levels)
    pub fn set_vcomh_deselect(&mut self, level: VcomhLevelB) -> Result<(), DisplayError> {
        self.send_raw_command(&[0xDB, level as u8])
    }

    /// SSD1306B §8 — Set fade out or blinking mode
    pub fn set_fade_blink(&mut self, mode: FadeMode, interval: u8) -> Result<(), DisplayError> {
        if interval > 0x0F {
            return Err(DisplayError::InvalidArgument);
        }
        self.send_raw_command(&[0x23, (mode as u8) | (interval & 0x0F)])
    }

    /// SSD1306B §8 — Enable/disable zoom in mode
    pub fn set_zoom(&mut self, enable: bool) -> Result<(), DisplayError> {
        self.send_raw_command(&[0xD6, if enable { 0x01 } else { 0x00 }])
    }

    /// SSD1306B §8 — Configure and activate horizontal scrolling with column bounds
    pub fn start_horizontal_scroll(
        &mut self,
        direction: ScrollDirection,
        start_page: u8,
        end_page: u8,
        interval: ScrollIntervalB,
        start_column: u8,
        end_column: u8,
    ) -> Result<(), DisplayError> {
        let max_page = self.core.size.pages() - 1;
        let max_col = (self.core.size.width - 1) as u8;
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

    /// SSD1306B §8 — Content scroll with column address bounds
    pub fn start_content_scroll(
        &mut self,
        direction: ScrollDirection,
        start_page: u8,
        end_page: u8,
        interval: ScrollIntervalB,
        start_column: u8,
        end_column: u8,
    ) -> Result<(), DisplayError> {
        let max_page = self.core.size.pages() - 1;
        let max_col = (self.core.size.width - 1) as u8;
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

    /// SSD1306B §8 — Stop scrolling
    pub fn stop_scroll(&mut self) -> Result<(), DisplayError> {
        self.core.stop_scroll()
    }
}

// --- Async API (Buffered) ---

impl<DI: AsyncDisplayInterface> Ssd1306b<DI, Buffered> {
    async fn send_raw_command_async(&mut self, bytes: &[u8]) -> Result<(), DisplayError> {
        self.core.interface.send_command(bytes).await
    }

    /// SSD1306B §8 — Async power ON initialization sequence.
    pub async fn init_async(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        self.core.init_common_async(config).await?;
        self.send_raw_command_async(&[0xAD, 0x10]).await?;
        self.send_raw_command_async(&[0x8D, ChargePumpVoltage::V7_5 as u8])
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

    /// SSD1306B §8 — Async display ON/OFF
    pub async fn display_on_async(&mut self, on: bool) -> Result<(), DisplayError> {
        self.core.display_on_async(on).await
    }

    /// SSD1306B §9.1.7 — Async set contrast
    pub async fn set_contrast_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.core.set_contrast_async(contrast).await
    }

    /// SSD1306B §9.1.10 — Async set inverse
    pub async fn set_inverse_async(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.core.set_inverse_async(inverse).await
    }

    /// SSD1306B §8 — Async set charge pump output voltage
    pub async fn set_charge_pump_voltage_async(
        &mut self,
        voltage: ChargePumpVoltage,
    ) -> Result<(), DisplayError> {
        self.send_raw_command_async(&[0x8D, voltage as u8]).await
    }

    /// SSD1306B §8 — Async set internal IREF
    pub async fn set_iref_async(&mut self, setting: IrefSetting) -> Result<(), DisplayError> {
        let byte = match setting {
            IrefSetting::Low => 0x10,
            IrefSetting::High => 0x30,
        };
        self.send_raw_command_async(&[0xAD, byte]).await
    }

    /// SSD1306B §8 — Async set fade/blink mode
    pub async fn set_fade_blink_async(
        &mut self,
        mode: FadeMode,
        interval: u8,
    ) -> Result<(), DisplayError> {
        if interval > 0x0F {
            return Err(DisplayError::InvalidArgument);
        }
        self.send_raw_command_async(&[0x23, (mode as u8) | (interval & 0x0F)])
            .await
    }

    /// SSD1306B §8 — Async enable/disable zoom
    pub async fn set_zoom_async(&mut self, enable: bool) -> Result<(), DisplayError> {
        self.send_raw_command_async(&[0xD6, if enable { 0x01 } else { 0x00 }])
            .await
    }
}

// --- Async API (Unbuffered) ---

impl<DI: AsyncDisplayInterface> Ssd1306b<DI, Unbuffered> {
    async fn send_raw_command_async(&mut self, bytes: &[u8]) -> Result<(), DisplayError> {
        self.core.interface.send_command(bytes).await
    }

    /// SSD1306B §8 — Async power ON initialization sequence (unbuffered).
    pub async fn init_async(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        self.core.init_common_async(config).await?;
        self.send_raw_command_async(&[0xAD, 0x10]).await?;
        self.send_raw_command_async(&[0x8D, ChargePumpVoltage::V7_5 as u8])
            .await?;
        if config.display_on {
            self.core
                .send_command_async(Command::DisplayOn(true))
                .await?;
        }
        Ok(())
    }

    /// SSD1306B §8 — Async display ON/OFF
    pub async fn display_on_async(&mut self, on: bool) -> Result<(), DisplayError> {
        self.core.display_on_async(on).await
    }

    /// SSD1306B §9.1.7 — Async set contrast
    pub async fn set_contrast_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.core.set_contrast_async(contrast).await
    }

    /// SSD1306B §9.1.10 — Async set inverse
    pub async fn set_inverse_async(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.core.set_inverse_async(inverse).await
    }

    /// SSD1306B §8 — Async set charge pump output voltage
    pub async fn set_charge_pump_voltage_async(
        &mut self,
        voltage: ChargePumpVoltage,
    ) -> Result<(), DisplayError> {
        self.send_raw_command_async(&[0x8D, voltage as u8]).await
    }

    /// SSD1306B §8 — Async set internal IREF
    pub async fn set_iref_async(&mut self, setting: IrefSetting) -> Result<(), DisplayError> {
        let byte = match setting {
            IrefSetting::Low => 0x10,
            IrefSetting::High => 0x30,
        };
        self.send_raw_command_async(&[0xAD, byte]).await
    }

    /// SSD1306B §8 — Async set fade/blink mode
    pub async fn set_fade_blink_async(
        &mut self,
        mode: FadeMode,
        interval: u8,
    ) -> Result<(), DisplayError> {
        if interval > 0x0F {
            return Err(DisplayError::InvalidArgument);
        }
        self.send_raw_command_async(&[0x23, (mode as u8) | (interval & 0x0F)])
            .await
    }

    /// SSD1306B §8 — Async enable/disable zoom
    pub async fn set_zoom_async(&mut self, enable: bool) -> Result<(), DisplayError> {
        self.send_raw_command_async(&[0xD6, if enable { 0x01 } else { 0x00 }])
            .await
    }
}

// --- embedded-graphics DrawTarget (Buffered only) ---

impl<DI, MODE: DisplayBufferMode> OriginDimensions for Ssd1306b<DI, MODE> {
    fn size(&self) -> Size {
        Size::new(self.core.size.width as u32, self.core.size.height as u32)
    }
}

impl<DI> DrawTarget for Ssd1306b<DI, Buffered> {
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
