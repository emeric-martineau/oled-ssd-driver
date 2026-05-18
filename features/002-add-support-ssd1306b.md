# 002 - Add support of oled driver ssd1306b

## Status: DONE

## Description

Update code to add support of SSD1306B driver (datasheet: `oled-ssd-driver/documentations/SSD1306B_rev1.1.pdf`).

Try to have own implementation with maximum shared code, but without timelife.

## What was created

- `src/display_1306b.rs` — `Ssd1306b<DI>` driver struct with SSD1306B-specific features: 4 charge pump voltages, internal IREF, 4 VCOMH levels, fade/blink, zoom, content scroll, column-bounded scrolling
- `src/command_1306b.rs` — SSD1306B-specific types: `ChargePumpVoltage`, `FadeMode`, `IrefSetting`, `ScrollIntervalB`, `VcomhLevelB`
- `tests/display_1306b_test.rs` — unit tests for SSD1306B driver
- `examples/esp32-wroom-32/ssd1306b/` — ESP32-WROOM example
- `examples/stm32-f1/ssd1306b/` — STM32F103 example

## What was updated

- `src/lib.rs` — added module and re-exports for SSD1306B
- `README.md` — added SSD1306B documentation, quick start, and API section