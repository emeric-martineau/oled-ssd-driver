# oled-ssd-driver

A platform-independent, `#![no_std]` Rust driver for **SSD1306**, **SSD1306B**, **SSD1307**, **SSD1309**, **SSD1312**, **SSD1315**, **SSD1320**, **SSD1325**, **SSD1327**, **SSD1331**, **SSD1332**, **SSD1351**, **SSD1362**, and **CH1116** OLED display controllers.

Works on any embedded target (ESP32, STM32, nRF, RP2040, etc.) with zero heap allocation.

## Features

- **Fourteen controller support** -- `Ssd1306`, `Ssd1306b`, `Ssd1307`, `Ssd1309`, `Ssd1312`, `Ssd1315`, `Ssd1320`, `Ssd1325`, `Ssd1327`, `Ssd1331`, `Ssd1332`, `Ssd1351`, `Ssd1362`, and `Ch1116` structs with separate, optimized init sequences
- **I2C and SPI** -- Generic over `embedded-hal 1.0` bus traits
- **Sync and async** -- Full async API via `embedded-hal-async`
- **embedded-graphics integration** -- Implements `DrawTarget` for the [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) ecosystem
- **No lifetimes on the driver struct** -- Simple ownership model, easy to store in global statics
- **Internal framebuffer** -- Monochrome (up to 132x64 / 1056 bytes), 4-bit grayscale (128x80 / 5120 bytes for SSD1325, 128x128 / 8192 bytes for SSD1327, 160x160 / 12800 bytes for SSD1320, 256x64 / 8192 bytes for SSD1362), or 65K color RGB565 (96x64 / 12288 bytes for SSD1331/SSD1332, 128x128 / 32768 bytes for SSD1351), flushed on demand

## Supported Controllers

| Controller | Struct          | Tested | Key Differences | Price of device |
|------------|-----------------|--------|-----------------|-----------------|
| SSD1306    | `Ssd1306<DI>`   | ✅ I2C | Standard charge pump (on/off), 3 VCOMH levels, basic horizontal scroll | 2,11€ (2025/10/12) |
| SSD1306B   | `Ssd1306b<DI>`  | ✅ I2C | 4 charge pump voltages (6.0-9.0V), internal IREF, 4 VCOMH levels, fade/blink, zoom, content scroll, column-bounded scrolling | N/A |
| SSD1307    | `Ssd1307<DI>`   | ❌     | 128x39 resolution, no charge pump (external VCC 7-15V), external IREF, 3 VCOMH levels, horizontal scroll | N/A |
| SSD1309    | `Ssd1309<DI>`   | ❌     | 128x64 resolution, no charge pump (external VCC 7-18V), 3 VCOMH levels (0.64/0.78/0.84 x VCC), horizontal scroll | N/A |
| SSD1312    | `Ssd1312<DI>`   | ✅ I2C | 4 charge pump voltages (7.5-10.0V), internal/external IREF, 4 VCOMH levels, content scroll, column-bounded scrolling | 2.09€ (2026/02/17) |
| SSD1315    | `Ssd1315<DI>`   | ✅ I2C | 3 charge pump voltages (7.5-9.0V), internal IREF, 4 VCOMH levels, fade/blink, zoom, content scroll, column-bounded scrolling | 1.77€ (2026/02/17) |
| SSD1320    | `Ssd1320<DI>`   | ❌     | 160x160 resolution, 4-bit grayscale (16 levels), no charge pump (external VCC 8-18V), internal/external IREF, standalone `InitConfig` | N/A |
| SSD1325    | `Ssd1325<DI>`   | ❌     | 128x80 resolution, 4-bit grayscale (16 levels), no charge pump (external VCC 8-16V), external IREF via resistor, quarter/half/full current range, VCOMH/precharge voltage control, standalone `InitConfig` | N/A |
| SSD1327    | `Ssd1327<DI>`   | ❌     | 128x128 resolution, 4-bit grayscale (16 levels), internal VDD regulator (enabled by default), 256-step contrast, second precharge period, internal/external VSL, standalone `InitConfig` | N/A |
| SSD1331    | `Ssd1331<DI>`   | ❌     | 96x64 resolution, 65K color (RGB565), no charge pump (external VCC 8-18V), 256-step contrast per color channel, 16-step master current, SPI only (serial write-only), standalone `InitConfig` | N/A |
| SSD1332    | `Ssd1332<DI>`   | ❌     | 96x64 resolution, 65K color (RGB565), internal DC-DC booster, 256-step contrast per color channel, 16-step master current, separate VPA/VPB/VPC pre-charge voltages, power save mode, SPI only (serial write-only), standalone `InitConfig` | N/A |
| SSD1351    | `Ssd1351<DI>`   | ❌     | 128x128 resolution, 65K color (RGB565), internal VDD regulator, 256-step contrast per color channel, 16-step master contrast, SPI only (serial write-only), command lock mechanism, standalone `InitConfig` | N/A |
| SSD1362    | `Ssd1362<DI>`   | ❌     | 256x64 resolution, 4-bit grayscale (16 levels), internal VDD regulator, 256-step contrast, internal/external IREF, SEG left/right remap, standalone `InitConfig` | N/A |
| CH1116     | `Ch1116<DI>`    | ✅ I2C | 132x64 resolution, 4 pump voltages (6.4-10.0V), internal/external IREF with 4 resistor values, breathing light effect, horizontal scroll with column bounds, page-mode addressing | N/A |

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
oled-ssd-driver = "0.1"
```

### SSD1306 over I2C

```rust
use oled_ssd_driver::{I2cInterface, Ssd1306, DEFAULT_I2C_ADDRESS};
use embedded_graphics::{prelude::*, pixelcolor::BinaryColor, text::Text,
    mono_font::{ascii::FONT_6X10, MonoTextStyle}};

