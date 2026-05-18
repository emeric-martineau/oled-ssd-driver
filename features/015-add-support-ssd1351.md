# 015 - Add support of oled driver ssd1351

## Status: DONE

## Description

Update code to add support of SSD1351 driver (datasheet: `oled-ssd-driver/documentations/SSD1351.pdf`).

Try to have own implementation with maximum shared code, but without timelife.

Update base code of example.

Add an example for this driver.

Update `README.md`.

## What was created

- `src/ssd1351/mod.rs` — Module declaration and public re-exports for SSD1351
- `src/ssd1351/init.rs` — DisplaySize1351 struct, SIZE_128X128/SIZE_128X96 constants, InitConfig with remap_byte() builder
- `src/ssd1351/display.rs` — Ssd1351 driver struct with 32KB framebuffer, sync/async init/flush, DrawTarget impl (Rgb565), command APIs (contrast, master contrast, VCOMH, precharge, inverse, display on/off)
- `examples/esp32-wroom-32/ssd1351/` — Complete ESP32 SPI example with all required files (.cargo/config.toml, build.rs, rust-toolchain.toml, Cargo.toml, src/main.rs)

## What was updated

- `src/lib.rs` — Added `pub mod ssd1351` and re-exports (Ssd1351, DisplaySize1351, InitConfig1351, SIZE_128X128_COLOR, SIZE_128X96_COLOR)
- `README.md` — Updated controller count, supported controllers table, added SSD1351 Quick Start section, added SSD1351-Specific API section, updated InitConfig note, updated examples listing
