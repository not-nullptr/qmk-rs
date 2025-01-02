use include_image_structs::Chunk;
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

    let mut chunks = vec![];
    let chunks_x = (width as f64 / 8.0).ceil() as u8;
    let chunks_y = (height as f64 / 8.0).ceil() as u8;

    for chunk_x in 0..chunks_x {
        for chunk_y in 0..chunks_y {
            let mut bytes: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
            let start_x = (chunk_x as u32) * 8;
            let end_x = start_x + 8;
            let start_y = (chunk_y as u32) * 8;
            let end_y = start_y + 8;
            for x in start_x..end_x {
                let rel_x = end_x - x - 1;
                let mut byte: u8 = 0;
                for y in start_y..end_y {
                    let rel_y = end_y - y - 1;
                    let Some(pixel_val) = img.get_pixel_checked(x, y) else {
                        continue;
                    };

                    let pixel_val = pixel_val.0[0];

                    if pixel_val > 127 {
                        byte |= 1 << rel_y;
                    } else {
                        byte &= !(1 << rel_y);
                    }
                }
                bytes[rel_x as usize] = byte.reverse_bits();
            }
            bytes.reverse();
            chunks.push(Chunk {
                x: chunk_x,
                y: chunk_y,
                bytes,
            })
        }
    }

    let split: Vec<_> = input_path.split('.').collect();
    let name = remove_non_alphanumeric(&split[0..split.len() - 1].join(".")).to_uppercase();
    let name_ident = syn::Ident::new(&name, Span::call_site().into());

    let chunk_data = chunks.iter().map(|chunk| {
        let x = chunk.x;
        let y = chunk.y;
        let bytes = chunk.bytes;
        quote! {
            include_image_structs::Chunk { x: #x, y: #y, bytes: [#(#bytes),*] }
        }
    });

    let chunk_count = chunks.len();

    let output = quote! {
        pub const #name_ident: [include_image_structs::Chunk; #chunk_count] = [
            #(#chunk_data),*
        ];
    };

    output.into()
}
