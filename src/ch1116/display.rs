use super::init::DEFAULT_DISPLAY_SIZE;
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::BinaryColor,
    Pixel,
};

use super::command::{
    BreathingBrightnessCh1116, IrefResistorCh1116, IrefSourceCh1116, PumpVoltageCh1116,
    ScrollDirectionCh1116, ScrollIntervalCh1116, ScrollModeCh1116,
};
use crate::common::{
    Buffered, DisplayBufferMode, DisplayCore, DisplaySize, InitConfig, Unbuffered,
};
use crate::error::DisplayError;
use crate::interface::{AsyncDisplayInterface, DisplayInterface};

/// CH1116 OLED display driver.
///
/// Generic over the bus interface `DI` (I2C or SPI) and buffer mode `MODE`.
/// In `Buffered` mode (default) the driver owns a framebuffer and supports all drawing APIs.
/// In `Unbuffered` mode the driver owns no RAM buffer; only control commands are available.
pub struct Ch1116<DI, MODE: DisplayBufferMode = Buffered> {
    core: DisplayCore<DI, MODE>,
}

impl<DI> Ch1116<DI, Buffered> {
    /// Create a new CH1116 driver with default 132x64 display size (buffered).
    pub fn new(interface: DI) -> Self {
        Self {
            core: DisplayCore::new(interface, DEFAULT_DISPLAY_SIZE),
        }
    }

    /// Create a new CH1116 driver with a custom display size (buffered).
    pub fn new_with_size(interface: DI, size: DisplaySize) -> Self {
        Self {
            core: DisplayCore::new(interface, size),
        }
    }

    pub fn release(self) -> DI {
        self.core.interface
    }

    /// CH1116 — Each byte represents 8 vertical pixels (D0=top, D7=bottom) within one page.
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

impl<DI> Ch1116<DI, Unbuffered> {
    /// Create a new CH1116 driver in unbuffered mode with default 132x64 display size.
    pub fn new_unbuffered(interface: DI) -> Self {
        Self {
            core: DisplayCore::new(interface, DEFAULT_DISPLAY_SIZE),
        }
    }

    /// Create a new CH1116 driver in unbuffered mode with a custom display size.
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

fn ch1116_init_commands(
    interface: &mut impl DisplayInterface,
    config: &InitConfig,
) -> Result<(), crate::error::DisplayError> {
    interface.send_command(&[0xAE])?;
    interface.send_command(&[0xD5, 0x50])?;
    interface.send_command(&[0xA8, 0x3F])?;
    interface.send_command(&[0xD3, config.display_offset & 0x3F])?;
    interface.send_command(&[0x40 | (config.display_start_line & 0x3F)])?;
    interface.send_command(&[0xAD, 0x8B])?;
    if config.segment_remap {
        interface.send_command(&[0xA1])?;
    } else {
        interface.send_command(&[0xA0])?;
    }
    if config.com_scan_reversed {
        interface.send_command(&[0xC8])?;
    } else {
        interface.send_command(&[0xC0])?;
    }
    interface.send_command(&[0xDA, 0x12])?;
    interface.send_command(&[0x81, config.contrast])?;
    interface.send_command(&[0xD9, 0x66])?;
    interface.send_command(&[0xDB, 0x35])?;
    interface.send_command(&[PumpVoltageCh1116::V7_4 as u8])?;
    if config.entire_display_on {
        interface.send_command(&[0xA5])?;
    } else {
        interface.send_command(&[0xA4])?;
    }
    if config.inverse_display {
        interface.send_command(&[0xA7])?;
    } else {
        interface.send_command(&[0xA6])?;
    }
    if config.deactivate_scroll {
        interface.send_command(&[0x2E])?;
    }
    if config.display_on {
        interface.send_command(&[0xAF])?;
    }
    Ok(())
}

// --- Synchronous API (Buffered) ---

impl<DI: DisplayInterface> Ch1116<DI, Buffered> {
    fn send_raw_command(&mut self, bytes: &[u8]) -> Result<(), DisplayError> {
        self.core.interface.send_command(bytes)
    }

    /// CH1116 — Power ON initialization sequence.
    pub fn init(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        ch1116_init_commands(&mut self.core.interface, config)
    }

    /// CH1116 — Flush the internal framebuffer to the display GDDRAM (page mode).
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

    /// CH1116 §17 — Turn the display panel ON or OFF (sleep mode)
    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        if on {
            self.send_raw_command(&[0xAF])
        } else {
            self.send_raw_command(&[0xAE])
        }
    }

