# Driver-Specific API Reference

This document describes the specific API for each driver beyond the common `DrawTarget` / `init` / `flush` interface.

## SSD0303-Specific API

The `Ssd0303` driver uses a standalone configuration (`ssd0303::InitConfig`, re-exported as `InitConfig0303`) and `BinaryColor` pixel color:

```rust
use oled_ssd_driver::{Ssd0303, InitConfig0303, ScrollInterval0303};

// Custom init config
let config = InitConfig0303 {
    contrast: 0x80,
    segment_remap: true,
    com_scan_reversed: true,
    dc_dc_enable: true,
    display_on: true,
    ..Default::default()
};
display.init(&config).unwrap();

// Set contrast (0-255)
display.set_contrast(0xCF).unwrap();

// Set VCOM deselect level (7-bit value, 0-127)
display.set_vcomh(0x35).unwrap();

// Set DC-DC on/off
display.set_dc_dc(true).unwrap();

// Set brightness for color banks (0-255)
display.set_brightness(0x80).unwrap();

// Horizontal scroll (start_page, end_page, num_columns_per_step, interval)
display.start_horizontal_scroll(0, 7, 1, ScrollInterval0303::Frames64).unwrap();

// Stop scrolling
display.stop_scroll().unwrap();

// Inverse display
display.set_inverse(true).unwrap();

// Display on/off
display.display_on(false).unwrap();
```

## SSD1300-Specific API

The `Ssd1300` driver uses a standalone configuration (`ssd1300::InitConfig`, re-exported as `InitConfig1300`) and `BinaryColor` pixel color:

```rust
use oled_ssd_driver::{Ssd1300, InitConfig1300, ScrollInterval1300};

// Custom init config
let config = InitConfig1300 {
    contrast: 0x80,
    segment_remap: true,
    com_scan_reversed: true,
    dc_dc_enable: true,
    internal_vcomh: true,
    display_on: true,
    ..Default::default()
};
display.init(&config).unwrap();

// Set contrast (0-255)
display.set_contrast(0xCF).unwrap();

// Set VCOMH deselect level (7-bit value, 0-127)
display.set_vcomh(0x3F).unwrap();

// Set DC-DC on/off and VCOMH source
display.set_dc_dc(true, true).unwrap();

// Horizontal scroll (start_page, end_page, num_columns_per_step, interval)
display.start_horizontal_scroll(0, 5, 1, ScrollInterval1300::Frames64).unwrap();

// Stop scrolling
display.stop_scroll().unwrap();

// Inverse display
display.set_inverse(true).unwrap();

// Display on/off
display.display_on(false).unwrap();
```

## SSD1306B-Specific API

The `Ssd1306b` driver exposes additional hardware features:

```rust
use oled_ssd_driver::{Ssd1306b, ChargePumpVoltage, IrefSetting,
    VcomhLevelB, FadeMode, ScrollDirection, ScrollIntervalB};

// Charge pump voltage (6.0V, 7.5V, 8.5V, 9.0V)
display.set_charge_pump_voltage(ChargePumpVoltage::V9_0).unwrap();

// Internal current reference (19uA or 30uA)
display.set_iref(IrefSetting::High).unwrap();

// VCOMH deselect level (4 options: 0.65, 0.71, 0.77, 0.83 x VCC)
display.set_vcomh_deselect(VcomhLevelB::V083).unwrap();

// Fade out or blink effect
display.set_fade_blink(FadeMode::Blink, 3).unwrap();

// Zoom in (requires alternative COM pin config)
display.set_zoom(true).unwrap();

// Horizontal scroll with column bounds
display.start_horizontal_scroll(
    ScrollDirection::Right, 0, 7,
    ScrollIntervalB::Frames6, 0, 127
).unwrap();

// Content scroll (SSD1306B-exclusive)
display.start_content_scroll(
    ScrollDirection::Left, 0, 7,
    ScrollIntervalB::Frames32, 10, 100
).unwrap();

display.stop_scroll().unwrap();
```

## SSD1312-Specific API

