mod keymap;

use std::collections::HashSet;
use std::fs;

use keymap::KeyboardDefinition;
use keymap::Keymap;
use proc_macro2::Span;
use qmk_callback_parsing::QmkCallback;
use qmk_callback_parsing::Signature;
use quote::ToTokens;
use quote::quote;
use syn::AttrStyle;
use syn::Expr;
use syn::ExprLit;
use syn::LitInt;
use syn::{
    Attribute, Ident, LitStr, MacroDelimiter, Meta, MetaList, Path, Token, Visibility,
    parse_macro_input, punctuated::Punctuated, token::Paren,
};

#[doc = "# QMK Callback"]
#[doc = "This macro is used to define a QMK callback function. It takes a signature in the form of `(Ident, ..) -> Ident` where each Ident is a C typedef available from within QMK. Each type is allowed to be a pointer type of any number of levels."]
#[doc = "## Example"]
#[doc = "```"]
#[doc = "use qmk_macro::qmk_callback;"]
#[doc = ""]
#[doc = "#[qmk_callback((uint8_t, bool) -> bool)]"]
#[doc = "fn my_callback(arg1: u8, arg2: bool) -> bool {"]
#[doc = "    // do something"]
#[doc = "    true"]
#[doc = "}"]
#[doc = "```"]
#[proc_macro_attribute]
pub fn qmk_callback(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let out_dir = std::env::var("OUT_DIR").unwrap_or_else(|_| "unknown".to_string());

    let out_dir = out_dir.split('/').collect::<Vec<_>>();

    // get the index of the last element where the string == "target"
    let target_index = out_dir
        .iter()
        .rposition(|s| s == &"target")
        .unwrap_or_else(|| panic!("Failed to get the target dir: {out_dir:?}"));

    let target = out_dir
        .get(target_index + 1)
        .unwrap_or_else(|| panic!("Could not find target in OUT_DIR: {out_dir:?}"));

    let is_wasm = *target == "wasm32-unknown-unknown";

    let mut function = parse_macro_input!(item as syn::ItemFn);
    // the new syntax is (Ident, Ident, Ident) -> Ident
    let signature = parse_macro_input!(attr as Signature);
    let name = function.sig.ident.to_string();
    // add _rs to the end of the ident
    if !is_wasm {
        let modified_name = format!("{name}_rs");
        function.sig.ident = Ident::new(&modified_name, Span::call_site());
    }
    let callback = QmkCallback::new(name, signature);
    function.vis = Visibility::Public(Token![pub](Span::call_site()));
    if is_wasm {
        // add #[::wasm_bindgen::prelude::wasm_bindgen]
        // to the function

        let wasm_bindgen = Attribute {
            bracket_token: Default::default(),
            meta: Meta::Path(Path {
                leading_colon: None,
                segments: {
                    let mut segments = Punctuated::new();
                    segments.push_value(Ident::new("wasm_bindgen", Span::call_site()).into());
                    segments.push_punct(Token![::](Span::call_site()));
                    segments.push_value(Ident::new("prelude", Span::call_site()).into());
                    segments.push_punct(Token![::](Span::call_site()));
                    segments.push_value(Ident::new("wasm_bindgen", Span::call_site()).into());
                    segments
                },
            }),
            pound_token: Token![#](Span::call_site()),
            style: AttrStyle::Outer,
        };

        function.attrs = vec![wasm_bindgen];
    } else {
        function.sig.abi = Some(syn::Abi {
            extern_token: Token![extern](Span::call_site()),
            name: Some(LitStr::new("C", Span::call_site())),
        });
        let no_mangle = Attribute {
            bracket_token: Default::default(),
            meta: Meta::List(MetaList {
                path: Path {
                    leading_colon: None,
                    segments: {
                        let mut segments = Punctuated::new();
                        segments.push_value(Ident::new("unsafe", Span::call_site()).into());
                        segments
                    },
                },
                delimiter: MacroDelimiter::Paren(Paren(Span::call_site())),
                tokens: quote! {
                    no_mangle
                },
            }),
            pound_token: Token![#](Span::call_site()),
            style: AttrStyle::Outer,
        };
        function.attrs = vec![no_mangle];
    }

    // get arg types
    let args = &callback.signature.args;

    let arg_types = args
        .iter()
        .chain(std::iter::once(&callback.signature.return_type))
        .map(|arg| {
            let arg_ident = &arg.token;
            // r-a doesn't support stringify! in doc, so we do this instead
            let doc_literal = format!("## C type `{}`\n`{}`, although shown as a Rust type, is actually a C type. **The lack of an error here does not guarantee a compiling QMK project** — you must verify this type is valid in the C glue code.", arg.name, arg.name);
            quote! {
                {
                    #[allow(unused_variables)]
                    #[allow(non_camel_case_types)]
                    #[doc = #doc_literal]
                    type #arg_ident = ();
                };
            }
        })
        .collect::<Vec<_>>();
    let arg_types = quote! {
        {
            #(#arg_types)*
        }
    };
    function
        .block
        .stmts
        .insert(0, syn::parse2(arg_types).unwrap());
    // convert function back to TokenStream
    function.into_token_stream().into()
}

/// # Keymap
///
/// This macro is used to define the keymap. Use as follows:
/// ```rust
/// use qmk_macro::keymap;
///
/// keymap! {
///     "sofle/rev1",
///     {
///         KC_NO, KC_NO, KC_NO, // ...
///     }
/// }
/// ```
#[proc_macro]
pub fn keymap(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let keymap = parse_macro_input!(input as Keymap);
    let keyboard_definition = fs::read_to_string(format!(
        "../keyboards/{}/keyboard.json",
        keymap.keeb
    ))
    .unwrap_or_else(|_| {
        fs::read_to_string(format!("../keyboards/{}/info.json", keymap.keeb))
            .unwrap_or_else(|_| panic!("Failed to read keyboard definition for {}", keymap.keeb))
    });

    let keyboard_definition: KeyboardDefinition = serde_json::from_str(&keyboard_definition)
        .unwrap_or_else(|_| panic!("Failed to parse keyboard definition for {}", keymap.keeb));

    let matrix_map = keyboard_definition
        .layouts
        .get("LAYOUT")
        .unwrap_or_else(|| {
            panic!(
                "Failed to find layout LAYOUT in keyboard definition for {}",
                keymap.keeb
            )
        });

    // find the number of unique matrix row values, ie matrix_map.layout[0].matrix[0]

    let matrix_rows = matrix_map
        .layout
        .iter()
        .map(|l| l.matrix[0])
        .collect::<HashSet<_>>()
        .len();

    let matrix_cols = matrix_map
        .layout
        .iter()
        .map(|l| l.matrix[1])
        .collect::<HashSet<_>>()
        .len();

    let mut layers = vec![];

    let num_layers = keymap.layers.len();

    for (x, layer) in keymap.layers.into_iter().enumerate() {
        let mut key_idents = vec![
            vec![
                Expr::Lit(ExprLit {
                    attrs: vec![],
                    lit: syn::Lit::Int(LitInt::new("0u16", Span::call_site())),
                });
                matrix_cols
            ];
            matrix_rows
        ];

        for (i, key) in layer.keys.into_iter().enumerate() {
            let Some(matrix_mapping) = matrix_map.layout.get(i).map(|m| m.matrix) else {
                panic!(
                    "Too many keys for {} in layer {x} (at key {i}, or '{}')",
                    keymap.keeb,
                    key.to_token_stream()
                );
            };
            let col = matrix_mapping[1] as usize;
            let row = matrix_mapping[0] as usize;
            // key_idents[row][col] = key;
            let Some(ident) = key_idents.get_mut(row).and_then(|r| r.get_mut(col)) else {
                panic!(
                    "Too many keys for {} in layer {x} (at key {i}, or '{}')",
                    keymap.keeb,
                    key.to_token_stream()
                );
            };

            *ident = key;
        }

        layers.push(key_idents);
    }

    let layers_tokens = layers
        .iter()
        .map(|layer| {
            let layer_tokens = layer
                .iter()
                .map(|row| {
                    let row_tokens = row.chunks(matrix_rows).map(|keys| {
                        // quote! {
                        //     #(
                        //         ::qmk::key!(#key)
                        //     ),*
                        // }
                        let key_tokens = keys.iter().map(|key| {
                            if key.to_token_stream().to_string().starts_with("CS_") {
                                quote! {
                                    #key
                                }
                            } else {
                                quote! {
                                    ::qmk::key!(#key)
                                }
                            }
                        });

                        quote! {
                            #(#key_tokens),*
                        }
                    });
                    quote! {
                        [
                            #(#row_tokens),*
                        ]
                    }
                })
                .collect::<Vec<_>>();

            quote! {
                [
                    #(#layer_tokens),*
                ]
            }
        })
        .collect::<Vec<_>>();

    let layers = quote! {
        [
            #(#layers_tokens),*
        ]
    };

    let output = quote! {
        #[unsafe(no_mangle)]
        #[allow(non_upper_case_globals)]
        static keymaps: [[[u16; #matrix_cols]; #matrix_rows]; #num_layers] = #layers;
    };

    output.into()
}
