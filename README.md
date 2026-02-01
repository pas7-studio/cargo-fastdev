# cargo-fastdev

[![Ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/pas7studio)
[Donate via PayPal](https://www.paypal.com/ncp/payment/KDSSNKK8REDM8)

Fast Rust dev loop for real projects:
- `doctor` checks your local toolchain (sccache, mold, etc.)
- `init` generates optional `.cargo/config.toml` for faster builds
- `watch` reruns `cargo check/test/run` on file changes
- `check/test/run` wrappers apply opt-in fast env defaults

Website: https://pas7.com.ua/
LinkedIn: https://www.linkedin.com/company/pas7-studio

## Install
```bash
cargo install cargo-fastdev
```

## Quick start
```bash
cargo fastdev doctor
cargo fastdev init --print

# write .cargo/config.toml (opt-in)
cargo fastdev init --write

# fast loop
cargo fastdev watch check
cargo fastdev watch test
cargo fastdev watch run -- --bin my_app
```

## What it changes

By default, cargo-fastdev is conservative:

- It does not modify your project unless you use init --write.
- It uses opt-in flags for more aggressive settings (wrapper/flags).

## Commands

### doctor

Prints detected tooling and suggestions:

```bash
cargo fastdev doctor --format json
cargo fastdev doctor
```

### init

Generates .cargo/config.toml snippet:

```bash
cargo fastdev init --print
cargo fastdev init --write
cargo fastdev init --write --use-sccache --use-mold
```

### watch

Re-runs a cargo subcommand on changes:

```bash
cargo fastdev watch check
cargo fastdev watch test
cargo fastdev watch run -- --example hello
```

## CI usage

```yaml
- run: cargo install cargo-fastdev
- run: cargo fastdev doctor
- run: cargo fastdev check
```

## Support

Ko-fi: https://ko-fi.com/pas7studio

PayPal: https://www.paypal.com/ncp/payment/KDSSNKK8REDM8

## License

Apache-2.0. See [LICENSE](LICENSE) and [NOTICE](NOTICE).
Trademarks: see [TRADEMARKS.md](TRADEMARKS.md).
