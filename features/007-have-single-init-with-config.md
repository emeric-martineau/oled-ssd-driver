# 007 - Have single init() method with config

## Status: DONE

## Description

Update code of all drivers to have only one method `pub fn init(&mut self)` (remove `init_with_orientation()`). This method accept a config struct that allow config (e.g. `DisplayOn`, segment_remap, com_scan_reversed).

Try to have a common config.

## What was created

- `src/display_common.rs` — `InitConfig` struct with configurable fields (segment_remap, com_scan_reversed, display_on, display_offset, display_start_line, addressing_mode, contrast, vcomh_deselect, entire_display_on, inverse_display, deactivate_scroll)

## What was updated

- `src/display_1306.rs` — replaced `init()` / `init_with_orientation()` with single `init(&self, config: &InitConfig)`
- `src/display_1306b.rs` — same refactor
- `src/display_1307.rs` — same refactor
- `src/display_1312.rs` — same refactor
- `src/display_1315.rs` — same refactor
- `src/display_ch1116.rs` — same refactor (ignores `addressing_mode` and `vcomh_deselect` fields)
- `src/lib.rs` — re-export `InitConfig`
- `README.md` — added `InitConfig` documentation table
- All examples updated to use `display.init(&Default::default())`