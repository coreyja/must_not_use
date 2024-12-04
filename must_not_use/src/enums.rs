use crate::shared::create_panic_on_use;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Field, Fields, FieldsNamed, Ident, Item, ItemEnum, Variant};

pub fn handle_enum(input_enum: ItemEnum) -> TokenStream {
    let vis = &input_enum.vis;
    let name = &input_enum.ident;
    let generics = &input_enum.generics;

    // Process each variant to wrap their fields in PanicOnUse
    let variants = input_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        match &variant.fields {
            // Named fields like Variant { x: T, y: U }
            Fields::Named(named) => {
                let fields = named.named.iter().map(|f| {
                    let name = &f.ident;
                    let ty = &f.ty;
                    quote! { #name: PanicOnUse<#ty> }
                });
                quote! { #variant_name { #(#fields,)* } }
            }
            // Tuple fields like Variant(T, U)
            Fields::Unnamed(unnamed) => {
                let fields = unnamed.unnamed.iter().map(|f| {
                    let ty = &f.ty;
                    quote! { PanicOnUse<#ty> }
                });
                quote! { #variant_name(#(#fields),*) }
            }
            // Unit variants like Variant
            Fields::Unit => quote! { #variant_name(PanicOnUse<()>) },
        }
    });

    // Generate constructors for each variant
    let constructors = input_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let constructor_name = Ident::new(
            &format!("new_{}", variant_name.to_string().to_lowercase()),
            variant_name.span(),
        );

        match &variant.fields {
            Fields::Named(named) => {
                let field_names = named.named.iter().map(|f| &f.ident);
                let field_types = named.named.iter().map(|f| &f.ty);
                let field_assignments = named.named.iter().map(|f| &f.ident);

                quote! {
                    pub fn #constructor_name(#(#field_names: #field_types),*) -> Self {
                        Self::#variant_name {
                            #(#field_assignments: PanicOnUse::new(#field_assignments)),*
                        }
                    }
                }
            }
            Fields::Unnamed(unnamed) => {
                let field_types = unnamed.unnamed.iter().map(|f| &f.ty);
                let field_names = (0..unnamed.unnamed.len())
                    .map(|i| Ident::new(&format!("field_{}", i), variant_name.span()));
                let field_names2 = field_names.clone();

                quote! {
                    pub fn #constructor_name(#(#field_names: #field_types),*) -> Self {
                        Self::#variant_name(#(PanicOnUse::new(#field_names2)),*)
                    }
                }
            }
            Fields::Unit => quote! {
                pub fn #constructor_name() -> Self {
                    Self::#variant_name(PanicOnUse::new(()))
                }
            },
        }
    });

    let wrapper = create_panic_on_use();

    let output = quote! {
        #wrapper
        #vis enum #name #generics {
            #(#variants),*
        }

        impl #generics #name #generics {
            #(#constructors)*

            pub fn get_field<T>(&self, _field: &str) -> T {
                panic!("🔥 YOU TRIED TO ACCESS A FIELD OF A MUST-NOT-USE ENUM! 🔥")
            }
        }

        impl #generics Clone for #name #generics {
            fn clone(&self) -> Self {
                panic!("🔥 YOU TRIED TO CLONE A MUST-NOT-USE ENUM! 🔥")
            }
        }
    };

    output.into()
}
