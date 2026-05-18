# 006 - Add support of oled driver ch1116

## Status: DONE

## Description

Update code to add support of ch1116 driver (datasheet: `oled-ssd-driver/documentations/CH1116.pdf`).

Try to have own implementation with maximum shared code, but without timelife.

Update base code of example.

Add an example for this driver.

Update `README.md`.

## What was created

- `src/display_ch1116.rs` — `Ch1116<DI>` driver struct (132x64, 1056-byte buffer, page-mode addressing), with CH1116-specific features: 4 pump voltages, internal/external IREF with 4 resistor values, breathing light, horizontal scroll with column bounds, scroll mode
- `src/command_ch1116.rs` — CH1116-specific types: `PumpVoltageCh1116`, `IrefSourceCh1116`, `IrefResistorCh1116`, `BreathingBrightnessCh1116`, `ScrollDirectionCh1116`, `ScrollIntervalCh1116`, `ScrollModeCh1116`
- `tests/display_ch1116_test.rs` — unit tests for CH1116 driver
- `examples/esp32-wroom-32/ch1116/` — ESP32-WROOM example
- `examples/stm32-f1/ch1116/` — STM32F103 example

## What was updated

- `src/lib.rs` — added module and re-exports for CH1116
- `README.md` — added CH1116 documentation, quick start, and API section