use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::BinaryColor,
    Pixel,
};

use super::command::{
    BreathingBrightnessCh1115, IrefResistorCh1115, IrefSourceCh1115, PumpVoltageCh1115,
    ScrollDirectionCh1115, ScrollIntervalCh1115, ScrollModeCh1115, SegPadsConfigCh1115,
};
use crate::common::{
    Buffered, DisplayBufferMode, DisplayCore, DisplaySize, InitConfig, Unbuffered, SIZE_128X64,
};
use crate::error::DisplayError;
use crate::interface::{AsyncDisplayInterface, DisplayInterface};

/// CH1115 OLED display driver.
///
/// Generic over the bus interface `DI` (I2C or SPI) and buffer mode `MODE`.
/// In `Buffered` mode (default) the driver owns a framebuffer and supports all drawing APIs.
/// In `Unbuffered` mode the driver owns no RAM buffer; only control commands are available.
pub struct Ch1115<DI, MODE: DisplayBufferMode = Buffered> {
    core: DisplayCore<DI, MODE>,
}

impl<DI> Ch1115<DI, Buffered> {
    /// Create a new CH1115 driver with default 128x64 display size (buffered).
    pub fn new(interface: DI) -> Self {
        Self {
            core: DisplayCore::new(interface, SIZE_128X64),
        }
    }

    /// Create a new CH1115 driver with a custom display size (buffered).
    pub fn new_with_size(interface: DI, size: DisplaySize) -> Self {
        Self {
            core: DisplayCore::new(interface, size),
        }
    }

    pub fn release(self) -> DI {
        self.core.interface
    }

    /// CH1115 — Each byte represents 8 vertical pixels (D0=top, D7=bottom) within one page.
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

impl<DI> Ch1115<DI, Unbuffered> {
    /// Create a new CH1115 driver in unbuffered mode with default 128x64 display size.
    pub fn new_unbuffered(interface: DI) -> Self {
        Self {
            core: DisplayCore::new(interface, SIZE_128X64),
        }
    }

    /// Create a new CH1115 driver in unbuffered mode with a custom display size.
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

// Shared raw-command helper used by both sync impls
fn ch1115_init_commands(
    interface: &mut impl DisplayInterface,
    config: &InitConfig,
    pages: u8,
    width: u8,
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
    interface.send_command(&[0xA2])?;
    interface.send_command(&[0x81, config.contrast])?;
    interface.send_command(&[0xD9, 0x66])?;
    interface.send_command(&[0xDB, 0x35])?;
    interface.send_command(&[PumpVoltageCh1115::V8_0 as u8])?;
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
    let _ = (pages, width);
    Ok(())
}

// --- Synchronous API (Buffered) ---

impl<DI: DisplayInterface> Ch1115<DI, Buffered> {
    fn send_raw_command(&mut self, bytes: &[u8]) -> Result<(), DisplayError> {
        self.core.interface.send_command(bytes)
    }

    /// CH1115 — Power ON initialization sequence.
    pub fn init(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        ch1115_init_commands(
            &mut self.core.interface,
            config,
            self.core.size.pages(),
            self.core.size.width,
        )
    }

    /// CH1115 — Flush the internal framebuffer to the display GDDRAM (page mode).
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        let pages = self.core.size.pages();
        let width = self.core.size.width;
        for page in 0..pages {
            self.send_raw_command(&[0xB0 | page])?;
            self.send_raw_command(&[0x00])?;
            self.send_raw_command(&[0x10])?;
            let start = page as usize * width as usize;
            let end = start + width as usize;
            self.core.interface.send_data(&self.core.buffer[start..end])?;
        }
        Ok(())
    }

    /// CH1115 §18 — Turn the display panel ON or OFF (sleep mode)
    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        if on { self.send_raw_command(&[0xAF]) } else { self.send_raw_command(&[0xAE]) }
    }

    /// CH1115 §10 — Set contrast (0-255)
    pub fn set_contrast(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.send_raw_command(&[0x81, contrast])
    }

    /// CH1115 §15 — Invert the display (swap ON/OFF pixels)
    pub fn set_inverse(&mut self, inverse: bool) -> Result<(), DisplayError> {
        if inverse { self.send_raw_command(&[0xA7]) } else { self.send_raw_command(&[0xA6]) }
    }

    /// CH1115 §8 — Set pump output voltage
    pub fn set_pump_voltage(&mut self, voltage: PumpVoltageCh1115) -> Result<(), DisplayError> {
        self.send_raw_command(&[voltage as u8])
    }

