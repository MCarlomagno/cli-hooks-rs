use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, Item, ItemFn};
use std::path::PathBuf;

const DEFAULT_HOOKS_PATH: &str = ".hooks";
const PRE_HOOK_NAME: &str = "pre.rs";
const POST_HOOK_NAME: &str = "post.rs";

fn extract_function_names(content: &TokenStream) -> Vec<String> {
    let file = syn::parse2::<syn::File>(content.clone().into())
        .expect("Failed to parse file content");
    
    file.items.iter()
        .filter_map(|item| {
            if let Item::Fn(func) = item {
                Some(func.sig.ident.to_string())
            } else {
                None
            }
        })
        .collect()
}


#[proc_macro_attribute]
pub fn with_hooks(_: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let sig = &input.sig;
    let block = &input.block;
    let fn_name = &sig.ident;

    let pre_hook = {
        let hook_path = PathBuf::from(DEFAULT_HOOKS_PATH).join(PRE_HOOK_NAME);
        let hook_path_str = hook_path.to_str().unwrap();

        // Read and parse the content outside quote!
        let hook_content = if let Ok(content) = std::fs::read_to_string(hook_path_str) {
            let tokens = content.parse::<TokenStream>()
                .expect(&format!("Failed to parse pre-hook file: {}", hook_path_str));
            proc_macro2::TokenStream::from(tokens)
        } else {
            panic!("Hook file not found: {}", hook_path_str);
        };

        let function_names = extract_function_names(&hook_content);
        let function_calls = function_names.iter()
            .map(|name| format_ident!("{}", name));
        
        quote! {
            #hook_content
            #(#function_calls();)*
        }
    };

    let post_hook = {
        let hook_path = PathBuf::from(DEFAULT_HOOKS_PATH).join(POST_HOOK_NAME);
        let hook_path_str = hook_path.to_str().unwrap();

        // Read and parse the content outside quote!
        let hook_content = if let Ok(content) = std::fs::read_to_string(hook_path_str) {
            let tokens = content.parse::<TokenStream>()
                .expect(&format!("Failed to parse post-hook file: {}", hook_path_str));
            proc_macro2::TokenStream::from(tokens)
        } else {
            panic!("Hook file not found: {}", hook_path_str);
        };

        let function_names = extract_function_names(&hook_content);
        let function_calls = function_names.iter()
            .map(|name| format_ident!("{}", name));
        
        quote! {
            #hook_content
            #(#function_calls();)*
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