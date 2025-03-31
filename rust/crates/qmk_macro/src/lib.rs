use proc_macro2::Span;
use qmk_callback_parsing::QmkCallback;
use qmk_callback_parsing::Signature;
use quote::ToTokens;
use quote::quote;
use syn::AttrStyle;
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
    let mut function = parse_macro_input!(item as syn::ItemFn);
    // the new syntax is (Ident, Ident, Ident) -> Ident
    let signature = parse_macro_input!(attr as Signature);
    let name = function.sig.ident.to_string();
    // add _rs to the end of the ident
    let modified_name = format!("{name}_rs");
    function.sig.ident = Ident::new(modified_name.as_str(), Span::call_site());
    let callback = QmkCallback::new(name, signature);
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
            },
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
