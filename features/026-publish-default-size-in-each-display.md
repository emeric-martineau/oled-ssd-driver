# 026 - Publish default size for each display

## Status: DONE

## Description

For each display, crate pubish a default size. 
The name of default size is named by size e.g.: `SIZE_256X64`.

Publish a `DEFAULT_DISPLAY_SIZE` const instead of `SIZE_XXXXXX`.
If many default size is publish, use append at end of name what is change.

From `lib.rs`, many display size are published:
```rust
pub use common::{
    Buffered, DisplaySize, InitConfig, Unbuffered, SIZE_104X48, SIZE_128X32, SIZE_128X39,
    SIZE_128X64, SIZE_132X64, SIZE_64X48, SIZE_96X16,
};
```

Remove `SIZE_XXX` and use (or republish) `DEFAULT_DISPLAY_SIZE`.

## What was created

- `src/ssd1306/init.rs`, `src/ssd1307/init.rs`, `src/ssd1309/init.rs`, `src/ssd1312/init.rs`, `src/ssd1315/init.rs`, `src/ssd1306b/init.rs`, `src/ch1115/init.rs`, and `src/ch1116/init.rs` — default size constants for these controllers.

## What was updated

- All controller `init.rs` and `mod.rs` files — expose each display's default as `DEFAULT_DISPLAY_SIZE` from its controller module.
- All display constructor files — use their controller's published default size.
- `src/common.rs` and `src/lib.rs` — remove the common default size constants from the crate root; retain generic alternate size presets.
- `src/ssd1306b/display.rs` and `src/ssd1306b/mod.rs` — add the SSD1306B default size export.
- `tests/display_size_test.rs` — refer to the SSD1306 default through its controller module.