// Construct the interface (pass your platform's I2C peripheral)
let interface = I2cInterface::new(i2c, DEFAULT_I2C_ADDRESS);
let mut display = Ssd1306::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
Text::new("Hello!", Point::new(0, 10), style)
    .draw(&mut display)
    .unwrap();

display.flush().unwrap();
```

### SSD1306B over I2C

```rust
use oled_ssd_driver::{I2cInterface, Ssd1306b, ChargePumpVoltage, DEFAULT_I2C_ADDRESS};

let interface = I2cInterface::new(i2c, DEFAULT_I2C_ADDRESS);
let mut display = Ssd1306b::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

// SSD1306B-specific: select charge pump voltage
display.set_charge_pump_voltage(ChargePumpVoltage::V8_5).unwrap();

// ... draw with embedded-graphics ...

display.flush().unwrap();
```

### SSD1307 over I2C

```rust
use oled_ssd_driver::{I2cInterface, Ssd1307, DEFAULT_I2C_ADDRESS};

let interface = I2cInterface::new(i2c, DEFAULT_I2C_ADDRESS);
let mut display = Ssd1307::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

// SSD1307: no charge pump — requires external VCC (7-15V)
// ... draw with embedded-graphics ...

display.flush().unwrap();
```

### SSD1309 over I2C

```rust
use oled_ssd_driver::{I2cInterface, Ssd1309, DEFAULT_I2C_ADDRESS};

let interface = I2cInterface::new(i2c, DEFAULT_I2C_ADDRESS);
let mut display = Ssd1309::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

// SSD1309: no charge pump — requires external VCC (7-18V)
// ... draw with embedded-graphics ...

display.flush().unwrap();
```

### SSD1312 over I2C

```rust
use oled_ssd_driver::{I2cInterface, Ssd1312, ChargePumpVoltage1312, DEFAULT_I2C_ADDRESS};

let interface = I2cInterface::new(i2c, DEFAULT_I2C_ADDRESS);
let mut display = Ssd1312::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

// SSD1312-specific: select charge pump voltage (7.5V, 8.5V, 9.0V, or 10.0V)
display.set_charge_pump_voltage(ChargePumpVoltage1312::V8_5).unwrap();

// ... draw with embedded-graphics ...

display.flush().unwrap();
```

### SSD1315 over I2C

```rust
use oled_ssd_driver::{I2cInterface, Ssd1315, ChargePumpVoltage1315, DEFAULT_I2C_ADDRESS};