    /// CH1115 §11 — Set IREF source and internal resistor value
    pub fn set_iref(
        &mut self,
        source: IrefSourceCh1115,
        resistor: IrefResistorCh1115,
    ) -> Result<(), DisplayError> {
        let d_bit: u8 = match source {
            IrefSourceCh1115::External => 0x00,
            IrefSourceCh1115::Internal => 0x04,
        };
        self.send_raw_command(&[0x82, d_bit | (resistor as u8)])
    }

    /// CH1115 §25 — Set VCOM deselect level (raw 8-bit value)
    pub fn set_vcom_deselect(&mut self, level: u8) -> Result<(), DisplayError> {
        self.send_raw_command(&[0xDB, level])
    }

    /// CH1115 §17 — Set DC-DC converter ON or OFF
    pub fn set_dc_dc(&mut self, enable: bool) -> Result<(), DisplayError> {
        let mode_byte = if enable { 0x8B } else { 0x8A };
        self.send_raw_command(&[0xAD, mode_byte])
    }

    /// CH1115 §3 — Set breathing light effect
    pub fn set_breathing_light(
        &mut self,
        enable: bool,
        brightness: BreathingBrightnessCh1115,
        interval: u8,
    ) -> Result<(), DisplayError> {
        let on_off: u8 = if enable { 0x80 } else { 0x00 };
        self.send_raw_command(&[0x23, on_off | (brightness as u8) | (interval & 0x07)])
    }

    /// CH1115 §5 — Configure and activate horizontal scroll
    pub fn start_horizontal_scroll(
        &mut self,
        direction: ScrollDirectionCh1115,
        start_page: u8,
        end_page: u8,
        interval: ScrollIntervalCh1115,
    ) -> Result<(), DisplayError> {
        let max_page = self.core.size.pages() - 1;
        if start_page > max_page || end_page > max_page || end_page < start_page {
            return Err(DisplayError::InvalidArgument);
        }
        self.send_raw_command(&[0x2E])?;
        let cmd_byte: u8 = match direction {
            ScrollDirectionCh1115::Right => 0x26,
            ScrollDirectionCh1115::Left => 0x27,
        };
        self.send_raw_command(&[cmd_byte, start_page & 0x07, interval as u8, end_page & 0x07])?;
        self.send_raw_command(&[0x2F])
    }

    /// CH1115 §4 — Set additional horizontal scroll column bounds
    pub fn set_scroll_columns(
        &mut self,
        start_column: u8,
        end_column: u8,
    ) -> Result<(), DisplayError> {
        let max_col = self.core.size.width - 1;
        if start_column > max_col || end_column > max_col || end_column < start_column {
            return Err(DisplayError::InvalidArgument);
        }
        self.send_raw_command(&[0x24, start_column, end_column])
    }

    /// CH1115 §6 — Set scroll mode
    pub fn set_scroll_mode(&mut self, mode: ScrollModeCh1115) -> Result<(), DisplayError> {
        let d_bit: u8 = match mode {
            ScrollModeCh1115::Continuous => 0x2C,
            ScrollModeCh1115::SingleScreen => 0x2D,
        };
        self.send_raw_command(&[d_bit])
    }

    /// CH1115 §7 — Stop scrolling
    pub fn stop_scroll(&mut self) -> Result<(), DisplayError> {
        self.send_raw_command(&[0x2E])
    }

    /// CH1115 §7 — Activate scrolling
    pub fn activate_scroll(&mut self) -> Result<(), DisplayError> {
        self.send_raw_command(&[0x2F])
    }

    /// CH1115 §9 — Set display start line (0-63)
    pub fn set_display_start_line(&mut self, line: u8) -> Result<(), DisplayError> {
        self.send_raw_command(&[0x40 | (line & 0x3F)])
    }

    /// CH1115 §16 — Set multiplex ratio (1-64)
    pub fn set_multiplex_ratio(&mut self, ratio: u8) -> Result<(), DisplayError> {
        self.send_raw_command(&[0xA8, ratio.saturating_sub(1) & 0x3F])
    }

    /// CH1115 §21 — Set display offset (0-63)
    pub fn set_display_offset(&mut self, offset: u8) -> Result<(), DisplayError> {
        self.send_raw_command(&[0xD3, offset & 0x3F])
    }

