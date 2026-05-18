# Workflow

## Step-by-step debugging process

1. **Reproduce** — Identify the failing test or compilation error.
2. **Read datasheet** — Check the relevant SSD1306 datasheet section for expected behavior.
3. **Locate** — Find the relevant module in `src/`.
4. **Understand** — Read the surrounding code before making changes.
5. **Fix** — Apply the minimal change that resolves the issue.
6. **Test** — Run `cargo test` and `cargo clippy -- -D warnings` to verify.
7. **Commit** — Use a conventional commit message describing the fix.

## Adding a new feature

1. **Spec** — Check the feature spec in `features/` for requirements and status.
2. **Datasheet** — Read the relevant SSD1306 datasheet sections.
3. **Implement** — Add or update modules in `src/`, referencing datasheet sections in comments.
4. **Trait impl** — Ensure new functionality integrates with `embedded-hal` / `embedded-graphics` traits.
5. **Test** — Add unit tests, run `cargo test` and `cargo clippy -- -D warnings`.
6. **Example** — Update examples if the public API changed and compile only esp32 example it with `cargo build`. After compile ok or fix error, remove `target` folder of example.
7. **Example** — If add a new example, compile only esp32 example it with `cargo build`. After compile ok or fix error, remove `target` folder of example
8. **Commit** — Use a conventional commit message.
