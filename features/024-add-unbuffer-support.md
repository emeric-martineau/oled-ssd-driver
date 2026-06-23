# 024 - Add support of unbuffering mode

## Status: DONE

## Description

Upgrade all code parts to allow support unbeffering mode throug `Config` structure.

# Feature 024 — Unbuffered Mode: Design Analysis

## What "unbuffered mode" means here

Every driver currently allocates a fixed-size framebuffer in RAM (e.g. 1 KB for 128×64 binary, 32 KB for SSD1351 color). Pixels are written to this buffer via `set_pixel()` / `DrawTarget::draw_iter()`, then the whole buffer is sent to the display GDDRAM on `flush()`.

In **unbuffered mode** the driver owns no RAM buffer. This saves significant RAM on microcontrollers (2–32 KB per driver instance).

---

## Driver categories

The 15 drivers fall into two groups with different constraints:

### Group A — Binary (monochrome) drivers using `DisplayCore<DI>`

These pack 8 vertical pixels per byte (page mode):

| Driver   | Struct field | Buffer size |
|----------|-------------|-------------|
| Ssd1306  | `core: DisplayCore<DI>` | 132×8 = 1056 B |
| Ssd1306b | `core: DisplayCore<DI>` | 132×8 = 1056 B |
| Ssd1307  | `core: DisplayCore<DI>` | 132×8 = 1056 B |
| Ssd1309  | `core: DisplayCore<DI>` | 132×8 = 1056 B |
| Ssd1312  | `core: DisplayCore<DI>` | 132×8 = 1056 B |
| Ssd1315  | `core: DisplayCore<DI>` | 132×8 = 1056 B |
| Ch1115   | `core: DisplayCore<DI>` | 132×8 = 1056 B |
| Ch1116   | `core: DisplayCore<DI>` | 132×8 = 1056 B |
| Ssd0303  | `core: DisplayCore<DI>` | 132×8 = 1056 B |
| Ssd1300  | `core: DisplayCore<DI>` | 132×8 = 1056 B |

**Critical constraint**: Setting one pixel requires a read-modify-write on the byte covering 8 rows in the same column. Without a local buffer there is no way to read back GDDRAM over I2C/SPI (write-only protocol). Therefore, in unbuffered mode for Group A drivers:
- `set_pixel()`, `clear_buffer()`, `buffer()`, and `DrawTarget` are **not available**.
- `flush()` is **not available**.
- All non-drawing APIs remain available: `init()`, `display_on()`, `set_contrast()`, `set_inverse()`, scroll commands, etc.

The user can still clear the display via the "entire display ON/OFF" command, but cannot draw arbitrary pixels.

### Group B — Color / grayscale drivers with standalone buffers

These own their buffer directly:

| Driver  | Pixel format | Buffer size |
|---------|-------------|-------------|
| Ssd1320 | 4-bit gray, 2 px/byte | 80×160 = 12 800 B |
| Ssd1325 | 4-bit gray, 2 px/byte | 64×80 = 5 120 B  |
| Ssd1327 | 4-bit gray, 2 px/byte | 64×128 = 8 192 B |
| Ssd1331 | RGB565, 2 B/px       | 96×64×2 = 12 288 B |
| Ssd1332 | RGB565, 2 B/px       | 96×64×2 = 12 288 B |
| Ssd1351 | RGB565, 2 B/px       | 128×128×2 = 32 768 B |
| Ssd1362 | 4-bit gray, 2 px/byte | 128×64 = 8 192 B |

**For RGB565 drivers (Ssd1331, Ssd1332, Ssd1351)**: Each pixel is a full 2-byte word independent of its neighbours. Unbuffered `DrawTarget` is fully implementable: for each pixel in `draw_iter`, set the column/row window to that one pixel and write 2 bytes directly.

**For 4-bit grayscale drivers (Ssd1320, Ssd1325, Ssd1327, Ssd1362)**: Two pixels share one byte (high/low nibble). Same read-modify-write problem as Group A, so unbuffered `DrawTarget` is not implementable. Same conclusion as Group A: no drawing API in unbuffered mode.

---

## API design

### Approach: type-level `BUFFERED` const generic

