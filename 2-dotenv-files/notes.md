# notes for dotenv rust test

## links

- [dotenvy](https://crates.io/crates/dotenvy)
  - a dotenv implementation for rust, more current and maintained than dotenv

## commands used

Init project and add dotenvy dependency:

```bash
cargo init --name dotenv-test --edition 2021
cargo add dotenvy
```

Build and run:

```bash
cargo build --release
copy example.env target\release\.env
target\release\dotenv-test.exe
```

## notes

- copy `example.env` to `.env`
- the `.env` file needs to be in the same directory as the `Cargo.toml` file or the binary itself
