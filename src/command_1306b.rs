/// SSD1306B §8 Table 8-1 — Charge pump output voltage mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ChargePumpVoltage {
    /// 7.5V (RESET)
    V7_5 = 0x14,
    /// 6.0V
    V6_0 = 0x15,
    /// 8.5V
    V8_5 = 0x94,
    /// 9.0V
    V9_0 = 0x95,
}

/// SSD1306B §8 Table 8-1 — VCOMH deselect level (4 levels vs SSD1306's 3)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VcomhLevelB {
    /// ~0.65 x VCC
    V065 = 0x00,
    /// ~0.71 x VCC
    V071 = 0x10,
    /// ~0.77 x VCC (RESET)
    V077 = 0x20,
    /// ~0.83 x VCC
    V083 = 0x30,
}

/// SSD1306B §8 Table 8-1 — Fade out / blinking mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FadeMode {
    Disable = 0x00,
    FadeOut = 0x20,
    Blink = 0x30,
}

/// SSD1306B §8 Table 8-1 — Scroll time interval (different from SSD1306)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ScrollIntervalB {
    Frames6 = 0b000,
    Frames32 = 0b001,
    Frames64 = 0b010,
    Frames128 = 0b011,
    Frames3 = 0b100,
    Frames4 = 0b101,
    Frames5 = 0b110,
    Frames2 = 0b111,
}

/// SSD1306B §8 Table 8-1 — Internal IREF current setting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrefSetting {
    /// 19uA, maximum ISEG=150uA (RESET)
    Low,
    /// 30uA, maximum ISEG=240uA
    High,
}
