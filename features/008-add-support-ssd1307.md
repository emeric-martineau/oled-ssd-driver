# 008 - Add support of oled driver ssd1307

## Status: DONE

## Description

Update code to add support of SSD1307 driver (datasheet: `oled-ssd-driver/documentations/SSD1307.pdf`).

Try to have own implementation with maximum shared code, but without timelife.

Update base code of example.

Add an example for this driver.

Update `README.md`.

## What was created

- `src/display_1307.rs` — `Ssd1307<DI>` driver struct (128x39, no charge pump — external VCC 7-15V, external IREF, 3 VCOMH levels, horizontal scroll)
- `tests/display_1307_test.rs` — unit tests for SSD1307 driver
- `examples/esp32-wroom-32/ssd1307/` — ESP32-WROOM example
- `examples/stm32-f1/ssd1307/` — STM32F103 example

## What was updated

- `src/lib.rs` — added module and re-exports for SSD1307
- `README.md` — added SSD1307 documentation and quick start section