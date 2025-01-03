use cli_hooks_rs::with_hooks;
mod fixtures;
use std::env;

#[test]
fn test_with_hooks_with_rust_hooks() {

    env::set_var("CLI_HOOKS_PATH", "tests/fixtures");

    #[with_hooks]
    fn test_fn() -> i32 {
        println!("{:?}", 42);
        42
    }

    let result = test_fn();
    assert_eq!(result, 42);

    env::remove_var("CLI_HOOKS_PATH");
}

#[test]
fn test_with_hooks_default() {

    env::set_var("CLI_HOOKS_PATH", "tests/fixtures");
    #[with_hooks]
    fn test_fn() -> i32 {
        println!("{:?}", 42);
        42
    }

    let result = test_fn();
    assert_eq!(result, 42);
    env::remove_var("CLI_HOOKS_PATH");
}
