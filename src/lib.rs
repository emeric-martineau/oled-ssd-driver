#![no_std]

pub mod ch1116;
mod common;
mod error;
pub mod interface;
pub mod ssd1306;
pub mod ssd1306b;
pub mod ssd1307;
pub mod ssd1309;
pub mod ssd1312;
pub mod ssd1315;
pub mod ssd1320;
pub mod ssd1325;
pub mod ssd1327;
pub mod ssd1331;

pub use ch1116::{
    BreathingBrightnessCh1116, Ch1116, ComScanDirectionCh1116, IrefResistorCh1116,
    IrefSourceCh1116, PumpVoltageCh1116, ScrollDirectionCh1116, ScrollIntervalCh1116,
    ScrollModeCh1116,
};
pub use common::{
    DisplaySize, InitConfig, SIZE_128X32, SIZE_128X39, SIZE_128X64, SIZE_132X64, SIZE_64X48,
    SIZE_96X16,
};
pub use error::DisplayError;
pub use interface::{
    AsyncDisplayInterface, DisplayInterface, I2cInterface, SpiInterface, DEFAULT_I2C_ADDRESS,
};
pub use ssd1306::{
    AddressingMode, ComPinConfig, ScrollDirection, ScrollInterval, Ssd1306, VcomhLevel,
};
pub use ssd1306b::{
    ChargePumpVoltage, FadeMode, IrefSetting, ScrollIntervalB, Ssd1306b, VcomhLevelB,
};
pub use ssd1307::Ssd1307;
pub use ssd1309::{Ssd1309, VcomhLevel1309};
pub use ssd1312::{
    ChargePumpVoltage1312, IrefSetting1312, ScrollInterval1312, Ssd1312, VcomhLevel1312,
};
pub use ssd1315::{
    ChargePumpVoltage1315, FadeMode1315, IrefSetting1315, ScrollInterval1315, Ssd1315,
    VcomhLevel1315,
};
pub use ssd1320::{
    AddressingMode1320, DisplaySize1320, InitConfig as InitConfig1320, IrefSetting1320, Ssd1320,
    SIZE_160X160,
};
pub use ssd1325::{
    CurrentRange1325, DisplaySize1325, InitConfig as InitConfig1325, Ssd1325, SIZE_128X80,
};
pub use ssd1327::{DisplaySize1327, InitConfig as InitConfig1327, Ssd1327, SIZE_128X128};
pub use ssd1331::{DisplaySize1331, InitConfig as InitConfig1331, Ssd1331, SIZE_96X64};

/// Default display width (128 pixels)
pub const WIDTH: u8 = 128;
/// Default display height (64 pixels)
pub const HEIGHT: u8 = 64;
/// Default number of pages (8 for 64px height)
pub const PAGES: u8 = HEIGHT / 8;
