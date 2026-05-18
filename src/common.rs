use crate::error::DisplayError;
use crate::interface::{AsyncDisplayInterface, DisplayInterface};
use crate::ssd1306::command::{AddressingMode, ComPinConfig, Command, VcomhLevel};

pub(crate) const MAX_WIDTH: u8 = 132;
pub(crate) const MAX_PAGES: u8 = 8;
pub(crate) const BUFFER_SIZE: usize = MAX_WIDTH as usize * MAX_PAGES as usize;

/// Display dimensions configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplaySize {
    pub width: u8,
    pub height: u8,
}

impl DisplaySize {
    pub const fn new(width: u8, height: u8) -> Self {
        Self { width, height }
    }

    pub(crate) const fn pages(&self) -> u8 {
        self.height.div_ceil(8)
    }
}

/// 132x64 (CH1116 default)
pub const SIZE_132X64: DisplaySize = DisplaySize::new(132, 64);
/// 128x64 (default)
pub const SIZE_128X64: DisplaySize = DisplaySize::new(128, 64);
/// 128x39 (SSD1307 default)
pub const SIZE_128X39: DisplaySize = DisplaySize::new(128, 39);
/// 128x32
pub const SIZE_128X32: DisplaySize = DisplaySize::new(128, 32);
/// 96x16
pub const SIZE_96X16: DisplaySize = DisplaySize::new(96, 16);
/// 64x48
pub const SIZE_64X48: DisplaySize = DisplaySize::new(64, 48);

/// Configuration for display initialization.
///
/// Controls orientation, addressing, contrast, and whether the display is turned on after init.
/// Fields using `Option<T>` are only sent if `Some` — use `None` to skip commands not supported
/// by a particular controller (e.g., CH1116 does not support addressing mode or VCOMH enum).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitConfig {
    /// Segment re-map: true = column address mapped in reverse direction to SEG pins.
    /// SSD1306/B/1315/CH1116 default: true. SSD1312 default: false.
    pub segment_remap: bool,
    /// COM output scan direction: true = scan from COM[N-1] to COM0 (vertically flipped).
    /// SSD1306/B/1315/CH1116 default: true. SSD1312 default: false.
    pub com_scan_reversed: bool,
    /// Whether to turn the display on at the end of initialization.
    pub display_on: bool,
    /// Display offset (vertical shift), 0-63. Default: 0.
    pub display_offset: u8,
    /// Display start line, 0-63. Default: 0.
    pub display_start_line: u8,
    /// Memory addressing mode. `Some(Horizontal)` for SSD13xx, `None` to skip (CH1116 uses page mode only).
    pub addressing_mode: Option<AddressingMode>,
    /// Contrast level 0-255. Default: 0x7F.
    pub contrast: u8,
    /// VCOMH deselect level for SSD13xx controllers. `None` to skip (CH1116 uses raw u8 via its own API).
    pub vcomh_deselect: Option<VcomhLevel>,
    /// Entire display ON (all pixels lit regardless of RAM). Default: false.
    pub entire_display_on: bool,
    /// Inverse display (swap ON/OFF pixels). Default: false.
    pub inverse_display: bool,
    /// Whether to deactivate scroll during init. Default: true.
    pub deactivate_scroll: bool,
}

impl Default for InitConfig {
    fn default() -> Self {
        Self {
            segment_remap: true,
            com_scan_reversed: true,
            display_on: true,
            display_offset: 0,
            display_start_line: 0,
            addressing_mode: Some(AddressingMode::Horizontal),
            contrast: 0x7F,
            vcomh_deselect: Some(VcomhLevel::V077),
            entire_display_on: false,
            inverse_display: false,
            deactivate_scroll: true,
        }
    }
}

pub(crate) struct DisplayCore<DI> {
    pub interface: DI,
    pub buffer: [u8; BUFFER_SIZE],
    pub size: DisplaySize,
}

impl<DI> DisplayCore<DI> {
    pub fn new(interface: DI, size: DisplaySize) -> Self {
        Self {
            interface,
            buffer: [0u8; BUFFER_SIZE],
            size,
        }
    }

    pub fn set_pixel(&mut self, x: u8, y: u8, on: bool) {
        if x >= self.size.width || y >= self.size.height {
            return;
        }
        let page = (y / 8) as usize;
        let bit = y % 8;
        let idx = page * self.size.width as usize + x as usize;
        if on {
            self.buffer[idx] |= 1 << bit;
        } else {
            self.buffer[idx] &= !(1 << bit);
        }
    }

    pub fn clear_buffer(&mut self) {
        let len = self.size.width as usize * self.size.pages() as usize;
        self.buffer[..len].fill(0);
    }

    pub fn active_buffer(&self) -> &[u8] {
        let len = self.size.width as usize * self.size.pages() as usize;
        &self.buffer[..len]
    }
}

impl<DI: DisplayInterface> DisplayCore<DI> {
    pub fn send_command(&mut self, cmd: Command) -> Result<(), DisplayError> {
        let (bytes, len) = cmd.bytes();
        self.interface.send_command(&bytes[..len])
    }

