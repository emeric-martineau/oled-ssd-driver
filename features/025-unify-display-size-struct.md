# 025 - Unify DisplaySize struct

## Status: DONE

## Description

The default `DisplaySize` declared in file `src/common.rs` is many time replace by another struct like:
- DisplaySize1320
- DisplaySize1325
- DisplaySize1327
- DisplaySize1331
- DisplaySize1332
- DisplaySize1351
- DisplaySize1362

Change all codes to use only `DisplaySize` and at runtime, when get column or row, apply specific compute (e.g. self.width / 2)

## What was created

- None.

## What was updated

- `src/common.rs` — widened `DisplaySize.width` to `u16` so it can represent 256-pixel displays.
- `src/ssd1320/init.rs`, `src/ssd1325/init.rs`, `src/ssd1327/init.rs`, `src/ssd1331/init.rs`, `src/ssd1332/init.rs`, `src/ssd1351/init.rs`, `src/ssd1362/init.rs` — use shared `DisplaySize` for their size constants; removed controller-specific size structs.
- `src/ssd1320/display.rs`, `src/ssd1325/display.rs`, `src/ssd1327/display.rs`, `src/ssd1331/display.rs`, `src/ssd1332/display.rs`, `src/ssd1351/display.rs`, `src/ssd1362/display.rs` — use shared `DisplaySize` and derive controller RAM columns from pixel width where required.
- `src/ssd1320/mod.rs`, `src/ssd1325/mod.rs`, `src/ssd1327/mod.rs`, `src/ssd1331/mod.rs`, `src/ssd1332/mod.rs`, `src/ssd1351/mod.rs`, `src/ssd1362/mod.rs`, `src/lib.rs` — removed exports for controller-specific size types.
- `src/common.rs`, `src/ch1115/display.rs`, `src/ch1116/display.rs`, `src/ssd1306b/display.rs`, `src/ssd1312/display.rs`, `src/ssd1315/display.rs` — adapted shared width handling for the widened width field.
