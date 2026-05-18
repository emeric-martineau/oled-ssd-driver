/// SSD1309 §9.5 — VCOMH deselect level (DBh)
/// Note: SSD1309 uses different bit patterns than SSD1306.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VcomhLevel1309 {
    /// ~0.64 x VCC
    V064 = 0x00,
    /// ~0.78 x VCC (RESET)
    V078 = 0x34,
    /// ~0.84 x VCC
    V084 = 0x3C,
}
