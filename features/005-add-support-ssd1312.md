# 005 - Add support of oled driver ssd1312

## Status: DONE

## Description

Update code to add support of SSD1315 driver (datasheet: `oled-ssd-driver/documentations/SSD1312.pdf`).

Try to have own implementation with maximum shared code, but without timelife.

Update base code of example.

Add an example for this driver.

Update `README.md`.

## What was created

- `src/display_1312.rs` — `Ssd1312<DI>` driver struct with SSD1312-specific features: 4 charge pump voltages (7.5-10.0V), internal/external IREF, 4 VCOMH levels, content scroll, column-bounded scrolling
- `src/command_1312.rs` — SSD1312-specific types: `ChargePumpVoltage1312`, `IrefSetting1312`, `VcomhLevel1312`, `ScrollInterval1312`
- `tests/display_1312_test.rs` — unit tests for SSD1312 driver
- `examples/esp32-wroom-32/ssd1312/` — ESP32-WROOM example
- `examples/stm32-f1/ssd1312/` — STM32F103 example

## What was updated

- `src/lib.rs` — added module and re-exports for SSD1312
- `README.md` — added SSD1312 documentation, quick start, and API section