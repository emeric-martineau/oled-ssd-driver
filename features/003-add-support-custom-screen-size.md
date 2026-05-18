# 003 - Add support custom screen size

## Status: DONE

## Description

I have a screen with driver SSD1306. Size is 128x32. Device mode 1/32DUTY.

I would like for any driver add a custom screen siez, always withou lifetime.

## What was created

- `tests/display_size_test.rs` — unit tests for custom screen sizes (128x32, 96x16, custom)
- `examples/esp32-wroom-32/ssd1306-128x32/` — ESP32-WROOM example for 128x32 display
- `examples/stm32-f1/ssd1306-128x32/` — STM32F103 example for 128x32 display

## What was updated

- `src/display_common.rs` — added `DisplaySize` struct with predefined sizes (`SIZE_128X32`, `SIZE_96X16`), `new_with_size()` constructors for all drivers
- `src/display_1306.rs` — added `Ssd1306::new_with_size()` and size-aware init (multiplex ratio, COM pin config)
- `src/display_1306b.rs` — added `Ssd1306b::new_with_size()` and size-aware init
- `README.md` — documented custom screen size support