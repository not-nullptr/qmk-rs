use bindgen::Formatter;
use glob::glob;
use std::{env, path::PathBuf};

const HEADER_PATHS: &[&str] = &[
    "../../../quantum/quantum_keycodes.h",
    "../../../drivers/oled/oled_driver.h",
];

fn main() {
    // bindgen will pass -D from BINDGEN_EXTRA_CLANG_ARGS to clang
    let cflags = std::env::var("BINDGEN_CFLAGS").unwrap_or_default();
    let extra_clang_args = cflags
        .split(" ")
        .filter(|s| s.starts_with("-D"))
        .collect::<Vec<&str>>()
        .join(" ");

    unsafe {
        env::set_var("BINDGEN_EXTRA_CLANG_ARGS", extra_clang_args);
    }

    let bindings = bindgen::builder()
        .headers(HEADER_PATHS.iter().map(|path| path.to_string()))
        .use_core()
        .clang_args(
            HEADER_PATHS
                .iter()
                .map(|path| {
                    let mut split = path.split("/").collect::<Vec<_>>();
                    split.pop();
                    split.push("**");
                    split.push("*");
                    split.join("/")
                })
                .flat_map(|path| glob(&path).expect("failed to glob"))
                .filter_map(|entry| entry.ok())
                .filter(|path| path.is_dir())
                .map(|path| path.to_string_lossy().to_string())
                .map(|p| format!("-I{}", p))
                .collect::<Vec<String>>(),
        )
        // .clang_arg("-I../../../quantum")
        .clang_args(HEADER_PATHS.iter().map(|path| {
            format!("-I{}", {
                let mut split = path.split("/").collect::<Vec<_>>();
                split.pop();
                split.join("/")
            })
        }))
        .formatter(Formatter::Rustfmt)
        .rustified_enum(".*")
        .constified_enum_module(".*")
        .generate_comments(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
