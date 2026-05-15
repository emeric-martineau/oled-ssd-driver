#![no_std]

mod command_1306;
mod command_1306b;
mod command_1309;
mod command_1312;
mod command_1315;
mod command_1320;
mod command_ch1116;
mod display_1306;
mod display_1306b;
mod display_1307;
mod display_1309;
mod display_1312;
mod display_1315;
mod display_1320;
mod display_ch1116;
mod display_common;
mod error;
pub mod interface;

pub use command_1306::{AddressingMode, ComPinConfig, ScrollDirection, ScrollInterval, VcomhLevel};
pub use command_1306b::{ChargePumpVoltage, FadeMode, IrefSetting, ScrollIntervalB, VcomhLevelB};
pub use command_1309::VcomhLevel1309;
pub use command_1312::{
    ChargePumpVoltage1312, IrefSetting1312, ScrollInterval1312, VcomhLevel1312,
};
pub use command_1315::{
    ChargePumpVoltage1315, FadeMode1315, IrefSetting1315, ScrollInterval1315, VcomhLevel1315,
};
pub use command_1320::{
    AddressingMode1320, DisplaySize1320, InitConfig1320, IrefSetting1320, SIZE_160X160,
};
pub use command_ch1116::{
    BreathingBrightnessCh1116, ComScanDirectionCh1116, IrefResistorCh1116, IrefSourceCh1116,
    PumpVoltageCh1116, ScrollDirectionCh1116, ScrollIntervalCh1116, ScrollModeCh1116,
};
pub use display_1306::Ssd1306;
pub use display_1306b::Ssd1306b;
pub use display_1307::Ssd1307;
pub use display_1309::Ssd1309;
pub use display_1312::Ssd1312;
pub use display_1315::Ssd1315;
pub use display_1320::Ssd1320;
pub use display_ch1116::Ch1116;
pub use display_common::{
    DisplaySize, InitConfig, SIZE_128X32, SIZE_128X39, SIZE_128X64, SIZE_132X64, SIZE_64X48,
    SIZE_96X16,
};
pub use error::DisplayError;
pub use interface::{
    AsyncDisplayInterface, DisplayInterface, I2cInterface, SpiInterface, DEFAULT_I2C_ADDRESS,
};

/// Default display width (128 pixels)
pub const WIDTH: u8 = 128;
/// Default display height (64 pixels)
pub const HEIGHT: u8 = 64;
/// Default number of pages (8 for 64px height)
pub const PAGES: u8 = HEIGHT / 8;
