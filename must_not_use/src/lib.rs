use proc_macro::TokenStream;
use syn::{parse_macro_input, Item};

mod shared;

mod functions;
use functions::handle_fn;

mod structs;
use structs::handle_struct;

mod enums;
use enums::handle_enum;

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
