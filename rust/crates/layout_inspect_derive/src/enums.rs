use proc_macro2::TokenStream;
use quote::quote;
use syn::{
	parse_quote, Attribute, DataEnum, Expr, Fields, FieldsUnnamed, GenericParam, Generics, Ident, Lit,
};

use crate::{
	attrs::{get_serde_attrs, SerdeAttrs},
	rename::{get_ident_name, get_ser_name},
};

// TODO: Support generic enums e.g. `enum Maybe<T> { Some(T), Nothing }`
// TODO: Should `discriminant` be `i64` not `u64`?
// TODO: For fieldless enums, use e.g. `Foo::Bar as u64` to get discriminants.
// Discrimants can be defined as a const expression which we can't parse
// e.g. `enum X { Y = mem::size_of::<u32>() }`
// NB Only legal for enums where all variants are fieldless, or `#[repr(u8)]`.
// That includes e.g. `enum X { Tuple(), Struct{} }`.
// Also see: https://rust-lang.github.io/rfcs/2363-arbitrary-enum-discriminant.html
// Maybe possible to obtain discriminant values for field-ful enums
// if enum is `#[repr(u../i..)]`.

pub fn derive_enum(
	data: DataEnum,
	ident: Ident,
	mut generics: Generics,
	attrs: Vec<Attribute>,
) -> TokenStream {
	let mut next_discriminant: u64 = 0;

	let SerdeAttrs {
		rename: ser_name,
		rename_all,
		tag,
		content,
		untagged,
		..
	} = get_serde_attrs(&attrs, "enum");
	let ser_name = ser_name.unwrap_or_else(|| ident.to_string());

	let tag = if let Some(tag) = tag {
		if let Some(content) = content {
			quote! { DefEnumTag::TagAndContent {tag: #tag.to_string(), content: #content.to_string()} }
		} else {
			quote! { DefEnumTag::Tag(#tag.to_string()) }
		}
	} else if untagged {
		quote! { DefEnumTag::Untagged }
	} else {
		quote! { DefEnumTag::None }
	};

	let variant_defs: Vec<_> = data
		.variants
		.into_iter()
		.map(|variant| {
			let name = get_ident_name(&variant.ident);

			let (ser_value, value_type_id) = match variant.fields {
				Fields::Unit => {
					let SerdeAttrs {
						rename: ser_value, ..
					} = get_serde_attrs(&variant.attrs, "enum variant");

					// Get variant value, optionally applying `rename_all` transform.
					// `serde(rename)` on variant takes precedence.
					let ser_value = get_ser_name(&name, &ser_value, &rename_all);
					let ser_value = quote! { Some(#ser_value.to_string()) };
					let value_type_id = quote! { None };
					(ser_value, value_type_id)
				}
				Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
					assert!(unnamed.len() == 1);
					let ty = &unnamed.first().unwrap().ty;
					let ser_value = quote! { None };
					let value_type_id = quote! { Some(collector.collect::<#ty>()) };
					(ser_value, value_type_id)
				}
				Fields::Named(_) => todo!(),
			};

			let discriminant = match variant.discriminant {
				Some(discriminant) => {
					match discriminant.1 {
						Expr::Lit(expr_lit) => {
							match expr_lit.lit {
								Lit::Int(int) => int.base10_parse::<u64>().unwrap(),
								_ => todo!(),
							}
						}
						_ => todo!(),
					}
				}
				None => next_discriminant,
			};
			next_discriminant = discriminant + 1;

			quote! {
				DefEnumVariant {
					name: #name.to_string(),
					discriminant: #discriminant,
					ser_value: #ser_value,
					value_type_id: #value_type_id,
				}
			}
		})
		.collect();

	// Add bound `Inspect` to type params
	for param in &mut generics.params {
		if let GenericParam::Type(ref mut type_param) = *param {
			type_param.bounds.push(parse_quote!(Inspect));
		}
	}

	// Return `impl` code
	let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

	quote! {
		const _: () = {
			use ::std::{
				mem,
				option::Option::{self, None, Some},
				string::String,
				stringify, vec,
			};
			use ::layout_inspect::{
				defs::{DefEnum, DefEnumTag, DefEnumVariant, DefType},
				Inspect, TypesCollector,
			};

			#[automatically_derived]
			impl #impl_generics Inspect for #ident #type_generics #where_clause {
				fn name() -> String {
					stringify!(#ident).to_string()
				}

				fn size() -> Option<usize> {
					Some(mem::size_of::<Self>())
				}

				fn align() -> Option<usize> {
					Some(mem::align_of::<Self>())
				}

				fn def(collector: &mut TypesCollector) -> DefType {
					DefType::Enum(DefEnum {
						name: <Self as Inspect>::name(),
						ser_name: #ser_name.to_string(),
						size: <Self as Inspect>::size().unwrap(),
						align: <Self as Inspect>::align().unwrap(),
						variants: vec![#(#variant_defs),*],
						tag: #tag,
					})
				}
			}
		};
	}
}
