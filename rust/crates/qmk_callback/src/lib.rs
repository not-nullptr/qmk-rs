use glob::glob;
use qmk::EEPROM_BYTES;
use qmk_callback_parsing::{QmkCallback, Signature};
use std::fs;
use syn::parse::Parse;

pub fn write_glue_code(path: impl Into<String>) {
    // re-run if any of the source files change
    let path: String = path.into();
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
                    let attr = func
                        .attrs
                        .into_iter()
                        .find(|attr| attr.path().is_ident("qmk_callback"))?;

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
    fs::write(&path, c_file).unwrap();

    let mut dir = path.split("/").collect::<Vec<_>>();
    dir.pop();
    let dir = dir.join("/");
    let mut config_file = fs::read_to_string(format!("{}/config.h", dir)).unwrap();
    // find the first line starting with // ### NULLPTR'S STUFF BEGINS HERE -- DO NOT TOUCH! DON'T EVEN MODIFY THIS COMMENT!
    let line =
        "\n// ### NULLPTR'S STUFF BEGINS HERE -- DO NOT TOUCH! DON'T EVEN MODIFY THIS COMMENT!";
    let pos = config_file.find(line);
    if let Some(pos) = pos {
        config_file = config_file[..pos].to_string();
    }
    config_file += line;
    config_file += "\n";
    config_file += &format!("#define EECONFIG_USER_DATA_SIZE {}\n", EEPROM_BYTES);
    fs::write(format!("{}/config.h", dir), config_file).unwrap();
}