The `Ssd1312` driver exposes additional hardware features:

```rust
use oled_ssd_driver::{Ssd1312, ChargePumpVoltage1312, IrefSetting1312,
    VcomhLevel1312, ScrollDirection, ScrollInterval1312};

// Charge pump voltage (7.5V, 8.5V, 9.0V, 10.0V)
display.set_charge_pump_voltage(ChargePumpVoltage1312::V10_0).unwrap();

// IREF selection (internal or external)
display.set_iref(IrefSetting1312::Internal).unwrap();

// VCOMH deselect level (4 options: 0.65, 0.71, 0.77, 0.83 x VCC)
display.set_vcomh_deselect(VcomhLevel1312::V083).unwrap();

// Horizontal scroll with column bounds
display.start_horizontal_scroll(
    ScrollDirection::Right, 0, 7,
    ScrollInterval1312::Frames6, 0, 127
).unwrap();

// Content scroll
display.start_content_scroll(
    ScrollDirection::Left, 0, 7,
    ScrollInterval1312::Frames32, 10, 100
).unwrap();

display.stop_scroll().unwrap();
```

## SSD1315-Specific API

The `Ssd1315` driver exposes additional hardware features:

```rust
use oled_ssd_driver::{Ssd1315, ChargePumpVoltage1315, IrefSetting1315,
    VcomhLevel1315, FadeMode1315, ScrollDirection, ScrollInterval1315};

// Charge pump voltage (7.5V, 8.5V, 9.0V — no 6.0V unlike SSD1306B)
display.set_charge_pump_voltage(ChargePumpVoltage1315::V9_0).unwrap();

// Internal current reference (19uA or 30uA)
display.set_iref(IrefSetting1315::High).unwrap();

// VCOMH deselect level (4 options: 0.65, 0.71, 0.77, 0.83 x VCC)
display.set_vcomh_deselect(VcomhLevel1315::V083).unwrap();

// Fade out or blink effect
display.set_fade_blink(FadeMode1315::Blink, 3).unwrap();

// Zoom in (requires alternative COM pin config)
display.set_zoom(true).unwrap();

// Horizontal scroll with column bounds
display.start_horizontal_scroll(
    ScrollDirection::Right, 0, 7,
    ScrollInterval1315::Frames6, 0, 127
).unwrap();

// Content scroll (SSD1315-exclusive)
display.start_content_scroll(
    ScrollDirection::Left, 0, 7,
    ScrollInterval1315::Frames32, 10, 100
).unwrap();

display.stop_scroll().unwrap();
```

## SSD1320-Specific API

The `Ssd1320` driver uses a standalone configuration (`ssd1320::InitConfig`, re-exported as `InitConfig1320`) and `Gray4` pixel color:

```rust
use oled_ssd_driver::{Ssd1320, IrefSetting1320, InitConfig1320};

// Custom init config
let config = InitConfig1320 {
    contrast: 0x7F,
    display_on: true,
    inverse_display: false,
    ..Default::default()
};
display.init(&config).unwrap();

// Internal/external current reference
display.set_iref(IrefSetting1320::Internal).unwrap();

// Set contrast (0-255)
display.set_contrast(0xCF).unwrap();

// Inverse display
display.set_inverse(true).unwrap();

// Display on/off
display.display_on(false).unwrap();
```

## SSD1325-Specific API

The `Ssd1325` driver uses a standalone configuration (`ssd1325::InitConfig`, re-exported as `InitConfig1325`) and `Gray4` pixel color:

```rust
use oled_ssd_driver::{Ssd1325, CurrentRange1325, InitConfig1325};

// Custom init config with remap options
let config = InitConfig1325 {
    contrast: 0x40,
    column_remap: true,
    com_remap: true,
    com_split_odd_even: true,
    display_on: true,
    ..Default::default()
};
display.init(&config).unwrap();

// Current range (quarter, half, full)
display.set_current_range(CurrentRange1325::Full).unwrap();

// Set contrast (0-127, 7-bit)
display.set_contrast(0x5F).unwrap();

// Set VCOMH deselect level (5-bit value)
display.set_vcomh(0x1D).unwrap();

// Set precharge voltage (8-bit value)
display.set_precharge_voltage(0x20).unwrap();

// Inverse display
display.set_inverse(true).unwrap();

// Display on/off
display.display_on(false).unwrap();
```

