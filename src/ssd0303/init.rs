use crate::common::DisplaySize;

/// 132x64 (SSD0303 default)
pub const DEFAULT_DISPLAY_SIZE: DisplaySize = DisplaySize::new(132, 64);

/// SSD0303 §8 — Horizontal scroll time interval (in frames)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ScrollInterval0303 {
    Frames12 = 0b00,
    Frames64 = 0b01,
    Frames128 = 0b10,
    Frames256 = 0b11,
}

/// Configuration for SSD0303 display initialization.
///
/// SSD0303 §8 — Command Table
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitConfig {
    /// SSD0303 §8 — Segment re-map (A0h/A1h).
    /// false = column address 0 mapped to SEG0 (POR), true = column address 131 mapped to SEG0.
    pub segment_remap: bool,
    /// SSD0303 §8 — COM output scan direction (C0h/C8h).
    /// false = normal (COM0 to COM[N-1]) (POR), true = remapped (COM[N-1] to COM0).
    pub com_scan_reversed: bool,
    /// Whether to turn the display on at the end of initialization.
    pub display_on: bool,
    /// SSD0303 §8 — Display offset (vertical shift, 0-63). Default: 0.
    pub display_offset: u8,
    /// SSD0303 §8 — Display start line (0-63). Default: 0.
    pub display_start_line: u8,
    /// SSD0303 §8 — Contrast level (0-255). Default: 0x80 (POR).
    pub contrast: u8,
    /// SSD0303 §8 — Entire display ON (all pixels lit regardless of RAM). Default: false.
    pub entire_display_on: bool,
    /// SSD0303 §8 — Inverse display. Default: false (normal).
    pub inverse_display: bool,
    /// SSD0303 §8 — Multiplex ratio (16-64). Default: 64 (POR).
    pub multiplex_ratio: u8,
    /// SSD0303 §8 — Display clock divide ratio (0-15). Actual divider = value + 1. Default: 0 (POR).
    pub clock_divide_ratio: u8,
    /// SSD0303 §8 — Oscillator frequency (0-15). Default: 0x7 (POR).
    pub osc_freq: u8,
    /// SSD0303 §8 — Pre-charge phase 1 period (1-15 DCLK). Default: 2 (POR).
    pub phase1: u8,
    /// SSD0303 §8 — Pre-charge phase 2 period (1-15 DCLK). Default: 2 (POR).
    pub phase2: u8,
    /// SSD0303 §8 — COM pins hardware configuration (DAh).
    /// false = sequential, true = alternative (POR).
    pub com_pin_alternative: bool,
    /// SSD0303 §8 — VCOM deselect level (7-bit, 0-127). Default: 0x35 (~0.77*Vref, POR).
    pub vcomh: u8,
    /// SSD0303 §8 — DC-DC enable on display ON (ADh).
    /// true = DC-DC will be turned on when display on (POR), false = DC-DC disabled.
    pub dc_dc_enable: bool,
    /// SSD0303 §8 — Deactivate scroll during init. Default: true.
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
            multiplex_ratio: 64,
            clock_divide_ratio: 0,
            osc_freq: 0x7,
            phase1: 2,
            phase2: 2,
            com_pin_alternative: true,
            vcomh: 0x35,
            dc_dc_enable: true,
            deactivate_scroll: true,
        }
    }
}
