# 014 - Add support of oled driver ssd1331

## Status: DONE

## Description

Update code to add support of SSD1331 driver (datasheet: `oled-ssd-driver/documentations/SSD1331.pdf`).

Try to have own implementation with maximum shared code, but without timelife.

Update base code of example.

Add an example for this driver.

Update `README.md`.

## What was created

- `src/ssd1331/mod.rs` — Module declaration for SSD1331 driver
- `src/ssd1331/init.rs` — Display size configuration and `InitConfig` struct with remap byte builder
- `src/ssd1331/display.rs` — Main driver struct with sync/async API, framebuffer (12288 bytes for 96x64 RGB565), and `DrawTarget` implementation for `Rgb565`
- `examples/esp32-wroom-32/ssd1331/` — Complete ESP32 example over SPI with hardware reset
- `examples/stm32-f1/ssd1331/` — Complete STM32F103 example over SPI with hardware reset

## What was updated

- `src/lib.rs` — Added `pub mod ssd1331` and re-exports (`Ssd1331`, `DisplaySize1331`, `InitConfig1331`, `SIZE_96X64`)
- `README.md` — Updated controller count, features list, supported controllers table, added SSD1331 quick start example, SSD1331-specific API section, and examples listing
