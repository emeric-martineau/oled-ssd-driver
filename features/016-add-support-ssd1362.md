# 016 - Add support of oled driver ssd1362

## Status: DONE

## Description

Update code to add support of SSD1362 driver (datasheet: `oled-ssd-driver/documentations/SSD1362.pdf`).

Try to have own implementation with maximum shared code, but without timelife.

Update base code of example.

Add an example for this driver.

Update `README.md`.

## What was created

- `src/ssd1362/mod.rs` — Module declaration, re-exports `Ssd1362`, `DisplaySize1362`, `InitConfig`, `SIZE_256X64`
- `src/ssd1362/init.rs` — `DisplaySize1362` struct, `SIZE_256X64` constant, `InitConfig` with all SSD1362-specific fields (column_remap, nibble_remap, vertical_address_increment, com_remap, seg_split_odd_even, seg_left_right_remap, internal_iref, etc.) and `remap_byte()` helper
- `src/ssd1362/display.rs` — `Ssd1362<DI>` driver struct with 8192-byte framebuffer, sync/async init, flush, display_on, set_contrast, set_inverse, set_vcomh, set_precharge_voltage, set_internal_vdd, set_iref methods, and `DrawTarget` implementation for `Gray4`
- `examples/esp32-wroom-32/ssd1362/` — Complete ESP32-WROOM example (Cargo.toml, src/main.rs, .cargo/config.toml, build.rs, rust-toolchain.toml)
- `examples/stm32-f1/ssd1362/` — Complete STM32F103 "Blue Pill" example (Cargo.toml, src/main.rs)

## What was updated

- `src/lib.rs` — Added `pub mod ssd1362` and re-exports (`Ssd1362`, `DisplaySize1362`, `InitConfig1362`, `SIZE_256X64`)
- `README.md` — Updated controller count, supported controllers table, framebuffer description, added SSD1362 usage examples, SSD1362-specific API section, and example listings
