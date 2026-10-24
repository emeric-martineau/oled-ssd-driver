use crate::common::DisplaySize;

/// 96x64 (SSD1332 default)
pub const SIZE_96X64: DisplaySize = DisplaySize::new(96, 64);

/// Configuration for SSD1332 display initialization.
///
/// SSD1332 §8 — Command Table
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitConfig {
    /// SSD1332 §9 — Address increment mode (A0h bit 0).
    /// false = horizontal (RESET), true = vertical.
    pub vertical_address_increment: bool,
    /// SSD1332 §9 — Column address mapping (A0h bit 1).
    /// true = RAM Column 0 to 95 maps to Pin Seg 95 to 0.
    pub column_remap: bool,
    /// SSD1332 §9 — COM scan direction remap (A0h bit 4).
    /// false = scan from COM0 to COM[N-1], true = scan from COM[N-1] to COM0.
    pub com_scan_reversed: bool,
    /// SSD1332 §9 — Odd/Even split of COM pins (A0h bit 5).
    pub com_split_odd_even: bool,
    /// SSD1332 §9 — Display color depth (A0h bits 7:6).
    /// 0 = 256 color, 1 = 65K color (RESET).
    pub color_depth: u8,
    /// Whether to turn the display on at the end of initialization.
    pub display_on: bool,
    /// SSD1332 §9 — Display start line (0-63).
    pub display_start_line: u8,
    /// SSD1332 §9 — Display offset (vertical shift, 0-63).
    pub display_offset: u8,
    /// SSD1332 §9 — Contrast for color A (0-255). Default: 0x80.
    pub contrast_a: u8,
    /// SSD1332 §9 — Contrast for color B (0-255). Default: 0x80.
    pub contrast_b: u8,
    /// SSD1332 §9 — Contrast for color C (0-255). Default: 0x80.
    pub contrast_c: u8,
    /// SSD1332 §9 — Master current attenuation (0-15). Default: 15 (no attenuation).
    pub master_current: u8,
    /// SSD1332 §9 — Phase 1 period (1-16 DCLK). Default: 4.
    pub phase1: u8,
    /// SSD1332 §9 — Phase 2 period (1-16 DCLK). Default: 7.
    pub phase2: u8,
    /// SSD1332 §9 — Display clock divide ratio (0-15). Actual divider = value + 1. Default: 0.
    pub clock_divide_ratio: u8,
    /// SSD1332 §9 — Oscillator frequency (0-15). Default: 0xD.
    pub osc_freq: u8,
    /// SSD1332 §9 — VPA pre-charge voltage level for color A (0-255). Default: connects to VCOMH.
    pub vpa: u8,
    /// SSD1332 §9 — VPB pre-charge voltage level for color B (0-255). Default: connects to VCOMH.
    pub vpb: u8,
    /// SSD1332 §9 — VPC pre-charge voltage level for color C (0-255). Default: connects to VCOMH.
    pub vpc: u8,
    /// SSD1332 §9 — VCOMH deselect level (0-63). Default: 0x3F (0.83*VREF).
    pub vcomh: u8,
    /// SSD1332 §9 — Display mode.
    /// false = normal, true = inverse.
    pub inverse_display: bool,
    /// SSD1332 §9 — Master Configuration (ADh).
    /// `A[0]`: false = external VCC, true = internal booster (RESET).
    pub internal_booster: bool,
    /// SSD1332 §9 — Master Configuration (ADh).
    /// `A[2]`: false = external VP supply, true = internal VP (RESET).
    pub internal_vp: bool,
}

impl Default for InitConfig {
    fn default() -> Self {
        Self {
            vertical_address_increment: false,
            column_remap: true,
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
            vpa: 0xFF,
            vpb: 0xFF,
            vpc: 0xFF,
            vcomh: 0x3F,
            inverse_display: false,
            internal_booster: true,
            internal_vp: true,
        }
    }
}

impl InitConfig {
    /// SSD1332 §9 — Build the A0h remap & color depth byte.
    pub(crate) fn remap_byte(&self) -> u8 {
        let mut v = 0u8;
        if self.vertical_address_increment {
            v |= 0x01;
        }
        if self.column_remap {
            v |= 0x02;
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

    /// SSD1332 §9 — Build the ADh master configuration byte.
    pub(crate) fn master_config_byte(&self) -> u8 {
        let mut v = 0u8;
        if self.internal_booster {
            v |= 0x01;
        }
        if self.internal_vp {
            v |= 0x04;
        }
        v
    }
}