let interface = I2cInterface::new(i2c, DEFAULT_I2C_ADDRESS);
let mut display = Ssd1315::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

// SSD1315-specific: select charge pump voltage (7.5V, 8.5V, or 9.0V)
display.set_charge_pump_voltage(ChargePumpVoltage1315::V8_5).unwrap();

// ... draw with embedded-graphics ...

display.flush().unwrap();
```

### CH1116 over I2C

```rust
use oled_ssd_driver::{I2cInterface, Ch1116, PumpVoltageCh1116, DEFAULT_I2C_ADDRESS};

let interface = I2cInterface::new(i2c, DEFAULT_I2C_ADDRESS);
let mut display = Ch1116::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

// CH1116-specific: select pump voltage (6.4V, 7.4V, 9.0V, or 10.0V)
display.set_pump_voltage(PumpVoltageCh1116::V9_0).unwrap();

// ... draw with embedded-graphics ...

display.flush().unwrap();
```

### SSD1320 over I2C (Grayscale)

```rust
use oled_ssd_driver::{I2cInterface, Ssd1320, DEFAULT_I2C_ADDRESS};
use embedded_graphics::{prelude::*, pixelcolor::Gray4, text::Text,
    mono_font::{ascii::FONT_6X10, MonoTextStyle}};

let interface = I2cInterface::new(i2c, DEFAULT_I2C_ADDRESS);
let mut display = Ssd1320::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

// SSD1320 uses Gray4 (4-bit grayscale, 16 levels: 0x00-0x0F)
let style = MonoTextStyle::new(&FONT_6X10, Gray4::new(0x0F));
Text::new("Hello!", Point::new(0, 10), style)
    .draw(&mut display)
    .unwrap();

display.flush().unwrap();
```

### SSD1325 over I2C (Grayscale)

```rust
use oled_ssd_driver::{I2cInterface, Ssd1325, DEFAULT_I2C_ADDRESS};
use embedded_graphics::{prelude::*, pixelcolor::Gray4, text::Text,
    mono_font::{ascii::FONT_6X10, MonoTextStyle}};

let interface = I2cInterface::new(i2c, DEFAULT_I2C_ADDRESS);
let mut display = Ssd1325::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

// SSD1325 uses Gray4 (4-bit grayscale, 16 levels: 0x00-0x0F)
let style = MonoTextStyle::new(&FONT_6X10, Gray4::new(0x0F));
Text::new("Hello!", Point::new(0, 10), style)
    .draw(&mut display)
    .unwrap();

display.flush().unwrap();
```

### SSD1327 over I2C (Grayscale)

```rust
use oled_ssd_driver::{I2cInterface, Ssd1327, DEFAULT_I2C_ADDRESS};
use embedded_graphics::{prelude::*, pixelcolor::Gray4, text::Text,
    mono_font::{ascii::FONT_6X10, MonoTextStyle}};

let interface = I2cInterface::new(i2c, DEFAULT_I2C_ADDRESS);
let mut display = Ssd1327::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

// SSD1327 uses Gray4 (4-bit grayscale, 16 levels: 0x00-0x0F)
let style = MonoTextStyle::new(&FONT_6X10, Gray4::new(0x0F));
Text::new("Hello!", Point::new(0, 10), style)
    .draw(&mut display)
    .unwrap();

display.flush().unwrap();
```

### SSD1362 over I2C (Grayscale)

```rust
use oled_ssd_driver::{I2cInterface, Ssd1362, DEFAULT_I2C_ADDRESS};
use embedded_graphics::{prelude::*, pixelcolor::Gray4, text::Text,
    mono_font::{ascii::FONT_6X10, MonoTextStyle}};

let interface = I2cInterface::new(i2c, DEFAULT_I2C_ADDRESS);
let mut display = Ssd1362::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

// SSD1362 uses Gray4 (4-bit grayscale, 16 levels: 0x00-0x0F)
let style = MonoTextStyle::new(&FONT_6X10, Gray4::new(0x0F));
Text::new("Hello!", Point::new(0, 10), style)
    .draw(&mut display)
    .unwrap();

