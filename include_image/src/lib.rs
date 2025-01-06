use std::fs;

use image::{ImageBuffer, Luma};
use proc_macro::{Span, TokenStream};
use quote::quote;
use regex::Regex;
use syn::{parse_macro_input, LitInt};

fn remove_non_alphanumeric(input: &str) -> String {
    let re = Regex::new(r"[^a-zA-Z0-9_]+").unwrap();
    re.replace_all(input, "").to_string()
}

fn to_format(img: ImageBuffer<Luma<u8>, Vec<u8>>, width: usize, height: usize) -> Vec<u8> {
    let iter_height = ((height as f64) / 8.0).ceil() as u32;

    let mut bytes: Vec<u8> = vec![];

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

    bytes
}

fn path_to_image(path: &str) -> (Vec<u8>, String, usize, usize) {
    let img = match image::open(path) {
        Ok(img) => img.to_luma8(),
        Err(e) => panic!("failed to open image {}: {}", path, e),
    };

    let width = img.width() as usize;
    let height = img.height() as usize;

    let bytes = to_format(img, width, height);

    let split: Vec<_> = path.split('.').collect();
    let name = remove_non_alphanumeric(&split[0..split.len() - 1].join(".")).to_uppercase();

    (bytes, name, width, height)
}

#[proc_macro]
pub fn include_image(input: TokenStream) -> TokenStream {
    let input_path = parse_macro_input!(input as syn::LitStr).value();

    let (bytes, name, width, height) = path_to_image(&input_path);

    let width = width as u8;
    let height = height as u8;

    let byte_array = bytes.as_slice();
    let byte_count = byte_array.len();

    let name_ident = syn::Ident::new(&name, Span::call_site().into());

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

#[proc_macro]
pub fn include_animation(input: TokenStream) -> TokenStream {
    let input_path = parse_macro_input!(input as syn::LitStr).value();

    let files = match fs::read_dir(&input_path) {
        Ok(res) => res,
        Err(e) => panic!("failed to read animation directory: {}", e),
    };

    let name_ident = syn::Ident::new(
        &remove_non_alphanumeric(&input_path.split("/").last().expect("invalid path"))
            .to_uppercase(),
        Span::call_site().into(),
    );

    let mut files: Vec<_> = files
        .into_iter()
        .filter_map(|f| f.ok())
        .filter_map(|f| {
            f.file_name()
                .to_str()
                .map(|name| (name.to_string(), f.path()))
        })
        .filter_map(|(name, path)| fs::read(&path).ok().map(|content| (name, content)))
        .collect();

    files.sort_by(|a, b| a.0.cmp(&b.0));

    let mut images_tokens = vec![];

    let mut all_lens = 0;

    for (_, (name, _)) in files.into_iter().enumerate() {
        let (bytes, _name, width, height) = path_to_image(&format!("{}/{}", input_path, name));

        if all_lens == 0 {
            all_lens = bytes.len();
        } else {
            if all_lens != bytes.len() {
                panic!("non-equal image sizes");
            }
        }

        let width = width as u8;
        let height = height as u8;

        let byte_tokens = bytes.iter().map(|b| quote! { #b }).collect::<Vec<_>>();

        let byte_count = bytes.len();

        images_tokens.push(quote! {
            include_image_structs::QmkImage::<#byte_count> {
                width: #width,
                height: #height,
                bytes: [#(#byte_tokens),*],
            }
        });
    }

    let images_tokens_len = images_tokens.len();

    // let images_tokens = images_tokens
    //     .iter()
    //     .fold(quote! {}, |acc, new| quote! {#acc #new});

    let output = quote! {
        pub const #name_ident: [include_image_structs::QmkImage<#all_lens>; #images_tokens_len] = [
            #(#images_tokens),*
        ];
    };

    output.into()
}
