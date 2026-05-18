# 013 - Refactoring  package

## Status: DONE

## Description

All drivers have 2 files:
- display_xxxx.rs
- command_xxxx.rs

Some drivers have also own `InitConfigXXXX` struct:
- Ssd1320
- Ssd1325
- Ssd1327

We want group `display_xxxx.rs` and `command_xxxx.rs` in sub Rust mod `xxxx`.

Drivers with `InitConfigXXXX` must have this sctruct in sub Rust mod `xxxx/init`.

## What was created

- `src/ssd1306/mod.rs` — module definition for SSD1306 (exposes `command` and `display`)
- `src/ssd1306b/mod.rs` — module definition for SSD1306B
- `src/ssd1307/mod.rs` — module definition for SSD1307
- `src/ssd1309/mod.rs` — module definition for SSD1309
- `src/ssd1312/mod.rs` — module definition for SSD1312
- `src/ssd1315/mod.rs` — module definition for SSD1315
- `src/ssd1320/mod.rs` — module definition for SSD1320 (with `init` submodule)
- `src/ssd1325/mod.rs` — module definition for SSD1325 (with `init` submodule)
- `src/ssd1327/mod.rs` — module definition for SSD1327 (with `init` submodule)
- `src/ch1116/mod.rs` — module definition for CH1116

## What was updated

- `src/lib.rs` — replaced flat `mod command_xxxx` / `mod display_xxxx` with `pub mod ssd1306`, `pub mod ssd1306b`, etc.; re-exports remain unchanged for public API compatibility
- `src/common.rs` — renamed from `display_common.rs`, updated import path from `crate::command_1306` to `crate::ssd1306::command`
- `src/ssd1306/display.rs` — moved from `display_1306.rs`, updated imports to use `super::command` and `crate::common`
- `src/ssd1306/command.rs` — moved from `command_1306.rs` (no import changes)
- `src/ssd1306b/display.rs` — moved from `display_1306b.rs`, updated imports
- `src/ssd1306b/command.rs` — moved from `command_1306b.rs`
- `src/ssd1307/display.rs` — moved from `display_1307.rs`, updated imports
- `src/ssd1309/display.rs` — moved from `display_1309.rs`, updated imports
- `src/ssd1309/command.rs` — moved from `command_1309.rs`
- `src/ssd1312/display.rs` — moved from `display_1312.rs`, updated imports
- `src/ssd1312/command.rs` — moved from `command_1312.rs`
- `src/ssd1315/display.rs` — moved from `display_1315.rs`, updated imports
- `src/ssd1315/command.rs` — moved from `command_1315.rs`
- `src/ssd1320/display.rs` — moved from `display_1320.rs`, updated imports to use `super::init`
- `src/ssd1320/init.rs` — moved from `command_1320.rs`
- `src/ssd1325/display.rs` — moved from `display_1325.rs`, updated imports to use `super::init`
- `src/ssd1325/init.rs` — moved from `command_1325.rs`
- `src/ssd1327/display.rs` — moved from `display_1327.rs`, updated imports to use `super::init`
- `src/ssd1327/init.rs` — moved from `command_1327.rs`
- `src/ch1116/display.rs` — moved from `display_ch1116.rs`, updated imports
- `src/ch1116/command.rs` — moved from `command_ch1116.rs`