    /// CH1115 §13 — Set SEG pads hardware configuration
    pub fn set_seg_pads_config(&mut self, config: SegPadsConfigCh1115) -> Result<(), DisplayError> {
        let cmd = match config {
            SegPadsConfigCh1115::EvenLeft => 0xA2,
            SegPadsConfigCh1115::EvenRight => 0xA3,
        };
        self.send_raw_command(&[cmd])
    }

    /// CH1115 §23 — Set adaptive power save mode
    pub fn set_adaptive_power_save(&mut self, enable: bool) -> Result<(), DisplayError> {
        if enable { self.send_raw_command(&[0xD7]) } else { self.send_raw_command(&[0xD6]) }
    }
}

// --- Synchronous API (Unbuffered) ---

impl<DI: DisplayInterface> Ch1115<DI, Unbuffered> {
    fn send_raw_command(&mut self, bytes: &[u8]) -> Result<(), DisplayError> {
        self.core.interface.send_command(bytes)
    }

    /// CH1115 — Power ON initialization sequence (unbuffered).
    pub fn init(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        ch1115_init_commands(
            &mut self.core.interface,
            config,
            self.core.size.pages(),
            self.core.size.width,
        )
    }

    /// CH1115 §18 — Turn the display panel ON or OFF (sleep mode)
    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        if on { self.send_raw_command(&[0xAF]) } else { self.send_raw_command(&[0xAE]) }
    }

    /// CH1115 §10 — Set contrast (0-255)
    pub fn set_contrast(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.send_raw_command(&[0x81, contrast])
    }

    /// CH1115 §15 — Invert the display (swap ON/OFF pixels)
    pub fn set_inverse(&mut self, inverse: bool) -> Result<(), DisplayError> {
        if inverse { self.send_raw_command(&[0xA7]) } else { self.send_raw_command(&[0xA6]) }
    }

    /// CH1115 §8 — Set pump output voltage
    pub fn set_pump_voltage(&mut self, voltage: PumpVoltageCh1115) -> Result<(), DisplayError> {
        self.send_raw_command(&[voltage as u8])
    }

    /// CH1115 §11 — Set IREF source and internal resistor value
    pub fn set_iref(
        &mut self,
        source: IrefSourceCh1115,
        resistor: IrefResistorCh1115,
    ) -> Result<(), DisplayError> {
        let d_bit: u8 = match source {
            IrefSourceCh1115::External => 0x00,
            IrefSourceCh1115::Internal => 0x04,
        };
        self.send_raw_command(&[0x82, d_bit | (resistor as u8)])
    }

    /// CH1115 §25 — Set VCOM deselect level
    pub fn set_vcom_deselect(&mut self, level: u8) -> Result<(), DisplayError> {
        self.send_raw_command(&[0xDB, level])
    }

    /// CH1115 §17 — Set DC-DC converter ON or OFF
    pub fn set_dc_dc(&mut self, enable: bool) -> Result<(), DisplayError> {
        self.send_raw_command(&[0xAD, if enable { 0x8B } else { 0x8A }])
    }

    /// CH1115 §3 — Set breathing light effect
    pub fn set_breathing_light(
        &mut self,
        enable: bool,
        brightness: BreathingBrightnessCh1115,
        interval: u8,
    ) -> Result<(), DisplayError> {
        let on_off: u8 = if enable { 0x80 } else { 0x00 };
        self.send_raw_command(&[0x23, on_off | (brightness as u8) | (interval & 0x07)])
    }

    /// CH1115 §5 — Configure and activate horizontal scroll
    pub fn start_horizontal_scroll(
        &mut self,
        direction: ScrollDirectionCh1115,
        start_page: u8,
        end_page: u8,
        interval: ScrollIntervalCh1115,
    ) -> Result<(), DisplayError> {
        let max_page = self.core.size.pages() - 1;
        if start_page > max_page || end_page > max_page || end_page < start_page {
            return Err(DisplayError::InvalidArgument);
        }
        self.send_raw_command(&[0x2E])?;
        let cmd_byte: u8 = match direction {
            ScrollDirectionCh1115::Right => 0x26,
            ScrollDirectionCh1115::Left => 0x27,
        };
        self.send_raw_command(&[cmd_byte, start_page & 0x07, interval as u8, end_page & 0x07])?;
        self.send_raw_command(&[0x2F])
    }

    /// CH1115 §4 — Set additional horizontal scroll column bounds
    pub fn set_scroll_columns(
        &mut self,
        start_column: u8,
        end_column: u8,
    ) -> Result<(), DisplayError> {
        let max_col = self.core.size.width - 1;
        if start_column > max_col || end_column > max_col || end_column < start_column {
            return Err(DisplayError::InvalidArgument);
        }
        self.send_raw_command(&[0x24, start_column, end_column])
    }

    /// CH1115 §6 — Set scroll mode
    pub fn set_scroll_mode(&mut self, mode: ScrollModeCh1115) -> Result<(), DisplayError> {
        let d_bit: u8 = match mode {
            ScrollModeCh1115::Continuous => 0x2C,
            ScrollModeCh1115::SingleScreen => 0x2D,
        };
        self.send_raw_command(&[d_bit])
    }

    /// CH1115 §7 — Stop scrolling
    pub fn stop_scroll(&mut self) -> Result<(), DisplayError> {
        self.send_raw_command(&[0x2E])
    }

    /// CH1115 §7 — Activate scrolling
    pub fn activate_scroll(&mut self) -> Result<(), DisplayError> {
        self.send_raw_command(&[0x2F])
    }

    /// CH1115 §9 — Set display start line (0-63)
    pub fn set_display_start_line(&mut self, line: u8) -> Result<(), DisplayError> {
        self.send_raw_command(&[0x40 | (line & 0x3F)])
    }

    /// CH1115 §16 — Set multiplex ratio (1-64)
    pub fn set_multiplex_ratio(&mut self, ratio: u8) -> Result<(), DisplayError> {
        self.send_raw_command(&[0xA8, ratio.saturating_sub(1) & 0x3F])
    }

    /// CH1115 §21 — Set display offset (0-63)
    pub fn set_display_offset(&mut self, offset: u8) -> Result<(), DisplayError> {
        self.send_raw_command(&[0xD3, offset & 0x3F])
    }

    /// CH1115 §13 — Set SEG pads hardware configuration
    pub fn set_seg_pads_config(&mut self, config: SegPadsConfigCh1115) -> Result<(), DisplayError> {
        let cmd = match config {
            SegPadsConfigCh1115::EvenLeft => 0xA2,
            SegPadsConfigCh1115::EvenRight => 0xA3,
        };
        self.send_raw_command(&[cmd])
    }

    /// CH1115 §23 — Set adaptive power save mode
    pub fn set_adaptive_power_save(&mut self, enable: bool) -> Result<(), DisplayError> {
        if enable { self.send_raw_command(&[0xD7]) } else { self.send_raw_command(&[0xD6]) }
    }
}

