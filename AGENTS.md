# AI Agent Rules (cargo-fastdev)

## Goal
Keep the tool safe: never corrupt Cargo.toml, always reversible.

## Non-negotiables
- Always run: `cargo fmt`, `cargo clippy -D warnings`, `cargo test`
- Any manifest edits must preserve formatting as much as possible (use toml_edit)
- Always keep a restore path (backup must exist before write)

## Safety requirements
- Do not overwrite backup if it already exists
- Never delete user data
- If parsing fails, exit with a clear error

## Tasks agent can do safely
- Add new presets behind explicit flags (no behavior change by default)
- Improve status JSON (additive fields only)
- Add tests and fixtures

## Do-not-do
- Do not change the default keys set without a migration note
- Do not touch member Cargo.toml files unless explicitly requested

## Quick commands
- Test: `cargo test`
- Lint: `cargo clippy -- -D warnings`
- Format: `cargo fmt`
