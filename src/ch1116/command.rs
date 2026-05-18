/// CH1116 §8 — Pump voltage output (4 levels: 6.4V, 7.4V, 9.0V, 10.0V)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PumpVoltageCh1116 {
    /// 6.4V
    V6_4 = 0x30,
    /// 7.4V (Power On Reset)
    V7_4 = 0x31,
    /// 9.0V
    V9_0 = 0x32,
    /// 10.0V
    V10_0 = 0x33,
}

/// CH1116 §11 — IREF resistor selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IrefResistorCh1116 {
    /// 510K (POR)
    K510 = 0x00,
    /// 310K
    K310 = 0x01,
    /// 220K
    K220 = 0x02,
    /// 180K
    K180 = 0x03,
}

/// CH1116 §11 — IREF source selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrefSourceCh1116 {
    /// External resistor (POR)
    External,
    /// Internal resistor
    Internal,
}

/// CH1116 §5 — Horizontal scroll time interval
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ScrollIntervalCh1116 {
    /// 6 frames (POR)
    Frames6 = 0b000,
    /// 32 frames
    Frames32 = 0b001,
    /// 64 frames
    Frames64 = 0b010,
    /// 128 frames
    Frames128 = 0b011,
    /// 3 frames
    Frames3 = 0b100,
    /// 4 frames
    Frames4 = 0b101,
    /// 5 frames
    Frames5 = 0b110,
    /// 2 frames
    Frames2 = 0b111,
}

/// CH1116 §5 — Scroll direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirectionCh1116 {
    Right,
    Left,
}

/// CH1116 §6 — Scroll mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollModeCh1116 {
    /// Continuous horizontal scroll (POR)
    Continuous,
    /// Single screen horizontal scroll
    SingleScreen,
}

/// CH1116 §3 — Breathing light maximum brightness
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BreathingBrightnessCh1116 {
    /// 256 (Contrast+1) — POR
    Full = 0x00,
    /// 128
    Half = 0x08,
    /// 64
    Quarter = 0x10,
    /// 32
    Eighth = 0x18,
}

/// CH1116 §19 — COM scan direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComScanDirectionCh1116 {
    /// Scan from COM0 to COM[N-1] (POR)
    Normal,
    /// Scan from COM[N-1] to COM0
    Reversed,
}
