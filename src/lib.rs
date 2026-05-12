#![no_std]

mod command;
mod command_b;
mod display;
mod display_b;
mod display_common;
mod error;
pub mod interface;

pub use command::{AddressingMode, ComPinConfig, ScrollDirection, ScrollInterval, VcomhLevel};
pub use command_b::{ChargePumpVoltage, FadeMode, IrefSetting, ScrollIntervalB, VcomhLevelB};
pub use display::Ssd1306;
pub use display_b::Ssd1306b;
pub use error::DisplayError;
pub use interface::{
    AsyncDisplayInterface, DisplayInterface, I2cInterface, SpiInterface, DEFAULT_I2C_ADDRESS,
};

pub const WIDTH: u8 = 128;
pub const HEIGHT: u8 = 64;
pub const PAGES: u8 = HEIGHT / 8;
