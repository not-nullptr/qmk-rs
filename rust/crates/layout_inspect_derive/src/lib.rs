use syn::{parse_macro_input, Data, DeriveInput};

mod structs;
use structs::derive_struct;
mod enums;
use enums::derive_enum;
mod attrs;
mod rename;

#[proc_macro_derive(Inspect, attributes(serde))]
pub fn inspect(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	inspect_impl(input).into()
}

fn inspect_impl(input: DeriveInput) -> proc_macro2::TokenStream {
	match input.data {
		Data::Struct(data) => derive_struct(data, input.ident, input.generics, input.attrs),
		Data::Enum(data) => derive_enum(data, input.ident, input.generics, input.attrs),
		Data::Union(_data) => todo!("Deriving `Inspect` on Unions not supported"),
	}
}
