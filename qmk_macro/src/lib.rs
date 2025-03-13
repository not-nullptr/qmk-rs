use proc_macro2::Span;
use quote::ToTokens;
use quote::quote;
use std::fs;
use std::sync::Mutex;
use syn::AttrStyle;
use syn::{
    Attribute, Ident, LitStr, MacroDelimiter, Meta, MetaList, Path, Token, Visibility,
    parse_macro_input, punctuated::Punctuated, token::Paren,
};

#[derive(Debug)]
struct QmkCallback {
    name: String,
    args: Vec<String>,
    return_type: String,
}

impl QmkCallback {
    fn new(name: String, args: Vec<String>, return_type: String) -> Self {
        QmkCallback {
            name,
            args,
            return_type,
        }
    }

    fn to_c_fn(&self) -> String {
        let suffix_name = self.suffix_name();
        let name = &self.name;
        let args = &self.args;
        let return_type = &self.return_type;

        if &self.return_type == "void" && self.args.len() == 0 {
            return format!(
                "void {suffix_name}(void);\nvoid {name}(void) {{\n  return {suffix_name}();\n}}",
            );
        }

        let fn_args = self
            .args
            .iter()
            .enumerate()
            .map(|(i, arg_type)| format!("{arg_type} arg{i}"))
            .collect::<Vec<_>>()
            .join(", ");

        let passed_args = self
            .args
            .iter()
            .enumerate()
            .map(|(i, _)| format!("arg{i}"))
            .collect::<Vec<_>>()
            .join(", ");

        let binding_fn = format!("{return_type} {suffix_name}({fn_args});");
        let glue_fn = format!(
            "{return_type} {name}({fn_args}) {{\n  return {suffix_name}({passed_args});\n}}",
        );

        format!("{binding_fn}\n{glue_fn}")
    }

    fn suffix_name(&self) -> String {
        format!("{}_rs", self.name)
    }
}

static CALLBACKS: Mutex<Vec<QmkCallback>> = Mutex::new(Vec::new());

#[proc_macro_attribute]
pub fn qmk_callback(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut function = parse_macro_input!(item as syn::ItemFn);
    let types = parse_macro_input!(attr with Punctuated::<Ident, Token![,]>::parse_terminated);

    let name = function.sig.ident.to_string();
    // add _rs to the end of the ident
    let modified_name = format!("{name}_rs");
    function.sig.ident = Ident::new(modified_name.as_str(), Span::call_site());
    let mut args: Vec<_> = types
        .iter()
        .map(|arg| {
            let arg_str = arg.to_string();
            if arg_str.ends_with("_PTR") {
                arg_str.replace("_PTR", "*")
            } else {
                arg_str
            }
        })
        .collect();
    eprintln!("{:?}", args);
    let return_type = args.pop().expect("return type is required");

    let mut callbacks = CALLBACKS.lock().unwrap();
    let callback = QmkCallback::new(name, args, return_type);
    eprintln!("Adding callback: {:?}", callback);
    callbacks.push(callback);

    function.vis = Visibility::Public(Token![pub](Span::call_site()));
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
            }
            .into(),
        }),
        pound_token: Token![#](Span::call_site()),
        style: AttrStyle::Outer,
    };
    function.attrs = vec![no_mangle];

    function.into_token_stream().into()
}

#[proc_macro]
pub fn save(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let path = parse_macro_input!(input as LitStr);
    write_bindings(path.value()).expect("failed to write bindings");
    proc_macro::TokenStream::default()
}

fn write_bindings(path: String) -> anyhow::Result<()> {
    // delete if exists
    if fs::exists(&path)? {
        fs::remove_file(&path)?;
    }
    let mut callbacks = CALLBACKS.lock().unwrap();

    let fns = callbacks
        .iter()
        .map(|f| f.to_c_fn())
        .collect::<Vec<_>>()
        .join("\n\n");

    fs::write(path, fns)?;
    callbacks.clear();

    Ok(())
}