// --- Async API (Buffered) ---

impl<DI: AsyncDisplayInterface> Ch1115<DI, Buffered> {
    async fn send_raw_command_async(&mut self, bytes: &[u8]) -> Result<(), DisplayError> {
        self.core.interface.send_command(bytes).await
    }

    /// CH1115 — Async power ON initialization sequence.
    pub async fn init_async(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        self.send_raw_command_async(&[0xAE]).await?;
        self.send_raw_command_async(&[0xD5, 0x50]).await?;
        self.send_raw_command_async(&[0xA8, 0x3F]).await?;
        self.send_raw_command_async(&[0xD3, config.display_offset & 0x3F]).await?;
        self.send_raw_command_async(&[0x40 | (config.display_start_line & 0x3F)]).await?;
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
        self.send_raw_command_async(&[0xA2]).await?;
        self.send_raw_command_async(&[0x81, config.contrast]).await?;
        self.send_raw_command_async(&[0xD9, 0x66]).await?;
        self.send_raw_command_async(&[0xDB, 0x35]).await?;
        self.send_raw_command_async(&[PumpVoltageCh1115::V8_0 as u8]).await?;
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

    /// CH1115 — Async flush of framebuffer to display GDDRAM (page mode).
    pub async fn flush_async(&mut self) -> Result<(), DisplayError> {
        let pages = self.core.size.pages();
        let width = self.core.size.width;
        for page in 0..pages {
            self.send_raw_command_async(&[0xB0 | page]).await?;
            self.send_raw_command_async(&[0x00]).await?;
            self.send_raw_command_async(&[0x10]).await?;
            let start = page as usize * width as usize;
            let end = start + width as usize;
            self.core.interface.send_data(&self.core.buffer[start..end]).await?;
        }
        Ok(())
    }

    /// CH1115 §18 — Async display ON/OFF
    pub async fn display_on_async(&mut self, on: bool) -> Result<(), DisplayError> {
        if on { self.send_raw_command_async(&[0xAF]).await } else { self.send_raw_command_async(&[0xAE]).await }
    }

    /// CH1115 §10 — Async set contrast
    pub async fn set_contrast_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.send_raw_command_async(&[0x81, contrast]).await
    }

