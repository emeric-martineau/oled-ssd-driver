#![no_std]

mod command;
mod command_1312;
mod command_1315;
mod command_b;
mod display;
mod display_1312;
mod display_1315;
mod display_b;
mod display_common;
mod error;
pub mod interface;

pub use command::{AddressingMode, ComPinConfig, ScrollDirection, ScrollInterval, VcomhLevel};
pub use command_1312::{
    ChargePumpVoltage1312, IrefSetting1312, ScrollInterval1312, VcomhLevel1312,
};
pub use command_1315::{
    ChargePumpVoltage1315, FadeMode1315, IrefSetting1315, ScrollInterval1315, VcomhLevel1315,
};
pub use command_b::{ChargePumpVoltage, FadeMode, IrefSetting, ScrollIntervalB, VcomhLevelB};
pub use display::Ssd1306;
pub use display_1312::Ssd1312;
pub use display_1315::Ssd1315;
pub use display_b::Ssd1306b;
pub use display_common::{DisplaySize, SIZE_128X32, SIZE_128X64, SIZE_64X48, SIZE_96X16};
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
