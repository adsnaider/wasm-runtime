# Little Wasm Interpreter

## Current limitations

* Table instructions are not supported.
* Memory instructions are not supported.
* Global instructions are not supported.
* Some numeric instructions are not supported.

## Requirements

* [Rust/Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed.

## Usage

Currently the easiest way to test the repo is by using the test suite under the
tests/ directory. Running `cargo test` will run these tests. To try new things,
you can write your own custom .wat and compile it with a tool such as
[wat2wasm](https://github.com/WebAssembly/wabt/releases/tag/1.0.23)
