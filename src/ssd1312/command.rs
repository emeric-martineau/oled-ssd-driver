/// SSD1312 §6.10 — Charge pump output voltage (4 levels: 7.5V, 8.5V, 9V, 10V)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ChargePumpVoltage1312 {
    /// 7.5V (RESET)
    V7_5 = 0x14,
    /// 10.0V
    V10_0 = 0x15,
    /// 8.5V
    V8_5 = 0x94,
    /// 9.0V
    V9_0 = 0x95,
}

/// SSD1312 §6.8 — VCOMH deselect level (4 levels)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VcomhLevel1312 {
    /// ~0.65 x VCC
    V065 = 0x00,
    /// ~0.71 x VCC
    V071 = 0x10,
    /// ~0.77 x VCC (RESET)
    V077 = 0x20,
    /// ~0.83 x VCC
    V083 = 0x30,
}

/// SSD1312 — Scroll time interval
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ScrollInterval1312 {
    Frames6 = 0b000,
    Frames32 = 0b001,
    Frames64 = 0b010,
    Frames128 = 0b011,
    Frames3 = 0b100,
    Frames4 = 0b101,
    Frames5 = 0b110,
    Frames2 = 0b111,
}

/// SSD1312 §6.8 — Internal IREF current setting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrefSetting1312 {
    /// External IREF (use external resistor)
    External,
    /// Internal IREF enabled
    Internal,
}
