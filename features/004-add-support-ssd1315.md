# 004 - Add support of oled driver ssd1315

## Status: DONE

## Description

Update code to add support of SSD1315 driver (datasheet: `oled-ssd-driver/documentations/SSD1315.pdf`).

Try to have own implementation with maximum shared code, but without timelife.

Update base code of example.

Add an example for this driver.

Update `README.md`.

## What was created

- `src/display_1315.rs` — `Ssd1315<DI>` driver struct with SSD1315-specific features: 3 charge pump voltages, internal IREF, 4 VCOMH levels, fade/blink, zoom, content scroll, column-bounded scrolling
- `src/command_1315.rs` — SSD1315-specific types: `ChargePumpVoltage1315`, `IrefSetting1315`, `VcomhLevel1315`, `FadeMode1315`, `ScrollInterval1315`
- `tests/display_1315_test.rs` — unit tests for SSD1315 driver
- `examples/esp32-wroom-32/ssd1315/` — ESP32-WROOM example
- `examples/stm32-f1/ssd1315/` — STM32F103 example

## What was updated

- `src/lib.rs` — added module and re-exports for SSD1315
- `README.md` — added SSD1315 documentation, quick start, and API section