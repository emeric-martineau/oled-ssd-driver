use crate::common::InitConfig;

/// SSD1312 default configuration
pub struct InitConfig1312;

impl InitConfig1312 {
    pub fn default() -> InitConfig {
        InitConfig {
            segment_remap: true,
            com_scan_reversed: false,
            ..InitConfig::default()
        }
    }
}
