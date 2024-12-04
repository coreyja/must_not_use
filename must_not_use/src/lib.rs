use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item, ItemEnum, ItemFn, ItemStruct, ReturnType, Type};

mod shared;
use shared::create_panic_on_use;

mod functions;
use functions::handle_fn;

mod structs;
use structs::handle_struct;

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
