use crate::common::DisplaySize;

/// 128x128 (SSD1351 default)
pub const SIZE_128X128: DisplaySize = DisplaySize::new(128, 128);

/// 128x96 (common module variant)
pub const SIZE_128X96: DisplaySize = DisplaySize::new(128, 96);

/// Configuration for SSD1351 display initialization.
///
/// SSD1351 §9 — Command Table
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitConfig {
    /// SSD1351 §10.1.5 — Address increment mode (A0h bit 0).
    /// false = horizontal (RESET), true = vertical.
    pub vertical_address_increment: bool,
    /// SSD1351 §10.1.5 — Column address remap (A0h bit 1).
    /// true = RAM Column 127 maps to SEG0.
    pub column_remap: bool,
    /// SSD1351 §10.1.5 — Color sequence swap (A0h bit 2).
    /// false = A→B→C (RGB), true = C→B→A (BGR).
    pub bgr_order: bool,
    /// SSD1351 §10.1.5 — COM scan direction remap (A0h bit 4).
    /// false = scan from COM0 to COM[N-1], true = scan from COM[N-1] to COM0.
    pub com_scan_reversed: bool,
    /// SSD1351 §10.1.5 — Odd/Even split of COM pins (A0h bit 5).
    pub com_split_odd_even: bool,
    /// SSD1351 §10.1.5 — Color depth (A0h bits 7:6).
    /// 0 = 65K color (RESET), 1 = 262K color, 2 = 262K color format 2.
    pub color_depth: u8,
    /// Whether to turn the display on at the end of initialization.
    pub display_on: bool,
    /// SSD1351 §10.1.6 — Display start line (0-127).
    pub display_start_line: u8,
    /// SSD1351 §10.1.7 — Display offset (vertical shift, 0-127).
    pub display_offset: u8,
    /// SSD1351 §10.1.20 — Contrast for color A (0-255). Default: 0x8A.
    pub contrast_a: u8,
    /// SSD1351 §10.1.20 — Contrast for color B (0-255). Default: 0x51.
    pub contrast_b: u8,
    /// SSD1351 §10.1.20 — Contrast for color C (0-255). Default: 0x8A.
    pub contrast_c: u8,
    /// SSD1351 §10.1.21 — Master contrast current control (0-15). Default: 15 (no reduction).
    pub master_contrast: u8,
    /// SSD1351 §10.1.11 — Phase 1 period (5-31 DCLKs in units of 2). Default: 2 (=5 DCLKs).
    pub phase1: u8,
    /// SSD1351 §10.1.11 — Phase 2 period (3-15 DCLKs). Default: 8.
    pub phase2: u8,
    /// SSD1351 §10.1.13 — Front clock divider (0-10). Actual divider = 2^value. Default: 1.
    pub clock_divider: u8,
    /// SSD1351 §10.1.13 — Oscillator frequency (0-15). Default: 0xD.
    pub osc_freq: u8,
    /// SSD1351 §10.1.15 — Second pre-charge period (1-15 DCLKs). Default: 8.
    pub second_precharge: u8,
    /// SSD1351 §10.1.18 — Pre-charge voltage level (0-31). Default: 0x17 (0.50*VCC).
    pub precharge_voltage: u8,
    /// SSD1351 §10.1.19 — VCOMH deselect level (0-7). Default: 0x05 (0.82*VCC).
    pub vcomh: u8,
    /// SSD1351 §10.1.8 — Display mode.
    /// false = normal, true = inverse.
    pub inverse_display: bool,
    /// SSD1351 §10.1.9 — Function selection: enable internal VDD regulator.
    /// Default: true (enabled).
    pub internal_vdd: bool,
}

impl Default for InitConfig {
    fn default() -> Self {
        Self {
            vertical_address_increment: false,
            column_remap: true,
            bgr_order: false,
            com_scan_reversed: true,
            com_split_odd_even: true,
            color_depth: 0,
            display_on: true,
            display_start_line: 0,
            display_offset: 96,
            contrast_a: 0x8A,
            contrast_b: 0x51,
            contrast_c: 0x8A,
            master_contrast: 0x0F,
            phase1: 2,
            phase2: 8,
            clock_divider: 1,
            osc_freq: 0xF,
            second_precharge: 8,
            precharge_voltage: 0x17,
            vcomh: 0x05,
            inverse_display: false,
            internal_vdd: true,
        }
    }
}

impl InitConfig {
    /// SSD1351 §10.1.5 — Build the A0h remap & color depth byte.
    pub(crate) fn remap_byte(&self) -> u8 {
        let mut v = 0u8;
        if self.vertical_address_increment {
            v |= 0x01;
        }
        if self.column_remap {
            v |= 0x02;
        }
        if self.bgr_order {
            v |= 0x04;
        }
        if self.com_scan_reversed {
            v |= 0x10;
        }
        if self.com_split_odd_even {
            v |= 0x20;
        }
        v |= (self.color_depth & 0x03) << 6;
        v
    }
}
