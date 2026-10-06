## Rust examples

### How to
```
mkdir rust-examples
cd rust-examples

cargo init

cargo build
./target/debug/rust-examples

cargo build --release
./target/release/rust-examples

cargo build --bin hello
./target/debug/hello

cargo clean
```

Run/check:
```
cargo --list
cargo fmt
cargo clippy
cargo check
cargo run
cargo run --bin hello
cargo test
cargo test test_welcome2 # Run specific test
cargo bench
```

Add/remove dependencies:
```
cargo add ..
cargo remove ..
cargo tree
```

* [Package Layout](https://doc.rust-lang.org/cargo/guide/project-layout.html#package-layout)


## Links
* [Learn Rust](https://rust-lang.org/learn/)
* [Rust Developer](https://roadmap.sh/rust)
