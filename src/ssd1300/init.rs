use crate::common::DisplaySize;

/// 104x48 (SSD1300 default)
pub const DEFAULT_DISPLAY_SIZE: DisplaySize = DisplaySize::new(104, 48);

/// SSD1300 §8 — Horizontal scroll time interval (in frames)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ScrollInterval1300 {
    Frames12 = 0b00,
    Frames64 = 0b01,
    Frames128 = 0b10,
    Frames256 = 0b11,
}

/// Configuration for SSD1300 display initialization.
///
/// SSD1300 §8 — Command Table
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitConfig {
    /// SSD1300 §8 — Segment re-map (A0h/A1h).
    /// false = column address 0 mapped to SEG0 (POR), true = column address 103 mapped to SEG0.
    pub segment_remap: bool,
    /// SSD1300 §8 — COM output scan direction (C0h/C8h).
    /// false = normal (COM0 to COM[N-1]) (POR), true = remapped (COM[N-1] to COM0).
    pub com_scan_reversed: bool,
    /// Whether to turn the display on at the end of initialization.
    pub display_on: bool,
    /// SSD1300 §8 — Display offset (vertical shift, 0-47). Default: 0.
    pub display_offset: u8,
    /// SSD1300 §8 — Display start line (0-47). Default: 0.
    pub display_start_line: u8,
    /// SSD1300 §8 — Contrast level (0-255). Default: 0x80 (POR).
    pub contrast: u8,
    /// SSD1300 §8 — Entire display ON (all pixels lit regardless of RAM). Default: false.
    pub entire_display_on: bool,
    /// SSD1300 §8 — Inverse display. Default: false (normal).
    pub inverse_display: bool,
    /// SSD1300 §8 — Multiplex ratio (16-48). Default: 48 (POR).
    pub multiplex_ratio: u8,
    /// SSD1300 §8 — Display clock divide ratio (0-15). Actual divider = value + 1. Default: 0 (POR).
    pub clock_divide_ratio: u8,
    /// SSD1300 §8 — Oscillator frequency (0-15). Default: 0x8.
    pub osc_freq: u8,
    /// SSD1300 §8 — Pre-charge phase 1 period (1-15 DCLK). Default: 2 (POR).
    pub phase1: u8,
    /// SSD1300 §8 — Pre-charge phase 2 period (1-15 DCLK). Default: 2 (POR).
    pub phase2: u8,
    /// SSD1300 §8 — COM pins hardware configuration (DAh).
    /// false = sequential, true = alternative (POR).
    pub com_pin_alternative: bool,
    /// SSD1300 §8 — VCOMH deselect level (7-bit, 0-127). Default: 0x3F (0.83*Vref, POR).
    pub vcomh: u8,
    /// SSD1300 §8 — DC-DC enable on display ON (ADh).
    /// X0: false = DC-DC disabled, true = DC-DC enabled (POR).
    pub dc_dc_enable: bool,
    /// SSD1300 §8 — DC-DC (ADh): VCOMH source.
    /// X1: false = external VCOMH, true = internal VCOMH regulator (POR).
    pub internal_vcomh: bool,
    /// SSD1300 §8 — Deactivate scroll during init. Default: true.
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
            contrast: 0x80,
            entire_display_on: false,
            inverse_display: false,
            multiplex_ratio: 48,
            clock_divide_ratio: 0,
            osc_freq: 0x8,
            phase1: 2,
            phase2: 2,
            com_pin_alternative: true,
            vcomh: 0x3F,
            dc_dc_enable: true,
            internal_vcomh: true,
            deactivate_scroll: true,
        }
    }
}
