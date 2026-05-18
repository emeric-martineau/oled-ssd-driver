# 022 - Add STM32 examples for SSD0303 and SSD1300

## Status: DONE

## Description

Add STM32-F1 "Blue Pill" examples for the SSD0303 and SSD1300 OLED drivers.

- SSD0303: 132x64 monochrome display over I2C
- SSD1300: 104x48 monochrome display over SPI

## What was created

- `examples/stm32-f1/ssd0303/Cargo.toml` — Cargo manifest for the SSD0303 STM32 example
- `examples/stm32-f1/ssd0303/src/main.rs` — SSD0303 example using I2C (PB6/PB7), draws text, line, and circle on 132x64 display
- `examples/stm32-f1/ssd1300/Cargo.toml` — Cargo manifest for the SSD1300 STM32 example
- `examples/stm32-f1/ssd1300/src/main.rs` — SSD1300 example using SPI (PA5/PA7/PA4 CS, PB0 D/C, PB1 RES), draws text, line, and circle on 104x48 display