display.flush().unwrap();
```

### SSD1331 over SPI (Color)

```rust
use oled_ssd_driver::{SpiInterface, Ssd1331};
use embedded_graphics::{prelude::*, pixelcolor::Rgb565, text::Text,
    mono_font::{ascii::FONT_6X10, MonoTextStyle}};

let interface = SpiInterface::new(spi_device, dc_pin);
let mut display = Ssd1331::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

// SSD1331 uses Rgb565 (65K color, 16 bits per pixel)
let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
Text::new("Hello!", Point::new(0, 10), style)
    .draw(&mut display)
    .unwrap();

display.flush().unwrap();
```

### SSD1332 over SPI (Color)

```rust
use oled_ssd_driver::{SpiInterface, Ssd1332};
use embedded_graphics::{prelude::*, pixelcolor::Rgb565, text::Text,
    mono_font::{ascii::FONT_6X10, MonoTextStyle}};

let interface = SpiInterface::new(spi_device, dc_pin);
let mut display = Ssd1332::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

// SSD1332 uses Rgb565 (65K color, 16 bits per pixel)
let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
Text::new("Hello!", Point::new(0, 10), style)
    .draw(&mut display)
    .unwrap();

display.flush().unwrap();
```

### SSD1351 over SPI (Color)

```rust
use oled_ssd_driver::{SpiInterface, Ssd1351};
use embedded_graphics::{prelude::*, pixelcolor::Rgb565, text::Text,
    mono_font::{ascii::FONT_6X10, MonoTextStyle}};

let interface = SpiInterface::new(spi_device, dc_pin);
let mut display = Ssd1351::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

// SSD1351 uses Rgb565 (65K color, 16 bits per pixel)
let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
Text::new("Hello!", Point::new(0, 10), style)
    .draw(&mut display)
    .unwrap();

display.flush().unwrap();
```

### SPI Interface

```rust
use oled_ssd_driver::{SpiInterface, Ssd1306};

