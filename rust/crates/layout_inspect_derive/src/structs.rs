use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
	parse_quote, Attribute, DataStruct, Field, Fields, FieldsNamed, FieldsUnnamed, GenericParam,
	Generics, Ident, Index, Member,
};

use crate::{
	attrs::{get_serde_attrs, SerdeAttrs},
	rename::{get_ident_name, get_ser_name},
};

pub fn derive_struct(
	data: DataStruct,
	ident: Ident,
	mut generics: Generics,
	attrs: Vec<Attribute>,
) -> TokenStream {
	let SerdeAttrs {
		rename: ser_name,
		rename_all,
		tag,
		transparent,
		..
	} = get_serde_attrs(&attrs, "struct");

	// Get field definitions
	let field_defs: Vec<TokenStream> = match data.fields {
		Fields::Named(fields) => get_named_field_defs(fields, &rename_all),
		Fields::Unnamed(fields) => get_unnamed_field_defs(fields, &rename_all),
		Fields::Unit => vec![],
	};

	// Add bound `Inspect` to type params
	for param in &mut generics.params {
		if let GenericParam::Type(ref mut type_param) = *param {
			type_param.bounds.push(parse_quote!(Inspect));
		}
	}

	// Create code for name
	let sub_types: Vec<TokenStream> = generics
		.params
		.iter()
		.filter_map(|param| {
			match param {
				GenericParam::Type(param) => {
					let ident = &param.ident;
					Some(quote! {&<#ident as Inspect>::name() +})
				}
				_ => None,
			}
		})
		.collect();

	let sub_types_str = if !sub_types.is_empty() {
		let mut sub_types: Vec<_> = sub_types
			.into_iter()
			.flat_map(|sub_type| [sub_type, quote! {"," +}])
			.collect();
		sub_types.pop();
		quote! {+ "<" + #(#sub_types)* ">"}
	} else {
		quote! {}
	};
	let name = quote! { stringify!(#ident).to_string() #sub_types_str };

	let ser_name = if let Some(ser_name) = ser_name {
		quote! { #ser_name.to_string() }
	} else {
		name.clone()
	};

	let tag = tag.map_or_else(|| quote! { None }, |tag| quote! { Some(#tag.to_string()) });

	// Return `impl` code
	let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

	quote! {
		const _: () = {
			use ::std::{
				mem,
				option::Option::{self, Some},
				string::String,
				stringify, vec,
			};
			use ::layout_inspect::{
				__private::memoffset::offset_of,
				defs::{DefStruct, DefStructField, DefType},
				Inspect, TypesCollector,
			};

			#[automatically_derived]
			impl #impl_generics Inspect for #ident #type_generics #where_clause {
				fn name() -> String {
					#name
				}

				// TODO: Allow deriving for unsized types
				// TODO: Deduce alignment for unsized types where possible e.g. `struct X { n: u64, s: str }`
				fn size() -> Option<usize> {
					Some(mem::size_of::<Self>())
				}

				fn align() -> Option<usize> {
					Some(mem::align_of::<Self>())
				}

				fn def(collector: &mut TypesCollector) -> DefType {
					DefType::Struct(DefStruct {
						name: <Self as Inspect>::name(),
						ser_name: #ser_name,
						size: <Self as Inspect>::size(),
						align: <Self as Inspect>::align(),
						fields: vec![#(#field_defs),*],
						tag: #tag,
						transparent: #transparent,
					})
				}
			}
		};
	}
}

fn get_named_field_defs(fields: FieldsNamed, rename_all: &Option<String>) -> Vec<TokenStream> {
	fields
		.named
		.iter()
		.map(|field| {
			let ident = field.ident.as_ref().expect("Missing field name");
			let member: Member = parse_quote!(#ident);
			let name = get_ident_name(ident);
			get_field_def(field, member, name, rename_all)
		})
		.collect()
}

fn get_unnamed_field_defs(fields: FieldsUnnamed, rename_all: &Option<String>) -> Vec<TokenStream> {
	fields
		.unnamed
		.iter()
		.enumerate()
		.map(|(index, field)| {
			let member = Member::Unnamed(Index {
				index: index as u32,
				span: Span::call_site(),
			});
			get_field_def(field, member, index.to_string(), rename_all)
		})
		.collect()
}

fn get_field_def(
	field: &Field,
	member: Member,
	name: String,
	rename_all: &Option<String>,
) -> TokenStream {
	let SerdeAttrs {
		rename: ser_name,
		flatten,
		skip,
		..
	} = get_serde_attrs(&field.attrs, "struct field");

	// Get field name, optionally applying `rename_all` transform.
	// `serde(rename)` on field takes precedence.
	let ser_name = get_ser_name(&name, &ser_name, rename_all);

	let ty = &field.ty;
	quote! {
		DefStructField {
			name: #name.to_string(),
			ser_name: #ser_name.to_string(),
			type_id: collector.collect::<#ty>(),
			offset: offset_of!(Self, #member),
			flatten: #flatten,
			skip: #skip,
		}
	}
}
