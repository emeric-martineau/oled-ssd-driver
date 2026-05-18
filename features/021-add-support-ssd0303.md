# 021 - Add support of oled driver ssd0303

## Status: DONE

## Description

Update code to add support of SSD0303 driver (datasheet: `oled-ssd-driver/documentations/SSD0303.pdf`).

Try to have own implementation with maximum shared code, but without timelife.

Update base code of example.

Add an example for this driver.

Update `README.md`.

## What was created

- `src/ssd0303/mod.rs` — Module root, re-exports `Ssd0303`, `InitConfig`, `ScrollInterval0303`, `SIZE_132X64_0303`
- `src/ssd0303/init.rs` — `InitConfig` struct with SSD0303-specific fields (DC-DC, VCOMH, pre-charge, COM pins, etc.), `ScrollInterval0303` enum, `SIZE_132X64_0303` constant
- `src/ssd0303/display.rs` — `Ssd0303<DI>` driver struct with sync/async init, flush (page-mode), set_contrast, set_inverse, set_vcomh, set_dc_dc, set_brightness, horizontal scroll, display on/off, and `DrawTarget` implementation
- `examples/esp32-wroom-32/ssd0303/` — Complete ESP32 example (I2C, 132x64 monochrome) with `.cargo/config.toml`, `build.rs`, `Cargo.toml`, `rust-toolchain.toml`, and `src/main.rs`

## What was updated

- `src/lib.rs` — Added `pub mod ssd0303` and re-exports (`Ssd0303`, `InitConfig0303`, `ScrollInterval0303`)
- `README.md` — Added SSD0303 to controller list, updated feature count, updated framebuffer description, updated InitConfig note, added example entry
- `QUICK_START.md` — Added SSD0303 over I2C quick start section
- `SPECIFIC-API-DRIVERS.md` — Added SSD0303-specific API reference section
