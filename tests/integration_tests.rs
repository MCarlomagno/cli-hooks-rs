use cli_hooks::with_hooks;

#[test]
fn test_with_hooks_with_rust_hooks() {

    #[with_hooks]
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
    fn test_fn_2() -> i32 {
        println!("{:?}", 42);
        42
    }

    let result = test_fn_2();
    assert_eq!(result, 42);
}