## SSD1327-Specific API

The `Ssd1327` driver uses a standalone configuration (`ssd1327::InitConfig`, re-exported as `InitConfig1327`) and `Gray4` pixel color:

```rust
use oled_ssd_driver::{Ssd1327, InitConfig1327};

// Custom init config with remap options
let config = InitConfig1327 {
    contrast: 0x7F,
    column_remap: true,
    com_remap: true,
    com_split_odd_even: true,
    internal_vdd: true,
    display_on: true,
    ..Default::default()
};
display.init(&config).unwrap();

// Set contrast (0-255, 8-bit)
display.set_contrast(0xCF).unwrap();

// Set VCOMH deselect level (3-bit value, 0-7)
display.set_vcomh(0x07).unwrap();

// Set precharge voltage (4-bit value, 0-8)
display.set_precharge_voltage(0x08).unwrap();

// Enable/disable internal VDD regulator
display.set_internal_vdd(true).unwrap();

// Inverse display
display.set_inverse(true).unwrap();

// Display on/off
display.display_on(false).unwrap();
```

## SSD1331-Specific API

The `Ssd1331` driver uses a standalone configuration (`ssd1331::InitConfig`, re-exported as `InitConfig1331`) and `Rgb565` pixel color:

```rust
use oled_ssd_driver::{Ssd1331, InitConfig1331};

// Custom init config with remap options
let config = InitConfig1331 {
    contrast_a: 0x80,
    contrast_b: 0x80,
    contrast_c: 0x80,
    master_current: 0x0F,
    column_remap: true,
    com_scan_reversed: true,
    com_split_odd_even: true,
    display_on: true,
    ..Default::default()
};
display.init(&config).unwrap();

// Set contrast per color channel (0-255)
display.set_contrast_a(0xFF).unwrap();
display.set_contrast_b(0xC0).unwrap();
display.set_contrast_c(0xC0).unwrap();

// Set master current attenuation (0-15)
display.set_master_current(0x0F).unwrap();

// Set VCOMH deselect level (6-bit value, 0-63)
display.set_vcomh(0x1E).unwrap();

// Set precharge voltage (6-bit value, 0-63)
display.set_precharge_voltage(0x1E).unwrap();

// Inverse display
display.set_inverse(true).unwrap();

// Display on/off
display.display_on(false).unwrap();
```

## SSD1332-Specific API

The `Ssd1332` driver uses a standalone configuration (`ssd1332::InitConfig`, re-exported as `InitConfig1332`) and `Rgb565` pixel color:

```rust
use oled_ssd_driver::{Ssd1332, InitConfig1332};

// Custom init config with remap options
let config = InitConfig1332 {
    contrast_a: 0x80,
    contrast_b: 0x80,
    contrast_c: 0x80,
    master_current: 0x0F,
    column_remap: true,
    com_scan_reversed: true,
    com_split_odd_even: true,
    internal_booster: true,
    internal_vp: true,
    display_on: true,
    ..Default::default()
};
display.init(&config).unwrap();

// Set contrast per color channel (0-255)
display.set_contrast_a(0xFF).unwrap();
display.set_contrast_b(0xC0).unwrap();
display.set_contrast_c(0xC0).unwrap();

// Set master current attenuation (0-15)
display.set_master_current(0x0F).unwrap();

// Set VCOMH deselect level (6-bit value, 0-63)
display.set_vcomh(0x3F).unwrap();

// Set VPA/VPB/VPC pre-charge voltages (0-255)
display.set_vpa(0xFF).unwrap();
display.set_vpb(0xFF).unwrap();
display.set_vpc(0xFF).unwrap();

// Power save mode
display.set_power_save(true).unwrap();

// Inverse display
display.set_inverse(true).unwrap();

// Display on/off
display.display_on(false).unwrap();
```

