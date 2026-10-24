use crate::common::DisplaySize;

/// 128x128 (SSD1327 default)
pub const DEFAULT_DISPLAY_SIZE: DisplaySize = DisplaySize::new(128, 128);

/// Configuration for SSD1327 display initialization.
///
/// SSD1327 §8 — Command Table
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitConfig {
    /// SSD1327 §9.1.5 — Column address remapping (A0h bit 0).
    /// true = column addresses are mapped in reverse direction to SEG pins.
    pub column_remap: bool,
    /// SSD1327 §9.1.5 — Nibble remapping (A0h bit 1).
    /// true = high/low nibbles of data bytes are swapped.
    pub nibble_remap: bool,
    /// SSD1327 §9.1.5 — Address increment mode (A0h bit 2).
    /// false = horizontal (RESET), true = vertical.
    pub vertical_address_increment: bool,
    /// SSD1327 §9.1.5 — COM remapping (A0h bit 4).
    /// true = scan from COM[N-1] to COM0.
    pub com_remap: bool,
    /// SSD1327 §9.1.5 — COM split odd/even (A0h bit 6).
    /// true = enable odd/even splitting of COM pins.
    pub com_split_odd_even: bool,
    /// Whether to turn the display on at the end of initialization.
    pub display_on: bool,
    /// SSD1327 §9.1.7 — Display offset (vertical shift), 0-127.
    pub display_offset: u8,
    /// SSD1327 §9.1.6 — Display start line, 0-127.
    pub display_start_line: u8,
    /// SSD1327 §9.1.3 — Contrast level 0-255. Default: 0x7F.
    pub contrast: u8,
    /// SSD1327 §9.1.10 — Function selection A: enable internal VDD regulator. Default: true (RESET).
    pub internal_vdd: bool,
    /// SSD1327 §9.1.14 — Display clock divide ratio (0-15). Actual divider = value + 1. Default: 0.
    pub clock_divide_ratio: u8,
    /// SSD1327 §9.1.14 — Oscillator frequency (0-15). Default: 0.
    pub osc_freq: u8,
    /// SSD1327 §9.1.12 — Pre-charge phase 1 period (1-15 DCLK). Default: 4.
    pub phase1: u8,
    /// SSD1327 §9.1.12 — Pre-charge phase 2 period (1-15 DCLK). Default: 7.
    pub phase2: u8,
    /// SSD1327 §9.1.21 — VCOMH deselect level (3-bit value, 0-7). Default: 0x05 (0.82*VCC).
    pub vcomh: u8,
    /// SSD1327 §9.1.20 — Precharge voltage (4-bit value, 0-8). Default: 0x05 (0.5*VCC).
    pub precharge_voltage: u8,
    /// SSD1327 §9.1.16 — Second pre-charge period (0-15 DCLK). Default: 0x04.
    pub second_precharge_period: u8,
    /// SSD1327 §9.1.22 — Function selection B: enable second precharge. Default: false.
    pub second_precharge_enable: bool,
    /// SSD1327 §9.1.22 — Function selection B: external VSL. Default: false (internal VSL).
    pub external_vsl: bool,
    /// SSD1327 §9.1.8 — Entire display ON (all pixels lit at GS15). Default: false.
    pub entire_display_on: bool,
    /// SSD1327 §9.1.8 — Inverse display. Default: false.
    pub inverse_display: bool,
}

impl Default for InitConfig {
    fn default() -> Self {
        Self {
            column_remap: true,
            nibble_remap: false,
            vertical_address_increment: false,
            com_remap: true,
            com_split_odd_even: true,
            display_on: true,
            display_offset: 0,
            display_start_line: 0,
            contrast: 0x7F,
            internal_vdd: true,
            clock_divide_ratio: 0,
            osc_freq: 0,
            phase1: 4,
            phase2: 7,
            vcomh: 0x05,
            precharge_voltage: 0x05,
            second_precharge_period: 0x04,
            second_precharge_enable: false,
            external_vsl: false,
            entire_display_on: false,
            inverse_display: false,
        }
    }
}

impl InitConfig {
    /// SSD1327 §9.1.5 — Build the A0h Re-map byte from config flags.
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

    /// SSD1327 §9.1.22 — Build the D5h Function Selection B byte.
    pub(crate) fn function_b_byte(&self) -> u8 {
        let mut v = 0u8;
        if self.external_vsl {
            v |= 0x01;
        }
        if self.second_precharge_enable {
            v |= 0x02;
        }
        v
    }
}
