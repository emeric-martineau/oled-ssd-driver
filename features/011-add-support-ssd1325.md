# 011 - Add support of oled driver ssd1325

## Status: DONE

## Description

Update code to add support of SSD1325 driver (datasheet: `oled-ssd-driver/documentations/SSD1325.pdf`).

Try to have own implementation with maximum shared code, but without timelife.

Update base code of example.

Add an example for this driver.

Update `README.md`.

## What was created

- `src/display_1325.rs` — `Ssd1325<DI>` driver struct (128x80, 4-bit grayscale / 16 levels, 5120-byte buffer), standalone implementation (not using shared `DisplayCore`), `DrawTarget` for `Gray4`, nibble-based pixel packing (2 pixels per byte)
- `src/command_1325.rs` — SSD1325-specific types: `CurrentRange1325` (Quarter/Half/Full), `DisplaySize1325`, `SIZE_128X80`, `InitConfig1325` (with re-map byte builder for column/nibble/address/COM/split options)
- `tests/display_1325_test.rs` — 28 unit tests covering pixel packing, init commands, flush, contrast, inverse, current range, VCOMH, precharge voltage, display on/off, remap options
- `examples/esp32-wroom-32/ssd1325/` — ESP32-WROOM example using `Gray4` grayscale drawing
- `examples/stm32-f1/ssd1325/` — STM32F103 example using `Gray4` grayscale drawing

## What was updated

- `src/lib.rs` — added module and re-exports for SSD1325
- `README.md` — added SSD1325 to controller list (now 9 controllers), updated framebuffer description, added table row, quick start section, API section, and examples listing
