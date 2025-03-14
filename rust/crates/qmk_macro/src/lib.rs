use proc_macro2::Span;
use quote::ToTokens;
use quote::quote;
use std::fs;
use std::sync::Mutex;
use syn::AttrStyle;
use syn::Type;
use syn::parse::Parse;
use syn::{
    Attribute, Ident, LitStr, MacroDelimiter, Meta, MetaList, Path, Token, Visibility,
    parse_macro_input, punctuated::Punctuated, token::Paren,
};

#[derive(Debug)]
struct QmkCallback {
    name: String,
    signature: Signature,
}

#[derive(Debug)]
struct Signature {
    args: Vec<SigType>,
    return_type: SigType,
}

#[derive(Debug)]
struct SigType {
    name: String,
    token: syn::Ident,
}

impl SigType {
    fn new(name: String, token: syn::Type) -> Self {
        // convert to ident token
        let token = match &token {
            Type::Path(type_path) => type_path.path.segments.last().unwrap().ident.clone(),
            _ => panic!("Unsupported type"),
        };
        SigType { name, token }
    }
}

impl QmkCallback {
    const fn new(name: String, signature: Signature) -> Self {
        QmkCallback { name, signature }
    }

    fn to_c_fn(&self) -> String {
        let suffix_name = self.suffix_name();
        let name = &self.name;
        let args = &self.signature.args;
        let return_type = &self.signature.return_type;

        if return_type.name == "void" && args.len() == 0 {
            return format!(
                "void {suffix_name}(void);\nvoid {name}(void) {{\n  return {suffix_name}();\n}}",
            );
        }

        let fn_args = args
            .iter()
            .enumerate()
            .map(|(i, arg_type)| format!("{} arg{i}", arg_type.name))
            .collect::<Vec<_>>()
            .join(", ");

        let passed_args = args
            .iter()
            .enumerate()
            .map(|(i, _)| format!("arg{i}"))
            .collect::<Vec<_>>()
            .join(", ");

        let binding_fn = format!("{} {suffix_name}({fn_args});", return_type.name);
        let glue_fn = format!(
            "{} {name}({fn_args}) {{\n  return {suffix_name}({passed_args});\n}}",
            return_type.name
        );

        format!("{binding_fn}\n{glue_fn}")
    }

    fn suffix_name(&self) -> String {
        format!("{}_rs", self.name)
    }
}

static CALLBACKS: Mutex<Vec<QmkCallback>> = Mutex::new(Vec::new());

unsafe impl Sync for QmkCallback {}
unsafe impl Send for QmkCallback {}

impl Parse for SigType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let arg: Type = input.parse()?;
        let mut arg_str = arg.to_token_stream().to_string();

        while input.peek(Token![*]) {
            input.parse::<Token![*]>()?;
            arg_str.push('*');
        }

        Ok(SigType::new(arg_str, arg))
    }
}

impl Parse for Signature {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut args = Vec::new();

        // parse parens and content within
        let content;
        syn::parenthesized!(content in input);

        // parse args
        while !content.is_empty() {
            // let arg: Type = content.parse()?;
            // let mut arg_str = arg.to_token_stream().to_string();

            // while content.peek(Token![*]) {
            //     content.parse::<Token![*]>()?;
            //     arg_str.push('*');
            // }

            // args.push(SigType::new(arg_str, arg));
            let arg: SigType = content.parse()?;
            args.push(arg);
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }

        input.parse::<Token![->]>()?;

        let return_type: SigType = input.parse()?;

        Ok(Signature { args, return_type })
    }
}

#[doc = "# QMK Callback"]
#[doc = "This macro is used to define a QMK callback function. It takes a signature in the form of `(Ident, ..) -> Ident` where each Ident is a C typedef available from within QMK. Each type is allowed to be a pointer type of any number of levels."]
#[doc = "## Example"]
#[doc = "```"]
#[doc = "use qmk_macro::qmk_callback;"]
#[doc = "use qmk_macro::save;"]
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
    let mut function = parse_macro_input!(item as syn::ItemFn);
    // the new syntax is (Ident, Ident, Ident) -> Ident
    let signature = parse_macro_input!(attr as Signature);
    let name = function.sig.ident.to_string();
    // add _rs to the end of the ident
    let modified_name = format!("{name}_rs");
    function.sig.ident = Ident::new(modified_name.as_str(), Span::call_site());
    let callback = QmkCallback::new(name, signature);
    eprintln!("Adding callback: {:?}", callback);
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
    let mut callbacks = CALLBACKS.lock().unwrap();
    callbacks.push(callback);
    drop(callbacks);
    let arg_types = quote! {
        {
            #(#arg_types)*
        }
    };
    eprintln!("Arg types: {:?}", arg_types.to_string());
    function
        .block
        .stmts
        .insert(0, syn::parse2(arg_types).unwrap());
    // convert function back to TokenStream
    function.into_token_stream().into()
}

#[doc = "This macro saves the collected QMK callbacks to a file. It takes a string literal as an argument, as the path to save the C file to. This should be called after all QMK callbacks have been defined."]
#[doc = "## Example"]
#[doc = "```"]
#[doc = "// callback.rs"]
#[doc = "use qmk_macro::qmk_callback;"]
#[doc = ""]
#[doc = "#[qmk_callback((uint8_t, bool) -> bool)]"]
#[doc = "fn my_callback(arg1: u8, arg2: bool) -> bool {"]
#[doc = "    // do something"]
#[doc = "    true"]
#[doc = "}"]
#[doc = ""]
#[doc = "// lib.rs"]
#[doc = "mod callback;"]
#[doc = ""]
#[doc = "use qmk_macro::save;"]
#[doc = ""]
#[doc = "save!(\"../keyboards/my_keyboard/keymaps/my_keymap/callbacks.c\");"]
#[doc = "```"]
#[proc_macro]
pub fn save(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let path = parse_macro_input!(input as LitStr);
    write_bindings(path.value())
        .expect(format!("failed to write bindings to {}", path.value()).as_str());
    proc_macro::TokenStream::default()
}

fn write_bindings(path: String) -> anyhow::Result<()> {
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
