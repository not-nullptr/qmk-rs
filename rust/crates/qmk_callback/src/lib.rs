use glob::glob;
use qmk_callback_parsing::{QmkCallback, Signature};
use std::fs;
use syn::parse::Parse;

pub fn write_glue_code(path: impl Into<String>) {
    // re-run if any of the source files change
    let path = path.into();
    println!("cargo:rerun-if-changed=src/**/*.rs");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", path);
    let mut attributes = Vec::new();

    for entry in glob("src/**/*.rs").unwrap() {
        let Ok(entry) = entry else {
            continue;
        };

        let file = fs::read_to_string(entry).unwrap();
        let file = syn::parse_file(&file).unwrap();

        let fn_attributes = file
            .items
            .into_iter()
            .filter_map(|item| {
                if let syn::Item::Fn(func) = item {
                    let name = func.sig.ident.to_string();
                    let Some(attr) = func
                        .attrs
                        .into_iter()
                        .find(|attr| attr.path().is_ident("qmk_callback"))
                    else {
                        return None;
                    };

                    Some((attr, name))
                } else {
                    None
                }
            })
            .filter_map(|(attr, name)| {
                attr.parse_args_with(Signature::parse)
                    .map(|a| (a, name))
                    .ok()
            })
            .map(|(signature, name)| QmkCallback::new(name, signature))
            .map(|cb| cb.to_c_fn())
            .collect::<Vec<_>>();

        for attr in fn_attributes {
            attributes.push(attr);
        }
    }

    let c_file = attributes.join("\n\n");
    fs::write(path, c_file).unwrap();
}
