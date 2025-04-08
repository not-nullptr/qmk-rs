use std::collections::HashMap;

use serde::Deserialize;
use syn::{Ident, Token, braced, parse::Parse};

#[derive(Debug)]
pub struct Keymap {
    pub keeb: String,
    pub layers: Vec<Layer>,
}

impl Parse for Keymap {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // comma seperated arguments
        let mut layers = Vec::new();
        let lit = input.parse::<syn::LitStr>()?;
        let keeb = lit.value();
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            while !input.is_empty() {
                let content;
                braced!(content in input);
                while !content.is_empty() {
                    let layer: Layer = content.parse()?;
                    layers.push(layer);
                    if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?;
                    }
                }
                if !input.is_empty() {
                    input.parse::<Token![,]>()?;
                }
            }
        }

        Ok(Keymap { keeb, layers })
    }
}

#[derive(Debug)]
pub struct Layer {
    pub keys: Vec<Ident>,
}

impl Parse for Layer {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut keys = Vec::new();
        while !input.is_empty() {
            let key: Ident = input.parse()?;
            keys.push(key);
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(Layer { keys })
    }
}

#[derive(Deserialize, Debug)]
pub struct KeyboardDefinition {
    pub layouts: HashMap<String, KeebDefLayout>,
}

#[derive(Deserialize, Debug)]
pub struct KeebDefLayout {
    pub layout: Vec<MatrixMapping>,
}

#[derive(Deserialize, Debug)]
pub struct MatrixMapping {
    pub matrix: [u8; 2],
}
