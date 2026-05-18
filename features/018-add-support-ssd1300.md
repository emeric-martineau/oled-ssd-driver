# 018 - Add support of oled driver ssd1300

## Status: DONE

## Description

Update code to add support of SSD1300 driver (datasheet: `oled-ssd-driver/documentations/SSD1300.pdf`).

Try to have own implementation with maximum shared code, but without timelife.

Update base code of example.

Add an example for this driver.

Update `README.md`.

## What was created

- `src/ssd1300/mod.rs` — Module declaration and public re-exports for SSD1300
- `src/ssd1300/init.rs` — `InitConfig` struct, `ScrollInterval1300` enum, and `SIZE_104X48` constant
- `src/ssd1300/display.rs` — `Ssd1300` driver struct with sync/async APIs, page-mode flush, horizontal scroll, and `DrawTarget` implementation
- `examples/esp32-wroom-32/ssd1300/` — Complete ESP32-WROOM example (SPI, with .cargo/config.toml, build.rs, rust-toolchain.toml, Cargo.toml, src/main.rs)

## What was updated

- `src/lib.rs` — Added `pub mod ssd1300` and re-exports (`Ssd1300`, `InitConfig1300`, `ScrollInterval1300`)
- `src/common.rs` — Added `SIZE_104X48` display size constant
- `README.md` — Added SSD1300 to controller list, feature count, quick start example, SSD1300-specific API section, and examples listing