let interface = SpiInterface::new(spi_device, dc_pin);
let mut display = Ssd1306::new(interface);
display.init(&Default::default()).unwrap();
```

### Async Usage

```rust
let mut display = Ssd1306::new(interface);
display.init_async(&Default::default()).await.unwrap();
display.flush_async().await.unwrap();
```

## InitConfig

All drivers accept an `InitConfig` struct in their `init()` method to configure the display at startup:

| Field | Type | Default | Description | SSD13xx | CH1116 |
|-------|------|---------|-------------|---------|--------|
| `segment_remap` | `bool` | `true` | Column address mapped in reverse direction to SEG pins | Applied | Applied |
| `com_scan_reversed` | `bool` | `true` | COM scan from COM[N-1] to COM0 (vertically flipped) | Applied | Applied |
| `display_on` | `bool` | `true` | Turn the display on at the end of initialization | Applied | Applied |
| `display_offset` | `u8` | `0` | Vertical shift (0-63) | Applied | Applied |
| `display_start_line` | `u8` | `0` | Display start line (0-63) | Applied | Applied |
| `addressing_mode` | `Option<AddressingMode>` | `Some(Horizontal)` | Memory addressing mode | Applied if `Some` | Ignored (page-mode only) |
| `contrast` | `u8` | `0x7F` | Contrast level (0-255) | Applied | Applied |
| `vcomh_deselect` | `Option<VcomhLevel>` | `Some(V077)` | VCOMH deselect level | Applied if `Some` | Ignored (uses raw u8 API) |
| `entire_display_on` | `bool` | `false` | All pixels lit regardless of RAM content | Applied | Applied |
| `inverse_display` | `bool` | `false` | Swap ON/OFF pixels | Applied | Applied |
| `deactivate_scroll` | `bool` | `true` | Deactivate scroll during init | Applied | Applied |

The `Option<T>` fields (`addressing_mode`, `vcomh_deselect`) are skipped when `None`, which handles device-specific incompatibility. CH1116 ignores these two fields entirely since it uses page-mode addressing only and has its own raw VCOM API.

**Note:** `Ssd1320`, `Ssd1325`, `Ssd1327`, `Ssd1331`, `Ssd1332`, `Ssd1351`, and `Ssd1362` use their own `InitConfig` structs (in their respective modules, e.g. `ssd1320::InitConfig`) instead of the shared `InitConfig` because they are grayscale/color controllers with fundamentally different register layouts and commands. For convenience, the root re-exports them as `InitConfig1320`, `InitConfig1325`, `InitConfig1327`, `InitConfig1331`, `InitConfig1332`, `InitConfig1351`, and `InitConfig1362`.

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

## Hardware Wiring

### I2C

| Display Pin | MCU Pin | Notes |
|-------------|---------|-------|
| SDA | I2C SDA | Needs pull-up (typically 4.7k) |
| SCL | I2C SCL | Needs pull-up (typically 4.7k) |
| VCC | 3.3V | |
| GND | GND | |

Default I2C address: **0x3C** (SA0 = LOW). Use 0x3D if SA0 = HIGH.

### SPI (4-wire)

| Display Pin | MCU Pin | Notes |
|-------------|---------|-------|
| MOSI (D1) | SPI MOSI | Data in |
| CLK (D0) | SPI CLK | Clock |
| CS | Any GPIO | Chip select (active low) |
| D/C | Any GPIO | Data/Command select |
| RES | Any GPIO | Reset (optional, active low) |
| VCC | 3.3V | |
| GND | GND | |

## Examples

Complete working examples for real hardware:

```
examples/
    esp32-wroom-32/
        ssd1306/       # ESP32 + SSD1306 over I2C
        ssd1306b/      # ESP32 + SSD1306B over I2C
        ssd1307/       # ESP32 + SSD1307 over I2C
        ssd1309/       # ESP32 + SSD1309 over I2C
        ssd1312/       # ESP32 + SSD1312 over I2C
        ssd1315/       # ESP32 + SSD1315 over I2C
        ssd1320/       # ESP32 + SSD1320 over I2C (grayscale)
        ssd1325/       # ESP32 + SSD1325 over I2C (grayscale)
        ssd1327/       # ESP32 + SSD1327 over I2C (grayscale)
        ssd1331/       # ESP32 + SSD1331 over SPI (65K color)
        ssd1332/       # ESP32 + SSD1332 over SPI (65K color)
        ssd1362/       # ESP32 + SSD1362 over I2C (grayscale, 256x64)
        ssd1351/       # ESP32 + SSD1351 over SPI (65K color, 128x128)
        ch1116/        # ESP32 + CH1116 over I2C
    stm32-f1/
        ssd1306/       # STM32F103 "Blue Pill" + SSD1306 over I2C
        ssd1306b/      # STM32F103 "Blue Pill" + SSD1306B over I2C
        ssd1307/       # STM32F103 "Blue Pill" + SSD1307 over I2C
        ssd1309/       # STM32F103 "Blue Pill" + SSD1309 over I2C
        ssd1312/       # STM32F103 "Blue Pill" + SSD1312 over I2C
        ssd1315/       # STM32F103 "Blue Pill" + SSD1315 over I2C
        ssd1320/       # STM32F103 "Blue Pill" + SSD1320 over I2C (grayscale)
        ssd1325/       # STM32F103 "Blue Pill" + SSD1325 over I2C (grayscale)
        ssd1327/       # STM32F103 "Blue Pill" + SSD1327 over I2C (grayscale)
        ssd1331/       # STM32F103 "Blue Pill" + SSD1331 over SPI (65K color)
        ssd1332/       # STM32F103 "Blue Pill" + SSD1332 over SPI (65K color)
        ssd1362/       # STM32F103 "Blue Pill" + SSD1362 over I2C (grayscale, 256x64)
        ssd1351/       # STM32F103 "Blue Pill" + SSD1351 over SPI (65K color, 128x128)
        ch1116/        # STM32F103 "Blue Pill" + CH1116 over I2C
```

Each example draws text, lines, circles, and a bitmap image.

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `embedded-graphics-core` | 0.4 | `DrawTarget` trait |
| `embedded-hal` | 1.0 | I2C / SPI / GPIO traits (sync) |
| `embedded-hal-async` | 1.0 | I2C / SPI traits (async) |

## Minimum Supported Rust Version

This crate requires Rust **1.75+** (for async in traits / RPITIT).

## License

MIT
