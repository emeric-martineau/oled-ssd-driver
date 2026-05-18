// SSD1325 §9 — Current range selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CurrentRange1325 {
    /// Quarter current range (RESET)
    Quarter = 0x84,
    /// Half current range
    Half = 0x85,
    /// Full current range
    Full = 0x87,
}

/// SSD1325 display size configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplaySize1325 {
    pub width: u8,
    pub height: u8,
}

impl DisplaySize1325 {
    pub const fn new(width: u8, height: u8) -> Self {
        Self { width, height }
    }

    /// Number of column addresses (each column = 2 pixels at 4bpp)
    pub(crate) const fn columns(&self) -> u8 {
        self.width / 2
    }
}

/// 128x80 (SSD1325 default)
pub const SIZE_128X80: DisplaySize1325 = DisplaySize1325::new(128, 80);

/// Configuration for SSD1325 display initialization.
///
/// SSD1325 §9 — Command Table
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitConfig {
    /// SSD1325 §10.1.5 — Column address remapping (A0h bit 0).
    /// true = column addresses are mapped in reverse direction to SEG pins.
    pub column_remap: bool,
    /// SSD1325 §10.1.5 — Nibble remapping (A0h bit 1).
    /// true = high/low nibbles of data bytes are swapped.
    pub nibble_remap: bool,
    /// SSD1325 §10.1.5 — Address increment mode (A0h bit 2).
    /// false = horizontal (RESET), true = vertical.
    pub vertical_address_increment: bool,
    /// SSD1325 §10.1.5 — COM remapping (A0h bit 4).
    /// true = scan from COM[N-1] to COM0.
    pub com_remap: bool,
    /// SSD1325 §10.1.5 — COM split odd/even (A0h bit 6).
    /// true = enable odd/even splitting of COM pins.
    pub com_split_odd_even: bool,
    /// Whether to turn the display on at the end of initialization.
    pub display_on: bool,
    /// SSD1325 §10.1.7 — Display offset (vertical shift), 0-79.
    pub display_offset: u8,
    /// SSD1325 §10.1.6 — Display start line, 0-79.
    pub display_start_line: u8,
    /// SSD1325 §10.1.3 — Contrast level 0-127. Default: 0x40.
    pub contrast: u8,
    /// SSD1325 §10.1.4 — Current range. Default: Quarter.
    pub current_range: CurrentRange1325,
    /// SSD1325 §10.1.16 — Display clock divide ratio (0-15). Actual divider = value + 1. Default: 1 (divide by 2).
    pub clock_divide_ratio: u8,
    /// SSD1325 §10.1.16 — Oscillator frequency (0-15). Default: 0x4.
    pub osc_freq: u8,
    /// SSD1325 §10.1.14 — Pre-charge phase 1 period (1-15 DCLK). Default: 3.
    pub phase1: u8,
    /// SSD1325 §10.1.14 — Pre-charge phase 2 period (1-15 DCLK). Default: 5.
    pub phase2: u8,
    /// SSD1325 §10.1.12 — VCOMH deselect level (5-bit value). Default: 0x11 (0.68*VREF).
    pub vcomh: u8,
    /// SSD1325 §10.1.13 — Precharge voltage (8-bit value). Default: 0x18 (0.75*VREF).
    pub precharge_voltage: u8,
    /// SSD1325 §10.1.8 — Entire display ON (all pixels lit at GS15). Default: false.
    pub entire_display_on: bool,
    /// SSD1325 §10.1.8 — Inverse display. Default: false.
    pub inverse_display: bool,
}

impl Default for InitConfig {
    fn default() -> Self {
        Self {
            column_remap: false,
            nibble_remap: false,
            vertical_address_increment: false,
            com_remap: false,
            com_split_odd_even: false,
            display_on: true,
            display_offset: 0,
            display_start_line: 0,
            contrast: 0x40,
            current_range: CurrentRange1325::Quarter,
            clock_divide_ratio: 1,
            osc_freq: 0x4,
            phase1: 3,
            phase2: 5,
            vcomh: 0x11,
            precharge_voltage: 0x18,
            entire_display_on: false,
            inverse_display: false,
        }
    }
}

impl InitConfig {
    /// SSD1325 §10.1.5 — Build the A0h Re-map byte from config flags.
    pub(crate) fn remap_byte(&self) -> u8 {
        let mut v = 0u8;
        if self.column_remap {
            v |= 0x01;
        }
        if self.nibble_remap {
            v |= 0x02;
        }
        if self.vertical_address_increment {
            v |= 0x04;
        }
        if self.com_remap {
            v |= 0x10;
        }
        if self.com_split_odd_even {
            v |= 0x40;
        }
        v
    }
}
