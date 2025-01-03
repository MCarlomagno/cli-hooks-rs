## CLI hooks for Rust

> ⚠️ **Warning**: This project is experimental and under active development. The API and functionality may undergo significant changes. Use with caution in production environments.

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

Usage with custom hooks:

```rust
// pre_hook.rs
pub fn hook() {
  let start_time = std::time::Instant::now();
  println!("Starting execution at {:?}", start_time);
  ()
}

// post_hook.rs
pub fn hook() {
  println!("Execution completed");
  ()
}

// main.rs
use cli_hooks_rs::with_hooks;
#[with_hooks("hooks/pre_hook.rs", "hooks/post_hook.rs")]
  fn expensive_operation() -> i32 {
  std::thread::sleep(std::time::Duration::from_secs(1));
  42
}
```
