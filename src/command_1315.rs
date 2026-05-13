/// SSD1315 §7 — Charge pump output voltage (3 levels, no 6.0V unlike SSD1306B)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ChargePumpVoltage1315 {
    /// 7.5V (RESET)
    V7_5 = 0x14,
    /// 8.5V
    V8_5 = 0x94,
    /// 9.0V
    V9_0 = 0x95,
}

/// SSD1315 §7 — VCOMH deselect level (4 levels)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VcomhLevel1315 {
    /// ~0.65 x VCC
    V065 = 0x00,
    /// ~0.71 x VCC
    V071 = 0x10,
    /// ~0.77 x VCC (RESET)
    V077 = 0x20,
    /// ~0.83 x VCC
    V083 = 0x30,
}

/// SSD1315 §7 — Fade out / blinking mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FadeMode1315 {
    Disable = 0x00,
    FadeOut = 0x20,
    Blink = 0x30,
}

/// SSD1315 §7 — Scroll time interval
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ScrollInterval1315 {
    Frames6 = 0b000,
    Frames32 = 0b001,
    Frames64 = 0b010,
    Frames128 = 0b011,
    Frames3 = 0b100,
    Frames4 = 0b101,
    Frames5 = 0b110,
    Frames2 = 0b111,
}

/// SSD1315 §7 — Internal IREF current setting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrefSetting1315 {
    /// 19uA, maximum ISEG=150uA (RESET)
    Low,
    /// 30uA, maximum ISEG=240uA
    High,
}
