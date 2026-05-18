# 017 - Add support of oled driver ssd1332

## Status: DONE

## Description

Update code to add support of SSD1332 driver (datasheet: `oled-ssd-driver/documentations/SSD1332.pdf`).

Try to have own implementation with maximum shared code, but without timelife.

Update base code of example.

Add an example for this driver.

Update `README.md`.

## What was created

- `src/ssd1332/mod.rs` — Module root, re-exports driver struct and config types
- `src/ssd1332/init.rs` — DisplaySize1332, InitConfig struct with remap/master config byte builders
- `src/ssd1332/display.rs` — Ssd1332 driver struct with sync/async APIs, DrawTarget impl (Rgb565)
- `examples/esp32-wroom-32/ssd1332/` — Complete ESP32 example (Cargo.toml, src/main.rs, .cargo/config.toml, build.rs, rust-toolchain.toml)
- `examples/stm32-f1/ssd1332/` — Complete STM32F103 example (Cargo.toml, src/main.rs)

## What was updated

- `src/lib.rs` — Added `pub mod ssd1332` and re-exports (Ssd1332, DisplaySize1332, InitConfig1332, SIZE_96X64_1332)
- `README.md` — Updated controller count, supported controllers table, quick start examples, SSD1332-specific API section, examples list
