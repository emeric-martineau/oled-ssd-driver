/// SSD1362 display size configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplaySize1362 {
    pub width: u16,
    pub height: u8,
}

impl DisplaySize1362 {
    pub const fn new(width: u16, height: u8) -> Self {
        Self { width, height }
    }

    /// Number of column addresses (each column = 2 pixels at 4bpp)
    pub(crate) const fn columns(&self) -> u8 {
        (self.width / 2) as u8
    }
}

/// 256x64 (SSD1362 default)
pub const SIZE_256X64: DisplaySize1362 = DisplaySize1362::new(256, 64);

/// Configuration for SSD1362 display initialization.
///
/// SSD1362 §8 — Command Table
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitConfig {
    /// SSD1362 §9.1.4 — Column address remapping (A0h bit 0).
    /// true = column addresses are mapped in reverse direction to SEG pins.
    pub column_remap: bool,
    /// SSD1362 §9.1.4 — Nibble remapping (A0h bit 1).
    /// true = high/low nibbles of data bytes are swapped.
    pub nibble_remap: bool,
    /// SSD1362 §9.1.4 — Address increment mode (A0h bit 2).
    /// false = horizontal (RESET), true = vertical.
    pub vertical_address_increment: bool,
    /// SSD1362 §9.1.4 — COM remapping (A0h bit 4).
    /// true = scan from COM[N-1] to COM0.
    pub com_remap: bool,
    /// SSD1362 §9.1.4 — SEG split odd/even (A0h bit 6).
    /// true = enable odd/even splitting of SEG pins.
    pub seg_split_odd_even: bool,
    /// SSD1362 §9.1.4 — SEG left/right remap (A0h bit 7).
    /// true = enable left/right SEG remapping.
    pub seg_left_right_remap: bool,
    /// Whether to turn the display on at the end of initialization.
    pub display_on: bool,
    /// SSD1362 §9.1.6 — Display offset (vertical shift), 0-63.
    pub display_offset: u8,
    /// SSD1362 §9.1.5 — Display start line, 0-63.
    pub display_start_line: u8,
    /// SSD1362 §9.1.3 — Contrast level 0-255. Default: 0x7F.
    pub contrast: u8,
    /// SSD1362 §9.1.10 — Function selection A: enable internal VDD regulator. Default: true (RESET).
    pub internal_vdd: bool,
    /// SSD1362 §9.1.14 — Display clock divide ratio (0-15). Actual divider = value + 1. Default: 1.
    pub clock_divide_ratio: u8,
    /// SSD1362 §9.1.14 — Oscillator frequency (0-15). Default: 0xA.
    pub osc_freq: u8,
    /// SSD1362 §9.1.13 — Pre-charge phase 1 period (1-15 DCLK). Default: 1.
    pub phase1: u8,
    /// SSD1362 §9.1.13 — Pre-charge phase 2 period (1-15 DCLK). Default: 8.
    pub phase2: u8,
    /// SSD1362 §9.1.21 — VCOMH deselect level (4-bit value, 0-7). Default: 0x05 (0.82*VCC).
    pub vcomh: u8,
    /// SSD1362 §9.1.19 — Precharge voltage (5-bit value, 0-31). Default: 0x04 (0.15*VCC).
    pub precharge_voltage: u8,
    /// SSD1362 §9.1.16 — Second pre-charge period (0-15 DCLK). Default: 0x04.
    pub second_precharge_period: u8,
    /// SSD1362 §9.1.11 — External or internal IREF selection.
    /// true = internal IREF during display ON, false = external IREF (RESET).
    pub internal_iref: bool,
    /// SSD1362 §9.1.8 — Entire display ON (all pixels lit at GS15). Default: false.
    pub entire_display_on: bool,
    /// SSD1362 §9.1.8 — Inverse display. Default: false.
    pub inverse_display: bool,
}

impl Default for InitConfig {
    fn default() -> Self {
        Self {
            column_remap: false,
            nibble_remap: false,
            vertical_address_increment: false,
            com_remap: false,
            seg_split_odd_even: true,
            seg_left_right_remap: false,
            display_on: true,
            display_offset: 0,
            display_start_line: 0,
            contrast: 0x7F,
            internal_vdd: true,
            clock_divide_ratio: 1,
            osc_freq: 0x0A,
            phase1: 1,
            phase2: 8,
            vcomh: 0x05,
            precharge_voltage: 0x04,
            second_precharge_period: 0x04,
            internal_iref: false,
            entire_display_on: false,
            inverse_display: false,
        }
    }
}

impl InitConfig {
    /// SSD1362 §9.1.4 — Build the A0h Re-map byte from config flags.
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
        if self.seg_split_odd_even {
            v |= 0x40;
        }
        if self.seg_left_right_remap {
            v |= 0x80;
        }
        v
    }
}