Add `const BUFFERED: bool = true` to each driver struct. When `false`, the buffer field is removed (zero-sized). This is the idiomatic `no_std` approach — no runtime branching, no phantom data, no heap.

```rust
// Buffered (default, current behaviour)
pub struct Ssd1306<DI> { core: DisplayCore<DI> }

// With generic (backward compatible via default)
pub struct Ssd1306<DI, const BUFFERED: bool = true> { ... }
```

In practice, since Rust stable const generics don't yet support conditional field inclusion, the cleanest approach is a **marker type parameter** with two unit structs:

```rust
pub struct Buffered;
pub struct Unbuffered;

pub struct Ssd1306<DI, MODE = Buffered> { ... }
```

Then `impl<DI: DisplayInterface> Ssd1306<DI, Buffered>` provides `set_pixel`, `flush`, `clear_buffer`, `DrawTarget`.  
And `impl<DI: DisplayInterface> Ssd1306<DI, Unbuffered>` provides only `init`, `display_on`, `set_contrast`, `set_inverse`, scrolling, etc.

`DisplayCore<DI>` in unbuffered mode drops its `buffer: [u8; BUFFER_SIZE]` field. The same marker approach applies there.

### New constructors

```rust
impl<DI> Ssd1306<DI, Unbuffered> {
    pub fn new_unbuffered(interface: DI) -> Self { ... }
    pub fn new_unbuffered_with_size(interface: DI, size: DisplaySize) -> Self { ... }
}
```

The "through Config structure" phrasing in the feature spec suggests the `InitConfig`/driver-specific config structs may also get a `buffered` field as an alternative route to select the mode, but the type-level approach is safer (compile error if you call `flush()` on an unbuffered driver vs. a runtime panic).

---

## Files that need changes

### Core

| File | Change |
|------|--------|
| `src/common.rs` | Make `DisplayCore<DI, MODE>` generic over `MODE`; add `DisplayCoreUnbuffered<DI>` without the buffer field (or use const generic), expose `new_unbuffered()` |

### Group A binary drivers (10 files)

For each of: `src/ssd1306/display.rs`, `src/ssd1306b/display.rs`, `src/ssd1307/display.rs`, `src/ssd1309/display.rs`, `src/ssd1312/display.rs`, `src/ssd1315/display.rs`, `src/ch1115/display.rs`, `src/ch1116/display.rs`, `src/ssd0303/display.rs`, `src/ssd1300/display.rs`:

- Add `MODE` type parameter with default `Buffered`
- New `impl<DI> DriverName<DI, Unbuffered>` block with `new_unbuffered()` constructors
- Keep all `impl<DI: DisplayInterface> DriverName<DI, Buffered>` blocks unchanged
- Add `impl<DI: DisplayInterface> DriverName<DI, Unbuffered>` with only non-drawing methods

### Group B color/gray drivers (7 files)

For each of: `src/ssd1320/display.rs`, `src/ssd1325/display.rs`, `src/ssd1327/display.rs`, `src/ssd1331/display.rs`, `src/ssd1332/display.rs`, `src/ssd1351/display.rs`, `src/ssd1362/display.rs`:

- Add `MODE` type parameter with default `Buffered`
- `new_unbuffered()` constructors that do not allocate the buffer constant
- For RGB565 drivers (Ssd1331, Ssd1332, Ssd1351): implement unbuffered `DrawTarget` that sends each pixel directly with a 1-pixel window command
- For 4-bit gray drivers (Ssd1320, Ssd1325, Ssd1327, Ssd1362): no drawing API in unbuffered mode (same constraint as Group A)

### Public API / lib.rs

- Export `Buffered`, `Unbuffered` marker types from `src/common.rs`
- Re-export them from `src/lib.rs`

---

## Backward compatibility

All existing code continues to compile without changes: the default type parameter `MODE = Buffered` preserves the current API surface exactly.

---

## What unbuffered mode does NOT support (all drivers)

- `set_pixel()`
- `clear_buffer()`
- `buffer()`
- `flush()` / `flush_async()`
- `DrawTarget` (except RGB565 drivers where direct pixel writes are possible)

## What unbuffered mode supports (all drivers)

