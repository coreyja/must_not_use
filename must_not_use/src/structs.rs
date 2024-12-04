use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Field, FieldsNamed, Ident, Item, ItemStruct};

use crate::shared::create_panic_on_use;

pub fn handle_struct(input_struct: ItemStruct) -> TokenStream {
    let vis = &input_struct.vis;
    let name = &input_struct.ident;
    let generics = &input_struct.generics;

    // Only handle named fields for this example
    let fields = match &input_struct.fields {
        syn::Fields::Named(named) => {
            let fields = named.named.iter().map(|f| {
                let name = &f.ident;
                let ty = &f.ty;
                quote! { #name: PanicOnUse<#ty> }
            });
            quote! { { #(#fields,)* } }
        }
        _ => panic!("Only named fields are supported"),
    };

    // Generate constructor method names for each field
    let named_fields = if let syn::Fields::Named(named) = &input_struct.fields {
        named
            .named
            .iter()
            .map(|f| f.ident.clone().unwrap())
            .collect::<Vec<Ident>>()
    } else {
        vec![]
    };

    let field_types = if let syn::Fields::Named(named) = &input_struct.fields {
        named.named.iter().map(|f| &f.ty).collect::<Vec<_>>()
    } else {
        vec![]
    };

    let constructor = quote! {
        impl #generics #name<#generics> {
            pub fn new(#(#named_fields: #field_types),*) -> Self {
                Self {
                    #(#named_fields: PanicOnUse::new(#named_fields)),*
                }
            }
        }
    };

    let wrapper = create_panic_on_use();

    let output = quote! {
        #wrapper

        #vis struct #name #generics #fields

        #constructor

        impl #generics #name<#generics> {
            pub fn get_field<T>(&self, _field: &str) -> T {
                panic!("🔥 YOU TRIED TO ACCESS A FIELD OF A MUST-NOT-USE STRUCT! 🔥")
            }
        }

        impl #generics Clone for #name<#generics> {
            fn clone(&self) -> Self {
                panic!("🔥 YOU TRIED TO CLONE A MUST-NOT-USE STRUCT! 🔥")
            }
        }
    };

    output.into()
}
