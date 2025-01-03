use proc_macro::{Span, TokenStream};
use quote::quote;
use regex::Regex;
use syn::parse_macro_input;

fn remove_non_alphanumeric(input: &str) -> String {
    let re = Regex::new(r"[^a-zA-Z0-9_]+").unwrap();
    re.replace_all(input, "").to_string()
}

#[proc_macro]
pub fn include_image(input: TokenStream) -> TokenStream {
    let input_path = parse_macro_input!(input as syn::LitStr).value();

    let img = match image::open(&input_path) {
        Ok(img) => img.to_luma8(),
        Err(e) => panic!("failed to open image {}: {}", input_path, e),
    };

    let width = img.width() as usize;
    let height = img.height() as usize;
    let iter_height = ((height as f64) / 8.0).ceil() as u32;

    let mut bytes: Vec<u8> = vec![];

    // each byte in bytes is 1px horizontally and 8px vertically

    for unadjusted_y in 0..iter_height {
        for x in 0..width {
            let x = x as u32;
            let mut byte = 0;
            for bit in 0..8 {
                let y = (unadjusted_y * 8) + bit;
                let Some(px) = img.get_pixel_checked(x, y) else {
                    continue;
                };
                let should_be_lit = px.0[0] > 127;
                if should_be_lit {
                    byte |= 1 << bit;
                }
            }

            bytes.push(byte);
        }
    }

    let split: Vec<_> = input_path.split('.').collect();
    let name = remove_non_alphanumeric(&split[0..split.len() - 1].join(".")).to_uppercase();
    let name_ident = syn::Ident::new(&name, Span::call_site().into());

    let width = width as u8;
    let height = height as u8;

    let byte_array = bytes.as_slice();
    let byte_count = byte_array.len();

    let byte_tokens = bytes.iter().map(|b| quote! { #b }).collect::<Vec<_>>();

    let output = quote! {
        pub const #name_ident: include_image_structs::QmkImage<#byte_count> = include_image_structs::QmkImage {
            width: #width,
            height: #height,
            bytes: [#(#byte_tokens),*],
        };
    };

    output.into()
}
