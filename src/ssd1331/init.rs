/// SSD1331 display size configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplaySize1331 {
    pub width: u8,
    pub height: u8,
}

impl DisplaySize1331 {
    pub const fn new(width: u8, height: u8) -> Self {
        Self { width, height }
    }
}

/// 96x64 (SSD1331 default)
pub const SIZE_96X64: DisplaySize1331 = DisplaySize1331::new(96, 64);

/// Configuration for SSD1331 display initialization.
///
/// SSD1331 §8 — Command Table
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitConfig {
    /// SSD1331 §9.1.6 — Address increment mode (A0h bit 0).
    /// false = horizontal (RESET), true = vertical.
    pub vertical_address_increment: bool,
    /// SSD1331 §9.1.6 — Column address mapping (A0h bit 1).
    /// true = RAM Column 0 to 95 maps to Pin Seg 95 to 0.
    pub column_remap: bool,
    /// SSD1331 §9.1.6 — Color order (A0h bit 2).
    /// false = normal order SA,SB,SC (RGB), true = reverse SC,SB,SA (BGR).
    pub bgr_order: bool,
    /// SSD1331 §9.1.6 — COM left-right swapping (A0h bit 3).
    pub com_left_right_swap: bool,
    /// SSD1331 §9.1.6 — COM scan direction remap (A0h bit 4).
    /// false = scan from COM0 to COM[N-1], true = scan from COM[N-1] to COM0.
    pub com_scan_reversed: bool,
    /// SSD1331 §9.1.6 — Odd/Even split of COM pins (A0h bit 5).
    pub com_split_odd_even: bool,
    /// SSD1331 §9.1.6 — Display color depth (A0h bits 7:6).
    /// 0 = 256 color, 1 = 65K color format 1 (default), 2 = 65K color format 2.
    pub color_depth: u8,
    /// Whether to turn the display on at the end of initialization.
    pub display_on: bool,
    /// SSD1331 §9.1.7 — Display start line (0-63).
    pub display_start_line: u8,
    /// SSD1331 §9.1.8 — Display offset (vertical shift, 0-63).
    pub display_offset: u8,
    /// SSD1331 §9.1.3 — Contrast for color A (0-255). Default: 0x80.
    pub contrast_a: u8,
    /// SSD1331 §9.1.3 — Contrast for color B (0-255). Default: 0x80.
    pub contrast_b: u8,
    /// SSD1331 §9.1.3 — Contrast for color C (0-255). Default: 0x80.
    pub contrast_c: u8,
    /// SSD1331 §9.1.4 — Master current attenuation (0-15). Default: 15 (no attenuation).
    pub master_current: u8,
    /// SSD1331 §9.1.15 — Phase 1 period (1-15 DCLK). Default: 4.
    pub phase1: u8,
    /// SSD1331 §9.1.15 — Phase 2 period (1-15 DCLK). Default: 7.
    pub phase2: u8,
    /// SSD1331 §9.1.16 — Display clock divide ratio (0-15). Actual divider = value + 1. Default: 0.
    pub clock_divide_ratio: u8,
    /// SSD1331 §9.1.16 — Oscillator frequency (0-15). Default: 0xD.
    pub osc_freq: u8,
    /// SSD1331 §9.1.19 — Pre-charge voltage level (0-31). Default: 0x1E (0.50*VCC).
    pub precharge_voltage: u8,
    /// SSD1331 §9.1.20 — VCOMH deselect level (0-31). Default: 0x1E (0.83*VCC).
    pub vcomh: u8,
    /// SSD1331 §9.1.9 — Display mode.
    /// false = normal, true = inverse.
    pub inverse_display: bool,
}

impl Default for InitConfig {
    fn default() -> Self {
        Self {
            vertical_address_increment: false,
            column_remap: true,
            bgr_order: false,
            com_left_right_swap: false,
            com_scan_reversed: true,
            com_split_odd_even: true,
            color_depth: 1,
            display_on: true,
            display_start_line: 0,
            display_offset: 0,
            contrast_a: 0x80,
            contrast_b: 0x80,
            contrast_c: 0x80,
            master_current: 0x0F,
            phase1: 4,
            phase2: 7,
            clock_divide_ratio: 0,
            osc_freq: 0xD,
            precharge_voltage: 0x1E,
            vcomh: 0x1E,
            inverse_display: false,
        }
    }
}

impl InitConfig {
    /// SSD1331 §9.1.6 — Build the A0h remap & color depth byte.
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
        if self.com_left_right_swap {
            v |= 0x08;
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