    /// CH1116 §10 — Set contrast (0-255)
    pub fn set_contrast(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.send_raw_command(&[0x81, contrast])
    }

    /// CH1116 §14 — Invert the display (swap ON/OFF pixels)
    pub fn set_inverse(&mut self, inverse: bool) -> Result<(), DisplayError> {
        if inverse {
            self.send_raw_command(&[0xA7])
        } else {
            self.send_raw_command(&[0xA6])
        }
    }

    /// CH1116 §8 — Set pump output voltage
    pub fn set_pump_voltage(&mut self, voltage: PumpVoltageCh1116) -> Result<(), DisplayError> {
        self.send_raw_command(&[voltage as u8])
    }

    /// CH1116 §11 — Set IREF source and internal resistor value
    pub fn set_iref(
        &mut self,
        source: IrefSourceCh1116,
        resistor: IrefResistorCh1116,
    ) -> Result<(), DisplayError> {
        let d_bit: u8 = match source {
            IrefSourceCh1116::External => 0x00,
            IrefSourceCh1116::Internal => 0x04,
        };
        self.send_raw_command(&[0x82, d_bit | (resistor as u8)])
    }

    /// CH1116 §25 — Set VCOM deselect level (raw 8-bit value)
    pub fn set_vcom_deselect(&mut self, level: u8) -> Result<(), DisplayError> {
        self.send_raw_command(&[0xDB, level])
    }

    /// CH1116 §16 — Set DC-DC converter ON or OFF
    pub fn set_dc_dc(&mut self, enable: bool) -> Result<(), DisplayError> {
        self.send_raw_command(&[0xAD, if enable { 0x8B } else { 0x8A }])
    }

    /// CH1116 §3 — Set breathing light effect
    pub fn set_breathing_light(
        &mut self,
        enable: bool,
        brightness: BreathingBrightnessCh1116,
        interval: u8,
    ) -> Result<(), DisplayError> {
        let on_off: u8 = if enable { 0x80 } else { 0x00 };
        self.send_raw_command(&[0x23, on_off | (brightness as u8) | (interval & 0x07)])
    }

    /// CH1116 §5 — Configure and activate horizontal scroll
    pub fn start_horizontal_scroll(
        &mut self,
        direction: ScrollDirectionCh1116,
        start_page: u8,
        end_page: u8,
        interval: ScrollIntervalCh1116,
    ) -> Result<(), DisplayError> {
        let max_page = self.core.size.pages() - 1;
        if start_page > max_page || end_page > max_page || end_page < start_page {
            return Err(DisplayError::InvalidArgument);
        }
        self.send_raw_command(&[0x2E])?;
        let cmd_byte: u8 = match direction {
            ScrollDirectionCh1116::Right => 0x26,
            ScrollDirectionCh1116::Left => 0x27,
        };
        self.send_raw_command(&[cmd_byte, start_page & 0x07, interval as u8, end_page & 0x07])?;
        self.send_raw_command(&[0x2F])
    }

    /// CH1116 §4 — Set additional horizontal scroll column bounds
    pub fn set_scroll_columns(
        &mut self,
        start_column: u8,
        end_column: u8,
    ) -> Result<(), DisplayError> {
        let max_col = (self.core.size.width - 1) as u8;
        if start_column > max_col || end_column > max_col || end_column < start_column {
            return Err(DisplayError::InvalidArgument);
        }
        self.send_raw_command(&[0x24, start_column, end_column])
    }

    /// CH1116 §6 — Set scroll mode
    pub fn set_scroll_mode(&mut self, mode: ScrollModeCh1116) -> Result<(), DisplayError> {
        let d_bit: u8 = match mode {
            ScrollModeCh1116::Continuous => 0x2C,
            ScrollModeCh1116::SingleScreen => 0x2D,
        };
        self.send_raw_command(&[d_bit])
    }

    /// CH1116 §7 — Stop scrolling
    pub fn stop_scroll(&mut self) -> Result<(), DisplayError> {
        self.send_raw_command(&[0x2E])
    }

    /// CH1116 §7 — Activate scrolling
    pub fn activate_scroll(&mut self) -> Result<(), DisplayError> {
        self.send_raw_command(&[0x2F])
    }

    /// CH1116 §9 — Set display start line (0-63)
    pub fn set_display_start_line(&mut self, line: u8) -> Result<(), DisplayError> {
        self.send_raw_command(&[0x40 | (line & 0x3F)])
    }

    /// CH1116 §15 — Set multiplex ratio (1-64)
    pub fn set_multiplex_ratio(&mut self, ratio: u8) -> Result<(), DisplayError> {
        self.send_raw_command(&[0xA8, ratio.saturating_sub(1) & 0x3F])
    }

