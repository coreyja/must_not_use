use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, ReturnType, Type};

pub fn handle_fn(mut input_fn: ItemFn) -> TokenStream {
    // Modify the return type to wrap it in MustNotUse
    if let ReturnType::Type(arrow, return_type) = &input_fn.sig.output {
        input_fn.sig.output = ReturnType::Type(
            *arrow,
            Box::new(Type::Verbatim(quote! { MustNotUse<#return_type> })),
        );
    }

    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let body = &input_fn.block;

    let output = quote! {
        // Our wrapper type and its implementation
        #[derive(Debug, Clone)]
        pub struct MustNotUse<T> {
            value: T,
        }

        impl<T> MustNotUse<T> {
            fn new(value: T) -> Self {
                Self { value }
            }
        }

        impl<T> std::ops::Deref for MustNotUse<T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                panic!("🔥 YOU USED A MUST-NOT-USE VALUE! SHAME! SHAME! SHAME! 🔥");
            }
        }

        impl<T> std::fmt::Display for MustNotUse<T> where T: std::fmt::Display {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use std::ops::Deref;

                let inner: &T = self.deref();
                write!(f, "{}", inner)
            }
        }

        // The modified function
        #vis #sig {
            let result = (|| #body)();

            MustNotUse::new(result)
        }
    };

    output.into()
}
