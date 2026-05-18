pub(crate) mod command;
mod display;

pub use command::{
    ChargePumpVoltage1315, FadeMode1315, IrefSetting1315, ScrollInterval1315, VcomhLevel1315,
};
pub use display::Ssd1315;
