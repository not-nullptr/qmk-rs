use std::fs;

use image::{ImageBuffer, Luma};
use proc_macro::{Span, TokenStream};
use quote::quote;
use regex::Regex;
use syn::{Token, parse::Parse, parse_macro_input};

fn remove_non_alphanumeric(input: &str) -> String {
    let re = Regex::new(r"[^a-zA-Z0-9_]+").unwrap();
    re.replace_all(input, "").to_string()
}

fn to_format(
    img: ImageBuffer<Luma<u8>, Vec<u8>>,
    width: usize,
    height: usize,
    use_fb_format: bool,
) -> Vec<u8> {
    let mut output = Vec::new();
    let blocks_per_col = (height + 7) / 8;

    for x in 0..width {
        for block in 0..blocks_per_col {
            let mut byte: u8 = 0;
            for bit in 0..8 {
                let y = block * 8 + bit;
                if y < height {
                    let pixel = img.get_pixel(x as u32, y as u32)[0];
                    if pixel >= 127 {
                        if use_fb_format {
                            byte |= 1 << bit;
                        } else {
                            byte |= 1 << (7 - bit);
                        }
                    }
                }
            }
            output.push(byte);
        }
    }
    output
}

fn path_to_image(path: &str, use_fb_format: bool) -> (Vec<u8>, String, usize, usize) {
    let img = match image::open(path) {
        Ok(img) => img.to_luma8(),
        Err(e) => panic!("failed to open image {}: {}", path, e),
    };

    let width = img.width() as usize;
    let height = img.height() as usize;

    let bytes = to_format(img, width, height, use_fb_format);

    let path = path
        .split('/')
        .last()
        .expect("failed to get last part of path");
    let split: Vec<_> = path.split('.').collect();
    let name = remove_non_alphanumeric(&split[0..split.len() - 1].join(".")).to_uppercase();

    (bytes, name, width, height)
}

struct ParsedArgs {
    path: String,
    framebuffer_format: bool,
}

impl Parse for ParsedArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path: syn::LitStr = input.parse()?;
        let fb_format = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            let framebuffer_format: syn::LitBool = input.parse()?;
            framebuffer_format.value()
        } else {
            false
        };
        let path = path.value();
        Ok(ParsedArgs {
            path,
            framebuffer_format: fb_format,
        })
    }
}

#[proc_macro]
pub fn include_image(input: TokenStream) -> TokenStream {
    // parse the input into a comma separated list of arguments
    let parsed_args = parse_macro_input!(input as ParsedArgs);
    let (bytes, name, width, height) =
        path_to_image(&parsed_args.path, parsed_args.framebuffer_format);

    let width = width as u8;
    let height = height as u8;

    let byte_array = bytes.as_slice();
    let byte_count = byte_array.len();

    let name_ident = syn::Ident::new(&name, Span::call_site().into());

    let byte_tokens = bytes.iter().map(|b| quote! { #b }).collect::<Vec<_>>();

    let output = quote! {
        pub const #name_ident: ::include_image::QmkImage<#byte_count> = ::include_image::QmkImage {
            width: #width,
            height: #height,
            bytes: [#(#byte_tokens),*],
        };
    };

    output.into()
}

#[proc_macro]
pub fn include_animation(input: TokenStream) -> TokenStream {
    // let input_path = parse_macro_input!(input as syn::LitStr).value();
    let parsed_args = parse_macro_input!(input as ParsedArgs);

    let files = match fs::read_dir(&parsed_args.path) {
        Ok(res) => res,
        Err(e) => panic!("failed to read animation directory: {}", e),
    };

    let name_ident = syn::Ident::new(
        &remove_non_alphanumeric(parsed_args.path.split("/").last().expect("invalid path"))
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

    for (name, _) in files.into_iter() {
        let (bytes, _name, width, height) = path_to_image(
            &format!("{}/{}", parsed_args.path, name),
            parsed_args.framebuffer_format,
        );

        if all_lens == 0 {
            all_lens = bytes.len();
        } else if all_lens != bytes.len() {
            panic!("non-equal image sizes");
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
