use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, ReturnType, Type};

use crate::shared::create_panic_on_use;

pub fn handle_fn(mut input_fn: ItemFn) -> TokenStream {
    // Modify the return type to wrap it in MustNotUse
    if let ReturnType::Type(arrow, return_type) = &input_fn.sig.output {
        input_fn.sig.output = ReturnType::Type(
            *arrow,
            Box::new(Type::Verbatim(quote! { PanicOnUse<#return_type> })),
        );
    }

    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let body = &input_fn.block;

    let wrapper = create_panic_on_use();

    let output = quote! {
        // Our wrapper type and its implementation
        #wrapper

        // The modified function
        #vis #sig {
            let result = (|| #body)();

            PanicOnUse::new(result)
        }
    };

    output.into()
}