- `new_unbuffered()` / `new_unbuffered_with_size()`
- `release()`
- `init()` / `init_async()`
- `display_on()` / `display_on_async()`
- `set_contrast()` / `set_contrast_async()`
- `set_inverse()` / `set_inverse_async()`
- All scroll commands
- All driver-specific commands (pump voltage, IREF, fade, zoom, etc.)
- `size_config()`
- `OriginDimensions` (returns display size for layout calculations without drawing)

---

## What was created

No new files were created.

## What was updated

- `src/common.rs` — Added `Buffered` and `Unbuffered` public marker types; added `DisplayBufferMode` sealed trait with `new_buffer()` associated function; made `DisplayCore<DI, MODE: DisplayBufferMode = Buffered>` generic; restricted `set_pixel`, `clear_buffer`, `active_buffer`, `flush`, and async flush to `impl<DI> DisplayCore<DI, Buffered>`; kept all non-drawing methods on `impl<DI: DisplayInterface, MODE: DisplayBufferMode> DisplayCore<DI, MODE>`.
- `src/lib.rs` — Added `Buffered` and `Unbuffered` to the public re-export from `common`.
- `src/ssd1306/display.rs` — Made generic over `MODE: DisplayBufferMode = Buffered`; added `Ssd1306<DI, Unbuffered>` constructor block and separate sync/async `impl` blocks for `Unbuffered` (without `flush`/drawing); made `OriginDimensions` mode-agnostic; restricted `DrawTarget` to `Buffered`.
- `src/ssd1306b/display.rs` — Same pattern as ssd1306; includes all SSD1306B-specific extended commands for both `Buffered` and `Unbuffered`.
- `src/ssd1307/display.rs` — Same pattern; default size `SIZE_128X39`.
- `src/ssd1309/display.rs` — Same pattern; adds `set_vcomh_deselect` for both modes.
- `src/ssd1312/display.rs` — Same pattern; includes charge pump, IREF, VCOMH, and scroll commands for both modes.
- `src/ssd1315/display.rs` — Same pattern; includes all SSD1315 extended commands for both modes.
- `src/ch1115/display.rs` — Same pattern; uses a module-level `ch1115_init_commands()` helper to avoid duplicating the long init sequence; page-mode flush; all CH1115-specific commands for both modes.
- `src/ch1116/display.rs` — Same pattern as CH1115; default size `SIZE_132X64`; all CH1116-specific commands for both modes.
- `src/ssd0303/display.rs` — Made generic over `MODE`; added `Ssd0303<DI, Unbuffered>` blocks; extracted `ssd0303_init_commands` and `ssd0303_init_commands_async` module-level helpers shared by both modes.
- `src/ssd1300/display.rs` — Same pattern as ssd0303; default size `SIZE_104X48`; added `ssd1300_init_commands` and `ssd1300_init_commands_async` helpers.
- `src/ssd1320/display.rs` — Added driver-local `Ssd1320BufferMode` trait (buffer size differs from `DisplayCore`); made `Ssd1320<DI, MODE: Ssd1320BufferMode = Buffered>` generic; added `Unbuffered` constructor and separate sync/async `impl` blocks; `DrawTarget` restricted to `Buffered`.
- `src/ssd1325/display.rs` — Same pattern with `Ssd1325BufferMode`; default size `SIZE_128X80`.
- `src/ssd1327/display.rs` — Same pattern with `Ssd1327BufferMode`; default size `SIZE_128X128`.
- `src/ssd1362/display.rs` — Same pattern with `Ssd1362BufferMode`; default size `SIZE_256X64`.
- `src/ssd1331/display.rs` — Added `Ssd1331BufferMode` trait; made generic; `DrawTarget` implemented for **both** `Buffered` (infallible, buffer write) and `Unbuffered` (fallible, per-pixel 1×1 window column/row write to GDDRAM).
- `src/ssd1332/display.rs` — Same pattern as ssd1331 with `Ssd1332BufferMode`.
- `src/ssd1351/display.rs` — Same pattern with `Ssd1351BufferMode`; unbuffered `DrawTarget` uses the SSD1351 two-step column/row address + write-RAM command sequence per pixel.
