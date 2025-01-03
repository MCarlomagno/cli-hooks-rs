## CLI hooks for Rust

> ⚠️ **Warning**: This project is experimental and under active development. The API and functionality may undergo significant changes. Not ready for production environments.

A procedural macro that enables dynamic execution of Rust code before and after function calls. Unlike typical procedural macros that are limited to compile-time code generation, this crate allows you to:

- Execute arbitrary Rust code from external files at runtime
- Load different hook implementations without recompiling the CLI application

This is particularly useful for CLI applications where users need to customize command behavior by injecting their own code before or after execution. Common use cases include:

- Adding environment-specific logging
- Performing custom validation before commands
- Running cleanup operations after command execution
- Measuring performance metrics

## Setup

To build and test the project:
```
cargo build
cargo test -- --nocaption # this allows to see println in tests
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cli_hooks_rs = "0.1.0"
```

## Usage

See the complete working example in the [`example/`](example/) directory

Usage with custom hooks:

```rust
// .hooks/pre.rs (end user injected code)
fn check_pre() {
  println!("Starting execution");
}

// .hooks/post.rs (end user injected code)
fn check_post() {
  println!("Execution completed");
}

// main.rs  (CLI app)
use cli_hooks_rs::with_hooks;
#[with_hooks]
fn some_function() -> i32 {
  std::thread::sleep(std::time::Duration::from_secs(1));
  42
}
```
