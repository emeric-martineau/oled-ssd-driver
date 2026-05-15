/// SSD1320 §6.7 — Memory addressing mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AddressingMode1320 {
    Horizontal = 0x00,
    Vertical = 0x01,
}

/// SSD1320 §6.8 — IREF selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IrefSetting1320 {
    /// External IREF (RESET)
    External = 0x00,
    /// Internal IREF enabled during display ON
    Internal = 0x10,
}

/// SSD1320 display size configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplaySize1320 {
    pub width: u8,
    pub height: u8,
}

impl DisplaySize1320 {
    pub const fn new(width: u8, height: u8) -> Self {
        Self { width, height }
    }

    /// Number of column addresses (each column = 2 pixels at 4bpp)
    pub(crate) const fn columns(&self) -> u8 {
        self.width / 2
    }
}

/// 160x160 (SSD1320 default)
pub const SIZE_160X160: DisplaySize1320 = DisplaySize1320::new(160, 160);

/// Configuration for SSD1320 display initialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitConfig1320 {
    /// Column address re-map: true = column address mapped in reverse direction.
    pub column_remap: bool,
    /// COM output scan direction: true = remapped (scan from COM[N-1] to COM0).
    pub com_remap: bool,
    /// Whether to turn the display on at the end of initialization.
    pub display_on: bool,
    /// Display offset (vertical shift), 0-159.
    pub display_offset: u8,
    /// Memory addressing mode.
    pub addressing_mode: AddressingMode1320,
    /// Contrast level 0-255. Default: 0x7F.
    pub contrast: u8,
    /// Display clock divide ratio (0-15). Default: 1 (value 0 = divide by 1).
    pub clock_divide_ratio: u8,
    /// Oscillator frequency (0-15). Default: 0x7.
    pub osc_freq: u8,
    /// Pre-charge phase 1 period (1-15 DCLK). Default: 7.
    pub phase1: u8,
    /// Pre-charge phase 2 period (1-15 DCLK). Default: 2.
    pub phase2: u8,
    /// IREF selection.
    pub iref: IrefSetting1320,
    /// Entire display ON (all pixels lit regardless of RAM). Default: false.
    pub entire_display_on: bool,
    /// Inverse display. Default: false.
    pub inverse_display: bool,
}

impl Default for InitConfig1320 {
    fn default() -> Self {
        Self {
            column_remap: false,
            com_remap: false,
            display_on: true,
            display_offset: 0,
            addressing_mode: AddressingMode1320::Horizontal,
            contrast: 0x7F,
            clock_divide_ratio: 0,
            osc_freq: 0x7,
            phase1: 7,
            phase2: 2,
            iref: IrefSetting1320::External,
            entire_display_on: false,
            inverse_display: false,
        }
    }
}