## SSD1351-Specific API

The `Ssd1351` driver uses a standalone configuration (`ssd1351::InitConfig`, re-exported as `InitConfig1351`) and `Rgb565` pixel color:

```rust
use oled_ssd_driver::{Ssd1351, InitConfig1351};

// Custom init config with remap options
let config = InitConfig1351 {
    contrast_a: 0x8A,
    contrast_b: 0x51,
    contrast_c: 0x8A,
    master_contrast: 0x0F,
    column_remap: true,
    com_scan_reversed: true,
    com_split_odd_even: true,
    internal_vdd: true,
    display_on: true,
    ..Default::default()
};
display.init(&config).unwrap();

// Set contrast per color channel (0-255)
display.set_contrast_abc(0xFF, 0xC0, 0xC0).unwrap();

// Set master contrast current control (0-15)
display.set_master_contrast(0x0F).unwrap();

// Set VCOMH deselect level (3-bit value, 0-7)
display.set_vcomh(0x05).unwrap();

// Set precharge voltage (5-bit value, 0-31)
display.set_precharge_voltage(0x17).unwrap();

// Inverse display
display.set_inverse(true).unwrap();

// Display on/off
display.display_on(false).unwrap();
```

## SSD1362-Specific API

The `Ssd1362` driver uses a standalone configuration (`ssd1362::InitConfig`, re-exported as `InitConfig1362`) and `Gray4` pixel color:

```rust
use oled_ssd_driver::{Ssd1362, InitConfig1362};

// Custom init config with remap options
let config = InitConfig1362 {
    contrast: 0x7F,
    column_remap: true,
    com_remap: true,
    seg_split_odd_even: true,
    seg_left_right_remap: false,
    internal_vdd: true,
    internal_iref: true,
    display_on: true,
    ..Default::default()
};
display.init(&config).unwrap();

// Set contrast (0-255, 8-bit)
display.set_contrast(0xCF).unwrap();

// Set VCOMH deselect level (4-bit value, 0-7)
display.set_vcomh(0x07).unwrap();

// Set precharge voltage (5-bit value, 0-31)
display.set_precharge_voltage(0x08).unwrap();

// Enable/disable internal VDD regulator
display.set_internal_vdd(true).unwrap();

// Select internal/external IREF
display.set_iref(true).unwrap();

// Inverse display
display.set_inverse(true).unwrap();

// Display on/off
display.display_on(false).unwrap();
```

## CH1116-Specific API

The `Ch1116` driver exposes additional hardware features:

```rust
use oled_ssd_driver::{Ch1116, PumpVoltageCh1116, IrefSourceCh1116, IrefResistorCh1116,
    BreathingBrightnessCh1116, ScrollDirectionCh1116, ScrollIntervalCh1116, ScrollModeCh1116};

// Pump voltage (6.4V, 7.4V, 9.0V, 10.0V)
display.set_pump_voltage(PumpVoltageCh1116::V9_0).unwrap();

// IREF source and resistor (internal/external, 510K/310K/220K/180K)
display.set_iref(IrefSourceCh1116::Internal, IrefResistorCh1116::K310).unwrap();

// VCOM deselect level (raw 8-bit value, see datasheet)
display.set_vcom_deselect(0x35).unwrap();

// DC-DC converter ON/OFF
display.set_dc_dc(true).unwrap();

// Breathing light effect (ON/OFF, brightness, interval 1-8 frames)
display.set_breathing_light(true, BreathingBrightnessCh1116::Half, 3).unwrap();

// Horizontal scroll
display.set_scroll_columns(10, 100).unwrap();
display.start_horizontal_scroll(
    ScrollDirectionCh1116::Right, 0, 7,
    ScrollIntervalCh1116::Frames6
).unwrap();

// Scroll mode (continuous or single screen)
display.set_scroll_mode(ScrollModeCh1116::SingleScreen).unwrap();

display.stop_scroll().unwrap();
```
