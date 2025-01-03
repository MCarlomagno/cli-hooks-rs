use cli_hooks::with_hooks;

fn main() {
    println!("Hello, world!");

    #[with_hooks]
    fn my_function() {
        println!("function body")
    }
    
    my_function();
}
