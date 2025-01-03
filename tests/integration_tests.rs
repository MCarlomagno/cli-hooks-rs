use cli_hooks_rs::with_hooks;
mod fixtures;

#[test]
fn test_with_hooks_with_rust_hooks() {
    #[with_hooks(
      "tests/fixtures/pre_hook.rs",
      "tests/fixtures/post_hook.rs" 
    )]
    fn test_fn() -> i32 {
        println!("{:?}", 42);
        42
    }

    let result = test_fn();
    assert_eq!(result, 42);
}

#[test]
fn test_with_hooks_default() {
    #[with_hooks]
    fn test_fn() -> i32 {
        println!("{:?}", 42);
        42
    }

    let result = test_fn();
    assert_eq!(result, 42);
}