    /// CH1115 §15 — Async set inverse
    pub async fn set_inverse_async(&mut self, inverse: bool) -> Result<(), DisplayError> {
        if inverse { self.send_raw_command_async(&[0xA7]).await } else { self.send_raw_command_async(&[0xA6]).await }
    }

    /// CH1115 §8 — Async set pump output voltage
    pub async fn set_pump_voltage_async(
        &mut self,
        voltage: PumpVoltageCh1115,
    ) -> Result<(), DisplayError> {
        self.send_raw_command_async(&[voltage as u8]).await
    }

    /// CH1115 §11 — Async set IREF
    pub async fn set_iref_async(
        &mut self,
        source: IrefSourceCh1115,
        resistor: IrefResistorCh1115,
    ) -> Result<(), DisplayError> {
        let d_bit: u8 = match source {
            IrefSourceCh1115::External => 0x00,
            IrefSourceCh1115::Internal => 0x04,
        };
        self.send_raw_command_async(&[0x82, d_bit | (resistor as u8)]).await
    }
}

// --- Async API (Unbuffered) ---

impl<DI: AsyncDisplayInterface> Ch1115<DI, Unbuffered> {
    async fn send_raw_command_async(&mut self, bytes: &[u8]) -> Result<(), DisplayError> {
        self.core.interface.send_command(bytes).await
    }

    /// CH1115 — Async power ON initialization sequence (unbuffered).
    pub async fn init_async(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        self.send_raw_command_async(&[0xAE]).await?;
        self.send_raw_command_async(&[0xD5, 0x50]).await?;
        self.send_raw_command_async(&[0xA8, 0x3F]).await?;
        self.send_raw_command_async(&[0xD3, config.display_offset & 0x3F]).await?;
        self.send_raw_command_async(&[0x40 | (config.display_start_line & 0x3F)]).await?;
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
        self.send_raw_command_async(&[0xA2]).await?;
        self.send_raw_command_async(&[0x81, config.contrast]).await?;
        self.send_raw_command_async(&[0xD9, 0x66]).await?;
        self.send_raw_command_async(&[0xDB, 0x35]).await?;
        self.send_raw_command_async(&[PumpVoltageCh1115::V8_0 as u8]).await?;
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

    /// CH1115 §18 — Async display ON/OFF
    pub async fn display_on_async(&mut self, on: bool) -> Result<(), DisplayError> {
        if on { self.send_raw_command_async(&[0xAF]).await } else { self.send_raw_command_async(&[0xAE]).await }
    }

    /// CH1115 §10 — Async set contrast
    pub async fn set_contrast_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.send_raw_command_async(&[0x81, contrast]).await
    }

    /// CH1115 §15 — Async set inverse
    pub async fn set_inverse_async(&mut self, inverse: bool) -> Result<(), DisplayError> {
        if inverse { self.send_raw_command_async(&[0xA7]).await } else { self.send_raw_command_async(&[0xA6]).await }
    }

    /// CH1115 §8 — Async set pump output voltage
    pub async fn set_pump_voltage_async(
        &mut self,
        voltage: PumpVoltageCh1115,
    ) -> Result<(), DisplayError> {
        self.send_raw_command_async(&[voltage as u8]).await
    }

    /// CH1115 §11 — Async set IREF
    pub async fn set_iref_async(
        &mut self,
        source: IrefSourceCh1115,
        resistor: IrefResistorCh1115,
    ) -> Result<(), DisplayError> {
        let d_bit: u8 = match source {
            IrefSourceCh1115::External => 0x00,
            IrefSourceCh1115::Internal => 0x04,
        };
        self.send_raw_command_async(&[0x82, d_bit | (resistor as u8)]).await
    }
}

// --- embedded-graphics DrawTarget (Buffered only) ---

impl<DI, MODE: DisplayBufferMode> OriginDimensions for Ch1115<DI, MODE> {
    fn size(&self) -> Size {
        Size::new(self.core.size.width as u32, self.core.size.height as u32)
    }
}

impl<DI> DrawTarget for Ch1115<DI, Buffered> {
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
