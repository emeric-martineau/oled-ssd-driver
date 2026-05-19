/// CH1115 §8 — Pump voltage output (4 levels: 10.0V, 7.4V, 8.0V, 9.0V)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PumpVoltageCh1115 {
    /// 10.0V
    V10_0 = 0x30,
    /// 7.4V
    V7_4 = 0x31,
    /// 8.0V (Power On Reset)
    V8_0 = 0x32,
    /// 9.0V
    V9_0 = 0x33,
}

/// CH1115 §11 — IREF resistor selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IrefResistorCh1115 {
    /// 510K (POR)
    K510 = 0x00,
    /// 310K
    K310 = 0x01,
    /// 220K
    K220 = 0x02,
    /// 180K
    K180 = 0x03,
}

/// CH1115 §11 — IREF source selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrefSourceCh1115 {
    /// External resistor (POR)
    External,
    /// Internal resistor
    Internal,
}

/// CH1115 §5 — Horizontal scroll time interval
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ScrollIntervalCh1115 {
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

/// CH1115 §5 — Scroll direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirectionCh1115 {
    Right,
    Left,
}

/// CH1115 §6 — Scroll mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollModeCh1115 {
    /// Continuous horizontal/vertical scroll (POR)
    Continuous,
    /// Single screen horizontal scroll
    SingleScreen,
}

/// CH1115 §3 — Breathing light maximum brightness
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BreathingBrightnessCh1115 {
    /// 256 (Contrast+1) — POR
    Full = 0x00,
    /// 128
    Half = 0x08,
    /// 64
    Quarter = 0x10,
    /// 32
    Eighth = 0x18,
}

/// CH1115 §20 — COM scan direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComScanDirectionCh1115 {
    /// Scan from COM0 to COM[N-1] (POR)
    Normal,
    /// Scan from COM[N-1] to COM0
    Reversed,
}

/// CH1115 §13 — SEG pads hardware configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegPadsConfigCh1115 {
    /// Even on the left (POR)
    EvenLeft,
    /// Even on the right
    EvenRight,
}
