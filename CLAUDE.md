# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project overview

Platform-independent `no_std` Rust crate for driving OLED displays using SSD1306/SSD1306B controllers over I2C or SPI. The crate must work on any embedded target (ESP32, STM32, etc.) without heap allocation.

## Tech stack constraints

All code must be compatible with:
- `embedded-graphics = "0.8.2"` — drawing primitives and `DrawTarget` trait
- `embedded-hal = "1.0.0"` — hardware abstraction (I2C, SPI traits)
- `embedded-hal-bus = "0.3.0"` — shared bus management

The crate must support both sync and async usage. The main user-facing struct should minimize lifetime constraints.

## Build commands

```bash
cargo build                          # Build the library
cargo build --features async         # Build with async support (when available)
cargo test                           # Run tests (host-target unit tests)
cargo test <test_name>               # Run a single test
cargo clippy -- -D warnings          # Lint
cargo fmt --check                    # Check formatting
cargo doc --no-deps --open           # Generate and view docs
```

## Project structure

```
documentations/        # SSD1306 datasheets (reference for all register/command code)
src/                   # Library source (no_std)
features/              # Feature specs (numbered, with status tracking)
examples/
    esp32-wroom-32/    # Full example for ESP32-WROOM (separate Cargo project)
    stm32-f1/          # Full example for STM32F1 "Blue Pill" (separate Cargo project)
```

## Architecture guidelines

- Every command/register interaction must reference the relevant datasheet section in a comment (e.g., `// SSD1306 §8.1.5 — Set Display ON`).
- The driver struct is generic over the bus trait (`embedded_hal::i2c::I2c` or `embedded_hal::spi::SpiDevice`) so it remains platform-independent.
- Implement `embedded_graphics::draw_target::DrawTarget` to allow users to draw with the embedded-graphics ecosystem.
- No heap allocation (`#![no_std]`, no `alloc`).

## ESP32 examples requirements

Each ESP32 example (`examples/esp32-wroom-32/*/`) requires these additional files beyond `Cargo.toml` and `src/main.rs`:

- `.cargo/config.toml` — target configuration (xtensa-esp32-none-elf), runner, rustflags, build-std
- `.clippy.toml` — clippy configuration (stack-size-threshold)
- `build.rs` — linker script setup and error-handling hints
- `rust-toolchain.toml` — toolchain channel (`esp`)

Copy these from an existing ESP32 example (e.g., `ssd1306/`) when creating a new one.

## Workflow conventions

- All code changes are produced by the AI agent.
- Feature work follows specs in `features/` (check status field).
- Commits follow [Conventional Commits](https://www.conventionalcommits.org/) in English.
- After implementing a feature, add `## What was created` and `## What was updated` sections to the feature file listing all new and modified files with a short description of each.
- After compiling an example, remove its `target/` folder. Also remove the root `target/` folder after building/testing the library.