    /// CH1116 §20 — Set display offset (0-63)
    pub fn set_display_offset(&mut self, offset: u8) -> Result<(), DisplayError> {
        self.send_raw_command(&[0xD3, offset & 0x3F])
    }
}

// --- Synchronous API (Unbuffered) ---

impl<DI: DisplayInterface> Ch1116<DI, Unbuffered> {
    fn send_raw_command(&mut self, bytes: &[u8]) -> Result<(), DisplayError> {
        self.core.interface.send_command(bytes)
    }

    /// CH1116 — Power ON initialization sequence (unbuffered).
    pub fn init(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        ch1116_init_commands(&mut self.core.interface, config)
    }

    /// CH1116 §17 — Turn the display panel ON or OFF (sleep mode)
    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        if on {
            self.send_raw_command(&[0xAF])
        } else {
            self.send_raw_command(&[0xAE])
        }
    }

    /// CH1116 §10 — Set contrast (0-255)
    pub fn set_contrast(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.send_raw_command(&[0x81, contrast])
    }

    /// CH1116 §14 — Invert the display (swap ON/OFF pixels)
    pub fn set_inverse(&mut self, inverse: bool) -> Result<(), DisplayError> {
        if inverse {
            self.send_raw_command(&[0xA7])
        } else {
            self.send_raw_command(&[0xA6])
        }
    }

    /// CH1116 §8 — Set pump output voltage
    pub fn set_pump_voltage(&mut self, voltage: PumpVoltageCh1116) -> Result<(), DisplayError> {
        self.send_raw_command(&[voltage as u8])
    }

    /// CH1116 §11 — Set IREF source and internal resistor value
    pub fn set_iref(
        &mut self,
        source: IrefSourceCh1116,
        resistor: IrefResistorCh1116,
    ) -> Result<(), DisplayError> {
        let d_bit: u8 = match source {
            IrefSourceCh1116::External => 0x00,
            IrefSourceCh1116::Internal => 0x04,
        };
        self.send_raw_command(&[0x82, d_bit | (resistor as u8)])
    }

    /// CH1116 §25 — Set VCOM deselect level
    pub fn set_vcom_deselect(&mut self, level: u8) -> Result<(), DisplayError> {
        self.send_raw_command(&[0xDB, level])
    }

    /// CH1116 §16 — Set DC-DC converter ON or OFF
    pub fn set_dc_dc(&mut self, enable: bool) -> Result<(), DisplayError> {
        self.send_raw_command(&[0xAD, if enable { 0x8B } else { 0x8A }])
    }

    /// CH1116 §3 — Set breathing light effect
    pub fn set_breathing_light(
        &mut self,
        enable: bool,
        brightness: BreathingBrightnessCh1116,
        interval: u8,
    ) -> Result<(), DisplayError> {
        let on_off: u8 = if enable { 0x80 } else { 0x00 };
        self.send_raw_command(&[0x23, on_off | (brightness as u8) | (interval & 0x07)])
    }

    /// CH1116 §5 — Configure and activate horizontal scroll
    pub fn start_horizontal_scroll(
        &mut self,
        direction: ScrollDirectionCh1116,
        start_page: u8,
        end_page: u8,
        interval: ScrollIntervalCh1116,
    ) -> Result<(), DisplayError> {
        let max_page = self.core.size.pages() - 1;
        if start_page > max_page || end_page > max_page || end_page < start_page {
            return Err(DisplayError::InvalidArgument);
        }
        self.send_raw_command(&[0x2E])?;
        let cmd_byte: u8 = match direction {
            ScrollDirectionCh1116::Right => 0x26,
            ScrollDirectionCh1116::Left => 0x27,
        };
        self.send_raw_command(&[cmd_byte, start_page & 0x07, interval as u8, end_page & 0x07])?;
        self.send_raw_command(&[0x2F])
    }

    /// CH1116 §4 — Set additional horizontal scroll column bounds
    pub fn set_scroll_columns(
        &mut self,
        start_column: u8,
        end_column: u8,
    ) -> Result<(), DisplayError> {
        let max_col = (self.core.size.width - 1) as u8;
        if start_column > max_col || end_column > max_col || end_column < start_column {
            return Err(DisplayError::InvalidArgument);
        }
        self.send_raw_command(&[0x24, start_column, end_column])
    }

