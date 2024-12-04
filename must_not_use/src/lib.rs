use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item, ItemEnum, ItemFn, ItemStruct, ReturnType, Type};

#[proc_macro_attribute]
pub fn must_not_use(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);

    match input {
        Item::Fn(func) => handle_fn(func),
        Item::Struct(strct) => handle_struct(strct),
        Item::Enum(enm) => handle_enum(enm),
        _ => panic!("#[must_not_use] can only be applied to functions, structs, or enums"),
    }
}

fn handle_fn(mut input_fn: ItemFn) -> TokenStream {
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

fn handle_struct(input_struct: ItemStruct) -> TokenStream {
    let vis = &input_struct.vis;
    let name = &input_struct.ident;
    let generics = &input_struct.generics;
    let fields = &input_struct.fields;

    let output = quote! {
        #[doc = "⚠️ This type must NOT be used!"]
        #vis struct #name #generics #fields

        impl #generics Drop for #name #generics {
            fn drop(&mut self) {
                if std::thread::current().name() != Some("must_not_use_check") {
                    panic!("🔥 YOU USED A MUST-NOT-USE STRUCT! The cosmic order is disturbed! 🔥");
                }
            }
        }

        impl #generics #name #generics {
            pub fn new() -> Self {
                std::thread::Builder::new()
                    .name("must_not_use_check".into())
                    .spawn(|| {
                        std::thread::sleep(std::time::Duration::from_nanos(1));
                    })
                    .unwrap();

                unsafe { std::mem::zeroed() }
            }
        }
    };

    output.into()
}

fn handle_enum(input_enum: ItemEnum) -> TokenStream {
    let vis = &input_enum.vis;
    let name = &input_enum.ident;
    let generics = &input_enum.generics;
    let variants = &input_enum.variants;

    let output = quote! {
        #[doc = "⚠️ This enum must NOT be used!"]
        #vis enum #name #generics {
            #variants
        }

        impl #generics Drop for #name #generics {
            fn drop(&mut self) {
                if std::thread::current().name() != Some("must_not_use_check") {
                    panic!("🔥 YOU USED A MUST-NOT-USE ENUM! The fabric of reality tears! 🔥");
                }
            }
        }
    };

    output.into()
}