    pub fn flush(&mut self) -> Result<(), DisplayError> {
        self.send_command(Command::SetColumnAddress {
            start: 0,
            end: self.size.width - 1,
        })?;
        self.send_command(Command::SetPageAddress {
            start: 0,
            end: self.size.pages() - 1,
        })?;
        let len = self.size.width as usize * self.size.pages() as usize;
        self.interface.send_data(&self.buffer[..len])
    }

    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.send_command(Command::DisplayOn(on))
    }

    pub fn set_contrast(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.send_command(Command::SetContrast(contrast))
    }

    pub fn set_inverse(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.send_command(Command::InverseDisplay(inverse))
    }

    pub fn stop_scroll(&mut self) -> Result<(), DisplayError> {
        self.send_command(Command::DeactivateScroll)
    }

    pub fn init_common(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        let com_pin_config = if self.size.height <= 32 {
            ComPinConfig {
                alternative: false,
                left_right_remap: false,
            }
        } else {
            ComPinConfig::default()
        };

        self.send_command(Command::DisplayOn(false))?;
        self.send_command(Command::SetDisplayClock {
            divide_ratio: 0,
            osc_freq: 0x8,
        })?;
        self.send_command(Command::SetMultiplexRatio(self.size.height - 1))?;
        self.send_command(Command::SetDisplayOffset(config.display_offset))?;
        self.send_command(Command::SetDisplayStartLine(config.display_start_line))?;
        if let Some(mode) = config.addressing_mode {
            self.send_command(Command::SetAddressingMode(mode))?;
        }
        self.send_command(Command::SetSegmentRemap(config.segment_remap))?;
        self.send_command(Command::SetComScanDirection(config.com_scan_reversed))?;
        self.send_command(Command::SetComPinConfig(com_pin_config))?;
        self.send_command(Command::SetContrast(config.contrast))?;
        self.send_command(Command::SetPreChargePeriod {
            phase1: 0x1,
            phase2: 0xF,
        })?;
        if let Some(level) = config.vcomh_deselect {
            self.send_command(Command::SetVcomhDeselect(level))?;
        }
        self.send_command(Command::EntireDisplayOn(config.entire_display_on))?;
        self.send_command(Command::InverseDisplay(config.inverse_display))?;
        if config.deactivate_scroll {
            self.send_command(Command::DeactivateScroll)?;
        }
        Ok(())
    }
}

impl<DI: AsyncDisplayInterface> DisplayCore<DI> {
    pub async fn send_command_async(&mut self, cmd: Command) -> Result<(), DisplayError> {
        let (bytes, len) = cmd.bytes();
        self.interface.send_command(&bytes[..len]).await
    }

    pub async fn flush_async(&mut self) -> Result<(), DisplayError> {
        self.send_command_async(Command::SetColumnAddress {
            start: 0,
            end: self.size.width - 1,
        })
        .await?;
        self.send_command_async(Command::SetPageAddress {
            start: 0,
            end: self.size.pages() - 1,
        })
        .await?;
        let len = self.size.width as usize * self.size.pages() as usize;
        self.interface.send_data(&self.buffer[..len]).await
    }

    pub async fn display_on_async(&mut self, on: bool) -> Result<(), DisplayError> {
        self.send_command_async(Command::DisplayOn(on)).await
    }

    pub async fn set_contrast_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.send_command_async(Command::SetContrast(contrast))
            .await
    }

    pub async fn set_inverse_async(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.send_command_async(Command::InverseDisplay(inverse))
            .await
    }

    pub async fn init_common_async(&mut self, config: &InitConfig) -> Result<(), DisplayError> {
        let com_pin_config = if self.size.height <= 32 {
            ComPinConfig {
                alternative: false,
                left_right_remap: false,
            }
        } else {
            ComPinConfig::default()
        };

        self.send_command_async(Command::DisplayOn(false)).await?;
        self.send_command_async(Command::SetDisplayClock {
            divide_ratio: 0,
            osc_freq: 0x8,
        })
        .await?;
        self.send_command_async(Command::SetMultiplexRatio(self.size.height - 1))
            .await?;
        self.send_command_async(Command::SetDisplayOffset(config.display_offset))
            .await?;
        self.send_command_async(Command::SetDisplayStartLine(config.display_start_line))
            .await?;
        if let Some(mode) = config.addressing_mode {
            self.send_command_async(Command::SetAddressingMode(mode))
                .await?;
        }
        self.send_command_async(Command::SetSegmentRemap(config.segment_remap))
            .await?;
        self.send_command_async(Command::SetComScanDirection(config.com_scan_reversed))
            .await?;
        self.send_command_async(Command::SetComPinConfig(com_pin_config))
            .await?;
        self.send_command_async(Command::SetContrast(config.contrast))
            .await?;
        self.send_command_async(Command::SetPreChargePeriod {
            phase1: 0x1,
            phase2: 0xF,
        })
        .await?;
        if let Some(level) = config.vcomh_deselect {
            self.send_command_async(Command::SetVcomhDeselect(level))
                .await?;
        }
        self.send_command_async(Command::EntireDisplayOn(config.entire_display_on))
            .await?;
        self.send_command_async(Command::InverseDisplay(config.inverse_display))
            .await?;
        if config.deactivate_scroll {
            self.send_command_async(Command::DeactivateScroll).await?;
        }
        Ok(())
    }
}
