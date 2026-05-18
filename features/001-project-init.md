# 001 - Project Initialization

## Status: DONE

## Description

Implement Rust code to can use Oled display with driver SSD1306 (datasheet: `oled-ssd-driver/documentations/SSD1306.pdf`).

The main struct use by final user must have less timelive constraints.

All codes must be compatible with `sync` and `async` code.

All methods, code, must be commented with datasheet reference.

Create a full example for ESP32 WROOM and STM32-F1 microcontroller.

## What was created

- `src/display_1306.rs` — `Ssd1306<DI>` driver struct (128x64 monochrome, 1024-byte buffer), sync/async init, flush, display control, `DrawTarget` for `BinaryColor`
- `src/command_1306.rs` — SSD1306-specific types: `AddressingMode`, `ComPinConfig`, `ScrollDirection`, `ScrollInterval`, `VcomhLevel`
- `src/display_common.rs` — shared `DisplayCore` with common buffer management, `InitConfig`, `DisplaySize`, and predefined sizes (`SIZE_128X64`, `SIZE_128X32`, `SIZE_96X16`)
- `src/interface.rs` — `I2cInterface` and `SpiInterface` (generic over `embedded-hal` traits)
- `src/error.rs` — `DisplayError` enum
- `src/lib.rs` — crate root, re-exports all public types
- `tests/display_1306_test.rs` — unit tests for SSD1306 driver
- `examples/esp32-wroom-32/ssd1306/` — full ESP32-WROOM example (I2C)
- `examples/stm32-f1/ssd1306/` — full STM32F103 "Blue Pill" example (I2C)
- `Cargo.toml` — crate manifest with `embedded-graphics-core`, `embedded-hal`, `embedded-hal-async` dependencies
