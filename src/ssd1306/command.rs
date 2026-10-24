/// SSD1306 §10.1.3 — Memory addressing mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AddressingMode {
    /// Column pointer wraps within page, page does not auto-increment
    Horizontal = 0b00,
    /// Page pointer wraps within column, column auto-increments
    Vertical = 0b01,
    /// Column pointer wraps within page (must set page manually)
    Page = 0b10,
}

/// SSD1306 §10.2.1 — Horizontal scroll direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    Right,
    Left,
}

/// SSD1306 §10.2.1 — Scroll time interval between steps (in frames)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ScrollInterval {
    Frames5 = 0b000,
    Frames64 = 0b001,
    Frames128 = 0b010,
    Frames256 = 0b011,
    Frames3 = 0b100,
    Frames4 = 0b101,
    Frames25 = 0b110,
    Frames2 = 0b111,
}

/// SSD1306 §10.1.18 — COM pins hardware configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComPinConfig {
    /// `A[4]`: false = sequential, true = alternative
    pub alternative: bool,
    /// `A[5]`: enable COM left/right remap
    pub left_right_remap: bool,
}

impl ComPinConfig {
    pub(crate) fn bits(self) -> u8 {
        let mut val = 0x02;
        if self.alternative {
            val |= 0x10;
        }
        if self.left_right_remap {
            val |= 0x20;
        }
        val
    }
}

impl Default for ComPinConfig {
    fn default() -> Self {
        // SSD1306 §Table 9-1 — RESET: alternative COM pin, no remap
        Self {
            alternative: true,
            left_right_remap: false,
        }
    }
}

/// SSD1306 §10.1.19 — VCOMH deselect level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VcomhLevel {
    /// ~0.65 x VCC
    V065 = 0x00,
    /// ~0.77 x VCC (RESET)
    V077 = 0x20,
    /// ~0.83 x VCC
    V083 = 0x30,
}

/// Internal command representation — not exposed publicly
pub(crate) enum Command {
    /// SSD1306 §10.1.12 — Display ON/OFF (AEh/AFh)
    DisplayOn(bool),
    /// SSD1306 §10.1.7 — Set contrast (81h)
    SetContrast(u8),
    /// SSD1306 §10.1.9 — Entire display ON (A4h/A5h)
    EntireDisplayOn(bool),
    /// SSD1306 §10.1.10 — Normal/inverse display (A6h/A7h)
    InverseDisplay(bool),
    /// SSD1306 §10.1.3 — Set memory addressing mode (20h)
    SetAddressingMode(AddressingMode),
    /// SSD1306 §10.1.4 — Set column address (21h)
    SetColumnAddress { start: u8, end: u8 },
    /// SSD1306 §10.1.5 — Set page address (22h)
    SetPageAddress { start: u8, end: u8 },
    /// SSD1306 §10.1.6 — Set display start line (40h~7Fh)
    SetDisplayStartLine(u8),
    /// SSD1306 §10.1.8 — Set segment re-map (A0h/A1h)
    SetSegmentRemap(bool),
    /// SSD1306 §10.1.11 — Set multiplex ratio (A8h)
    SetMultiplexRatio(u8),
    /// SSD1306 §10.1.14 — Set COM output scan direction (C0h/C8h)
    SetComScanDirection(bool),
    /// SSD1306 §10.1.15 — Set display offset (D3h)
    SetDisplayOffset(u8),
    /// SSD1306 §10.1.18 — Set COM pins hardware configuration (DAh)
    SetComPinConfig(ComPinConfig),
    /// SSD1306 §10.1.16 — Set display clock divide ratio / oscillator frequency (D5h)
    SetDisplayClock { divide_ratio: u8, osc_freq: u8 },
    /// SSD1306 §10.1.17 — Set pre-charge period (D9h)
    SetPreChargePeriod { phase1: u8, phase2: u8 },
    /// SSD1306 §10.1.19 — Set VCOMH deselect level (DBh)
    SetVcomhDeselect(VcomhLevel),
    /// SSD1306 §8.9 — Charge pump setting (vendor-specific, 8Dh)
    SetChargePump(bool),
    /// SSD1306 §10.2.3 — Deactivate scroll (2Eh)
    DeactivateScroll,
}

impl Command {
    pub(crate) fn bytes(&self) -> ([u8; 3], usize) {
        match *self {
            Command::DisplayOn(on) => ([if on { 0xAF } else { 0xAE }, 0, 0], 1),
            Command::SetContrast(val) => ([0x81, val, 0], 2),
            Command::EntireDisplayOn(on) => ([if on { 0xA5 } else { 0xA4 }, 0, 0], 1),
            Command::InverseDisplay(inv) => ([if inv { 0xA7 } else { 0xA6 }, 0, 0], 1),
            Command::SetAddressingMode(mode) => ([0x20, mode as u8, 0], 2),
            Command::SetColumnAddress { start, end } => ([0x21, start, end], 3),
            Command::SetPageAddress { start, end } => ([0x22, start, end], 3),
            Command::SetDisplayStartLine(line) => ([0x40 | (line & 0x3F), 0, 0], 1),
            Command::SetSegmentRemap(remap) => ([if remap { 0xA1 } else { 0xA0 }, 0, 0], 1),
            Command::SetMultiplexRatio(ratio) => ([0xA8, ratio, 0], 2),
            Command::SetComScanDirection(remapped) => {
                ([if remapped { 0xC8 } else { 0xC0 }, 0, 0], 1)
            }
            Command::SetDisplayOffset(offset) => ([0xD3, offset & 0x3F, 0], 2),
            Command::SetComPinConfig(config) => ([0xDA, config.bits(), 0], 2),
            Command::SetDisplayClock {
                divide_ratio,
                osc_freq,
            } => ([0xD5, (osc_freq << 4) | (divide_ratio & 0x0F), 0], 2),
            Command::SetPreChargePeriod { phase1, phase2 } => {
                ([0xD9, (phase2 << 4) | (phase1 & 0x0F), 0], 2)
            }
            Command::SetVcomhDeselect(level) => ([0xDB, level as u8, 0], 2),
            Command::SetChargePump(enable) => ([0x8D, if enable { 0x14 } else { 0x10 }, 0], 2),
            Command::DeactivateScroll => ([0x2E, 0, 0], 1),
        }
    }
}
