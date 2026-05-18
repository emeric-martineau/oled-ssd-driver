# 012 - Add support of oled driver ssd1327

## Status: DONE

## Description

Update code to add support of SSD1327 driver (datasheet: `oled-ssd-driver/documentations/SSD1327.pdf`).

Try to have own implementation with maximum shared code, but without timelife.

Update base code of example.

Add an example for this driver.

Update `README.md`.

## What was created

- `src/display_1327.rs` — `Ssd1327<DI>` driver struct (128x128, 4-bit grayscale / 16 levels, 8192-byte buffer), standalone implementation (not using shared `DisplayCore`), `DrawTarget` for `Gray4`, nibble-based pixel packing (2 pixels per byte)
- `src/command_1327.rs` — SSD1327-specific types: `DisplaySize1327`, `SIZE_128X128`, `InitConfig1327` (with re-map byte builder, function selection A/B, internal VDD regulator, second precharge, external VSL options)
- `tests/display_1327_test.rs` — 31 unit tests covering pixel packing, init commands, flush, contrast, inverse, VCOMH, precharge voltage, internal VDD, function selection B, display on/off, remap options
- `examples/esp32-wroom-32/ssd1327/` — ESP32-WROOM example using `Gray4` grayscale drawing
- `examples/stm32-f1/ssd1327/` — STM32F103 example using `Gray4` grayscale drawing

## What was updated

- `src/lib.rs` — added module and re-exports for SSD1327
- `README.md` — added SSD1327 to controller list (now 10 controllers), updated framebuffer description, added table row, quick start section, API section, and examples listing
