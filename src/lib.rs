#![no_std]

pub mod ch1115;
pub mod ch1116;
mod common;
mod error;
pub mod interface;
pub mod ssd0303;
pub mod ssd1300;
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
pub mod ssd1332;
pub mod ssd1351;
pub mod ssd1362;

pub use ch1115::{
    BreathingBrightnessCh1115, Ch1115, ComScanDirectionCh1115, IrefResistorCh1115,
    IrefSourceCh1115, PumpVoltageCh1115, ScrollDirectionCh1115, ScrollIntervalCh1115,
    ScrollModeCh1115, SegPadsConfigCh1115,
};
pub use ch1116::{
    BreathingBrightnessCh1116, Ch1116, ComScanDirectionCh1116, IrefResistorCh1116,
    IrefSourceCh1116, PumpVoltageCh1116, ScrollDirectionCh1116, ScrollIntervalCh1116,
    ScrollModeCh1116,
};
pub use common::{Buffered, DisplaySize, InitConfig, Unbuffered};
pub use error::DisplayError;
pub use interface::{
    AsyncDisplayInterface, DisplayInterface, I2cInterface, SpiInterface, DEFAULT_I2C_ADDRESS,
};
pub use ssd0303::{InitConfig as InitConfig0303, ScrollInterval0303, Ssd0303};
pub use ssd1300::{InitConfig as InitConfig1300, ScrollInterval1300, Ssd1300};
pub use ssd1306::{
    AddressingMode, ComPinConfig, ScrollDirection, ScrollInterval, Ssd1306, VcomhLevel,
};
pub use ssd1306b::{
    ChargePumpVoltage, FadeMode, IrefSetting, ScrollIntervalB, Ssd1306b, VcomhLevelB,
};
pub use ssd1307::Ssd1307;
pub use ssd1309::{Ssd1309, VcomhLevel1309};
pub use ssd1312::{
    ChargePumpVoltage1312, InitConfig1312, IrefSetting1312, ScrollInterval1312, Ssd1312,
    VcomhLevel1312,
};
pub use ssd1315::{
    ChargePumpVoltage1315, FadeMode1315, IrefSetting1315, ScrollInterval1315, Ssd1315,
    VcomhLevel1315,
};
pub use ssd1320::{AddressingMode1320, InitConfig as InitConfig1320, IrefSetting1320, Ssd1320};
pub use ssd1325::{CurrentRange1325, InitConfig as InitConfig1325, Ssd1325};
pub use ssd1327::{InitConfig as InitConfig1327, Ssd1327};
pub use ssd1331::{InitConfig as InitConfig1331, Ssd1331};
pub use ssd1332::{InitConfig as InitConfig1332, Ssd1332};
pub use ssd1351::{
    InitConfig as InitConfig1351, Ssd1351, DEFAULT_DISPLAY_SIZE_H128, DEFAULT_DISPLAY_SIZE_H96,
};
pub use ssd1362::{InitConfig as InitConfig1362, Ssd1362};
