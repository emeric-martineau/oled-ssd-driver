# oled-ssd-driver

A platform-independent, `#![no_std]` Rust driver for **SSD1306**, **SSD1306B**, **SSD1312**, and **SSD1315** OLED display controllers.

Works on any embedded target (ESP32, STM32, nRF, RP2040, etc.) with zero heap allocation.

## Features

- **Quad controller support** -- `Ssd1306`, `Ssd1306b`, `Ssd1312`, and `Ssd1315` structs with separate, optimized init sequences
- **I2C and SPI** -- Generic over `embedded-hal 1.0` bus traits
- **Sync and async** -- Full async API via `embedded-hal-async`
- **embedded-graphics integration** -- Implements `DrawTarget` for the [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) ecosystem
- **No lifetimes on the driver struct** -- Simple ownership model, easy to store in global statics
- **1024-byte internal framebuffer** -- 128x64 monochrome, flushed on demand

## Supported Controllers

| Controller | Struct | Key Differences |
|------------|--------|----------------|
| SSD1306 | `Ssd1306<DI>` | Standard charge pump (on/off), 3 VCOMH levels, basic horizontal scroll |
| SSD1306B | `Ssd1306b<DI>` | 4 charge pump voltages (6.0-9.0V), internal IREF, 4 VCOMH levels, fade/blink, zoom, content scroll, column-bounded scrolling |
| SSD1312 | `Ssd1312<DI>` | 4 charge pump voltages (7.5-10.0V), internal/external IREF, 4 VCOMH levels, content scroll, column-bounded scrolling |
| SSD1315 | `Ssd1315<DI>` | 3 charge pump voltages (7.5-9.0V), internal IREF, 4 VCOMH levels, fade/blink, zoom, content scroll, column-bounded scrolling |

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

display.init().unwrap();
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

display.init().unwrap();
display.clear_buffer();

// SSD1306B-specific: select charge pump voltage
display.set_charge_pump_voltage(ChargePumpVoltage::V8_5).unwrap();

// ... draw with embedded-graphics ...

display.flush().unwrap();
```

### SSD1312 over I2C

```rust
use oled_ssd_driver::{I2cInterface, Ssd1312, ChargePumpVoltage1312, DEFAULT_I2C_ADDRESS};

let interface = I2cInterface::new(i2c, DEFAULT_I2C_ADDRESS);
let mut display = Ssd1312::new(interface);

display.init().unwrap();
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

display.init().unwrap();
display.clear_buffer();

// SSD1315-specific: select charge pump voltage (7.5V, 8.5V, or 9.0V)
display.set_charge_pump_voltage(ChargePumpVoltage1315::V8_5).unwrap();

// ... draw with embedded-graphics ...

display.flush().unwrap();
```

### SPI Interface

```rust
use oled_ssd_driver::{SpiInterface, Ssd1306};

let interface = SpiInterface::new(spi_device, dc_pin);
let mut display = Ssd1306::new(interface);
display.init().unwrap();
```

### Async Usage

```rust
let mut display = Ssd1306::new(interface);
display.init_async().await.unwrap();
display.flush_async().await.unwrap();
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
        ssd1312/       # ESP32 + SSD1312 over I2C
        ssd1315/       # ESP32 + SSD1315 over I2C
    stm32-f1/
        ssd1306/       # STM32F103 "Blue Pill" + SSD1306 over I2C
        ssd1306b/      # STM32F103 "Blue Pill" + SSD1306B over I2C
        ssd1312/       # STM32F103 "Blue Pill" + SSD1312 over I2C
        ssd1315/       # STM32F103 "Blue Pill" + SSD1315 over I2C
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
