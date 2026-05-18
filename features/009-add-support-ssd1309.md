# 009 - Add support of oled driver ssd1309

## Status: DONE

## Description

Update code to add support of SSD1309 driver (datasheet: `oled-ssd-driver/documentations/SSD1309.pdf`).

Try to have own implementation with maximum shared code, but without timelife.

Update base code of example.

Add an example for this driver.

Update `README.md`.

## What was created

- `src/display_1309.rs` — `Ssd1309<DI>` driver struct (128x64, no charge pump — external VCC 7-18V, 3 VCOMH levels 0.64/0.78/0.84 x VCC, horizontal scroll)
- `src/command_1309.rs` — SSD1309-specific types: `VcomhLevel1309`
- `tests/display_1309_test.rs` — unit tests for SSD1309 driver
- `examples/esp32-wroom-32/ssd1309/` — ESP32-WROOM example
- `examples/stm32-f1/ssd1309/` — STM32F103 example

## What was updated

- `src/lib.rs` — added module and re-exports for SSD1309
- `README.md` — added SSD1309 documentation and quick start section