    /// CH1116 §6 — Set scroll mode
    pub fn set_scroll_mode(&mut self, mode: ScrollModeCh1116) -> Result<(), DisplayError> {
        let d_bit: u8 = match mode {
            ScrollModeCh1116::Continuous => 0x2C,
            ScrollModeCh1116::SingleScreen => 0x2D,
        };
        self.send_raw_command(&[d_bit])
    }

    /// CH1116 §7 — Stop scrolling
    pub fn stop_scroll(&mut self) -> Result<(), DisplayError> {
        self.send_raw_command(&[0x2E])
    }

    /// CH1116 §7 — Activate scrolling
    pub fn activate_scroll(&mut self) -> Result<(), DisplayError> {
        self.send_raw_command(&[0x2F])
    }

    /// CH1116 §9 — Set display start line (0-63)
    pub fn set_display_start_line(&mut self, line: u8) -> Result<(), DisplayError> {
        self.send_raw_command(&[0x40 | (line & 0x3F)])
    }

    /// CH1116 §15 — Set multiplex ratio (1-64)
    pub fn set_multiplex_ratio(&mut self, ratio: u8) -> Result<(), DisplayError> {
        self.send_raw_command(&[0xA8, ratio.saturating_sub(1) & 0x3F])
    }

    /// CH1116 §20 — Set display offset (0-63)
    pub fn set_display_offset(&mut self, offset: u8) -> Result<(), DisplayError> {
        self.send_raw_command(&[0xD3, offset & 0x3F])
    }
}

// --- Async API (Buffered) ---

impl<DI: AsyncDisplayInterface> Ch1116<DI, Buffered> {
    async fn send_raw_command_async(&mut self, bytes: &[u8]) -> Result<(), DisplayError> {
        self.core.interface.send_command(bytes).await
    }

    /// CH1116 — Async power ON initialization sequence.
    pub async fn init_async(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        self.send_raw_command_async(&[0xAE]).await?;
        self.send_raw_command_async(&[0xD5, 0x50]).await?;
        self.send_raw_command_async(&[0xA8, 0x3F]).await?;
        self.send_raw_command_async(&[0xD3, config.display_offset & 0x3F])
            .await?;
        self.send_raw_command_async(&[0x40 | (config.display_start_line & 0x3F)])
            .await?;
        self.send_raw_command_async(&[0xAD, 0x8B]).await?;
        if config.segment_remap {
            self.send_raw_command_async(&[0xA1]).await?;
        } else {
            self.send_raw_command_async(&[0xA0]).await?;
        }
        if config.com_scan_reversed {
            self.send_raw_command_async(&[0xC8]).await?;
        } else {
            self.send_raw_command_async(&[0xC0]).await?;
        }
        self.send_raw_command_async(&[0xDA, 0x12]).await?;
        self.send_raw_command_async(&[0x81, config.contrast])
            .await?;
        self.send_raw_command_async(&[0xD9, 0x66]).await?;
        self.send_raw_command_async(&[0xDB, 0x35]).await?;
        self.send_raw_command_async(&[PumpVoltageCh1116::V7_4 as u8])
            .await?;
        if config.entire_display_on {
            self.send_raw_command_async(&[0xA5]).await?;
        } else {
            self.send_raw_command_async(&[0xA4]).await?;
        }
        if config.inverse_display {
            self.send_raw_command_async(&[0xA7]).await?;
        } else {
            self.send_raw_command_async(&[0xA6]).await?;
        }
        if config.deactivate_scroll {
            self.send_raw_command_async(&[0x2E]).await?;
        }
        if config.display_on {
            self.send_raw_command_async(&[0xAF]).await?;
        }
        Ok(())
    }

    /// CH1116 — Async flush of framebuffer to display GDDRAM (page mode).
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

    /// CH1116 §17 — Async display ON/OFF
    pub async fn display_on_async(&mut self, on: bool) -> Result<(), DisplayError> {
        if on {
            self.send_raw_command_async(&[0xAF]).await
        } else {
            self.send_raw_command_async(&[0xAE]).await
        }
    }

    /// CH1116 §10 — Async set contrast
    pub async fn set_contrast_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.send_raw_command_async(&[0x81, contrast]).await
    }

    /// CH1116 §14 — Async set inverse
    pub async fn set_inverse_async(&mut self, inverse: bool) -> Result<(), DisplayError> {
        if inverse {
            self.send_raw_command_async(&[0xA7]).await
        } else {
            self.send_raw_command_async(&[0xA6]).await
        }
    }

    /// CH1116 §8 — Async set pump output voltage
    pub async fn set_pump_voltage_async(
        &mut self,
        voltage: PumpVoltageCh1116,
    ) -> Result<(), DisplayError> {
        self.send_raw_command_async(&[voltage as u8]).await
    }

    /// CH1116 §11 — Async set IREF
    pub async fn set_iref_async(
        &mut self,
        source: IrefSourceCh1116,
        resistor: IrefResistorCh1116,
    ) -> Result<(), DisplayError> {
        let d_bit: u8 = match source {
            IrefSourceCh1116::External => 0x00,
            IrefSourceCh1116::Internal => 0x04,
        };
        self.send_raw_command_async(&[0x82, d_bit | (resistor as u8)])
            .await
    }
}

