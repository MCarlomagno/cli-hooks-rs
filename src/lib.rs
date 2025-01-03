use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, parse::Parse, parse::ParseStream, LitStr};

struct HookPaths {
    pre_hook: Option<String>,
    post_hook: Option<String>,
}

impl Parse for HookPaths {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut pre_hook = None;
        let mut post_hook = None;

        if !input.is_empty() {
            pre_hook = Some(input.parse::<LitStr>()?.value());
            if input.peek(syn::Token![,]) {
                let _ = input.parse::<syn::Token![,]>();
                post_hook = Some(input.parse::<LitStr>()?.value());
            }
        }

        Ok(HookPaths { pre_hook, post_hook })
    }
}

#[proc_macro_attribute]
pub fn with_hooks(attr: TokenStream, item: TokenStream) -> TokenStream {
    let paths = parse_macro_input!(attr as HookPaths);
    let input = parse_macro_input!(item as ItemFn);

    let sig = &input.sig;
    let block = &input.block;
    let fn_name = &sig.ident;

    let pre_hook = if let Some(path) = paths.pre_hook {
        let content = std::fs::read_to_string(&path)
            .expect(&format!("Failed to read pre-hook file: {}", path));
        let content_tokens = content.parse::<TokenStream>()
            .expect(&format!("Failed to parse pre-hook file: {}", path));
        let content_tokens = proc_macro2::TokenStream::from(content_tokens);
        
        quote! {
            println!("Executing pre-hook from {}", #path);
            #content_tokens
        }
    } else {
        quote! { println!("Before executing {}", stringify!(#fn_name)); }
    };

    let post_hook = if let Some(path) = paths.post_hook {
        let content = std::fs::read_to_string(&path)
            .expect(&format!("Failed to read post-hook file: {}", path));
        let content_tokens = content.parse::<TokenStream>()
            .expect(&format!("Failed to parse post-hook file: {}", path));
        let content_tokens = proc_macro2::TokenStream::from(content_tokens);
        
        quote! {
            println!("Executing post-hook from {}", #path);
            #content_tokens
        }
    } else {
        quote! { println!("After executing {}", stringify!(#fn_name)); }
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
