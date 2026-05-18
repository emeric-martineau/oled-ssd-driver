# 010 - Add support of oled driver ssd1320

## Status: DONE

## Description

Update code to add support of SSD1320 driver (datasheet: `oled-ssd-driver/documentations/SSD1320.pdf`).

Try to have own implementation with maximum shared code, but without timelife.

Update base code of example.

Add an example for this driver.

Update `README.md`.

## What was created

- `src/display_1320.rs` — `Ssd1320<DI>` driver struct (160x160, 4-bit grayscale / 16 levels, 12800-byte buffer), standalone implementation (not using shared `DisplayCore`), `DrawTarget` for `Gray4`, nibble-based pixel packing (2 pixels per byte)
- `src/command_1320.rs` — SSD1320-specific types: `AddressingMode1320`, `IrefSetting1320`, `DisplaySize1320`, `SIZE_160X160`, `InitConfig1320`
- `tests/display_1320_test.rs` — 22 unit tests covering pixel packing, init commands, flush, contrast, inverse, IREF, display on/off
- `examples/esp32-wroom-32/ssd1320/` — ESP32-WROOM example using `Gray4` grayscale drawing
- `examples/stm32-f1/ssd1320/` — STM32F103 example using `Gray4` grayscale drawing

## What was updated

- `src/lib.rs` — added module and re-exports for SSD1320
- `README.md` — added SSD1320 to controller list (now 8 controllers), updated framebuffer description, added table row, quick start section, API section, and examples listing