// --- Async API (Unbuffered) ---

impl<DI: AsyncDisplayInterface> Ch1116<DI, Unbuffered> {
    async fn send_raw_command_async(&mut self, bytes: &[u8]) -> Result<(), DisplayError> {
        self.core.interface.send_command(bytes).await
    }

    /// CH1116 — Async power ON initialization sequence (unbuffered).
    pub async fn init_async(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        self.send_raw_command_async(&[0xAE]).await?;
        self.send_raw_command_async(&[0xD5, 0x50]).await?;
        self.send_raw_command_async(&[0xA8, 0x3F]).await?;
        self.send_raw_command_async(&[0xD3, config.display_offset & 0x3F])
            .await?;
        self.send_raw_command_async(&[0x40 | (config.display_start_line & 0x3F)])
            .await?;
        self.send_raw_command_async(&[0xAD, 0x8B]).await?;
        if config.segment_remap {
            self.send_raw_command_async(&[0xA1]).await?;
        } else {
            self.send_raw_command_async(&[0xA0]).await?;
        }
        if config.com_scan_reversed {
            self.send_raw_command_async(&[0xC8]).await?;
        } else {
            self.send_raw_command_async(&[0xC0]).await?;
        }
        self.send_raw_command_async(&[0xDA, 0x12]).await?;
        self.send_raw_command_async(&[0x81, config.contrast])
            .await?;
        self.send_raw_command_async(&[0xD9, 0x66]).await?;
        self.send_raw_command_async(&[0xDB, 0x35]).await?;
        self.send_raw_command_async(&[PumpVoltageCh1116::V7_4 as u8])
            .await?;
        if config.entire_display_on {
            self.send_raw_command_async(&[0xA5]).await?;
        } else {
            self.send_raw_command_async(&[0xA4]).await?;
        }
        if config.inverse_display {
            self.send_raw_command_async(&[0xA7]).await?;
        } else {
            self.send_raw_command_async(&[0xA6]).await?;
        }
        if config.deactivate_scroll {
            self.send_raw_command_async(&[0x2E]).await?;
        }
        if config.display_on {
            self.send_raw_command_async(&[0xAF]).await?;
        }
        Ok(())
    }

    /// CH1116 §17 — Async display ON/OFF
    pub async fn display_on_async(&mut self, on: bool) -> Result<(), DisplayError> {
        if on {
            self.send_raw_command_async(&[0xAF]).await
        } else {
            self.send_raw_command_async(&[0xAE]).await
        }
    }

    /// CH1116 §10 — Async set contrast
    pub async fn set_contrast_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.send_raw_command_async(&[0x81, contrast]).await
    }

    /// CH1116 §14 — Async set inverse
    pub async fn set_inverse_async(&mut self, inverse: bool) -> Result<(), DisplayError> {
        if inverse {
            self.send_raw_command_async(&[0xA7]).await
        } else {
            self.send_raw_command_async(&[0xA6]).await
        }
    }

    /// CH1116 §8 — Async set pump output voltage
    pub async fn set_pump_voltage_async(
        &mut self,
        voltage: PumpVoltageCh1116,
    ) -> Result<(), DisplayError> {
        self.send_raw_command_async(&[voltage as u8]).await
    }

    /// CH1116 §11 — Async set IREF
    pub async fn set_iref_async(
        &mut self,
        source: IrefSourceCh1116,
        resistor: IrefResistorCh1116,
    ) -> Result<(), DisplayError> {
        let d_bit: u8 = match source {
            IrefSourceCh1116::External => 0x00,
            IrefSourceCh1116::Internal => 0x04,
        };
        self.send_raw_command_async(&[0x82, d_bit | (resistor as u8)])
            .await
    }
}

// --- embedded-graphics DrawTarget (Buffered only) ---

impl<DI, MODE: DisplayBufferMode> OriginDimensions for Ch1116<DI, MODE> {
    fn size(&self) -> Size {
        Size::new(self.core.size.width as u32, self.core.size.height as u32)
    }
}

impl<DI> DrawTarget for Ch1116<DI, Buffered> {
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
