# 023 - Add support of oled driver ch1115

## Status: DONE

## Description

Update code to add support of CH1115 driver (datasheet: `oled-ssd-driver/documentations/CH1115.pdf`).

Try to have own implementation with maximum shared code, but without timelife.

Update base code of example.

Add an example for this driver.

Update `README.md`.

## What was created

- `src/ch1115/mod.rs` — Module declaration and public re-exports for CH1115 driver
- `src/ch1115/command.rs` — CH1115-specific enums: PumpVoltageCh1115, IrefResistorCh1115, IrefSourceCh1115, ScrollIntervalCh1115, ScrollDirectionCh1115, ScrollModeCh1115, BreathingBrightnessCh1115, ComScanDirectionCh1115, SegPadsConfigCh1115
- `src/ch1115/display.rs` — CH1115 display driver struct with sync/async APIs (init, flush, contrast, pump voltage, IREF, VCOM, DC-DC, breathing light, horizontal scroll, SEG pads config, adaptive power save) and embedded-graphics DrawTarget implementation
- `examples/esp32-wroom-32/ch1115/` — Complete ESP32-WROOM example (Cargo.toml, .cargo/config.toml, rust-toolchain.toml, build.rs, src/main.rs)
- `examples/stm32-f1/ch1115/` — Complete STM32F103 "Blue Pill" example (Cargo.toml, src/main.rs)

## What was updated

- `src/lib.rs` — Added `pub mod ch1115` and public re-exports for all CH1115 types
- `README.md` — Added CH1115 to controller list, feature count, supported controllers table, examples listing, and InitConfig documentation
- `QUICK_START.md` — Added CH1115 over I2C quick start example
- `SPECIFIC-API-DRIVERS.md` — Added CH1115-Specific API section documenting all driver-specific methods
- `src/ssd1300/display.rs` — Fixed pre-existing `cargo fmt` issue in `set_dc_dc` method signature
