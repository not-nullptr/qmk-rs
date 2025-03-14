use quote::ToTokens;
use syn::{Token, Type, parse::Parse};

#[derive(Debug)]
pub struct QmkCallback {
    pub name: String,
    pub signature: Signature,
}

#[derive(Debug)]
pub struct Signature {
    pub args: Vec<SigType>,
    pub return_type: SigType,
}

#[derive(Debug)]
pub struct SigType {
    pub name: String,
    pub token: syn::Ident,
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
    pub const fn new(name: String, signature: Signature) -> Self {
        QmkCallback { name, signature }
    }

    pub fn to_c_fn(&self) -> String {
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
        let name = &self.name;
        const SUFFIX: &str = "_rs";
        format!("{name}{SUFFIX}")
    }
}

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
