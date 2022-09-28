# Rust-Book-Training
My personal experimentation while reading the Rust Book

## Getting started
### 1.2 Hello world
main.rs:
```rust
fn main() {
    println!("Hello World!");
}
```
Format it:
```bash
rustfmt main.rs
```
Compile it:
```bash
rustc main.rs
```

### 1.3 Hello Cargo

```bash
cargo new hello_cargo
cd hello_cargo

# Debug
cargo check
cargo build
cargo run

#Release
cargo build --release
cargo run --release
```

