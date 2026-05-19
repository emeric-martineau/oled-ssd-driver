pub(crate) mod command;
mod display;

pub use command::{
    BreathingBrightnessCh1115, ComScanDirectionCh1115, IrefResistorCh1115, IrefSourceCh1115,
    PumpVoltageCh1115, ScrollDirectionCh1115, ScrollIntervalCh1115, ScrollModeCh1115,
    SegPadsConfigCh1115,
};
pub use display::Ch1115;
