## Rust tutorial
Cargo.lock and Cargo.toml files are intentionally excluded.

1. Clone the repo.
2. Open the terminal in the root directory and type
```bash
cargo init
cargo build
```
3. Modify the cargo.toml file like the following to run different crates(files) as binaries
```toml
[package]
name = "rust-tutorial"
version = "0.1.0"
edition = "2021"

[dependencies]

[[bin]]
name = "default"
path = "src/22.rs"

[[bin]]
name = "binary1"
path = "src/23.rs"
```
4. run a particular binary lie this
```bash
cargo run --bin binary1
```