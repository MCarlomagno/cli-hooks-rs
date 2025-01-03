use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};
use std::path::PathBuf;

const HOOKS_PATH_ENV: &str = "CLI_HOOKS_PATH";
const DEFAULT_HOOKS_PATH: &str = ".hooks";
const PRE_HOOK_NAME: &str = "pre.rs";
const POST_HOOK_NAME: &str = "post.rs";

#[proc_macro_attribute]
pub fn with_hooks(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let sig = &input.sig;
    let block = &input.block;
    let fn_name = &sig.ident;

    let hooks_dir = std::env::var(HOOKS_PATH_ENV)
        .unwrap_or_else(|_| DEFAULT_HOOKS_PATH.to_string());

    let pre_hook = {
        let hook_path = PathBuf::from(&hooks_dir).join(PRE_HOOK_NAME);
        let hook_path_str = hook_path.to_str().unwrap();

        // Read and parse the content outside quote!
        let hook_content = if let Ok(content) = std::fs::read_to_string(hook_path_str) {
            let tokens = content.parse::<TokenStream>()
                .expect(&format!("Failed to parse pre-hook file: {}", hook_path_str));
            proc_macro2::TokenStream::from(tokens)
        } else {
            panic!("Hook file not found: {}", hook_path_str);
        };
        
        quote! {
            #hook_content
        }
    };

    let post_hook = {
        let hook_path = PathBuf::from(&hooks_dir).join(POST_HOOK_NAME);
        let hook_path_str = hook_path.to_str().unwrap();

        // Read and parse the content outside quote!
        let hook_content = if let Ok(content) = std::fs::read_to_string(hook_path_str) {
            let tokens = content.parse::<TokenStream>()
                .expect(&format!("Failed to parse post-hook file: {}", hook_path_str));
            proc_macro2::TokenStream::from(tokens)
        } else {
            panic!("Hook file not found: {}", hook_path_str);
        };
        
        quote! {
            #hook_content
        }
    };

    let output = quote! {
        #sig {
            #pre_hook
            let result = (|| #block)();
            #post_hook
            result
        }
    };

    output.into()
}