mod delimiter;
mod group;
mod macro_match;
mod match_rep;
mod matcher;
mod meta_var;
mod repetition;
mod rule;
mod rules_def;
mod transcriber;


use proc_macro2::{
    TokenStream as TokenStream2,
};

use quote::{quote, ToTokens};
use rules_def::MacroRulesDef;



use std::{fmt::Debug};
use syn::{
    parse::{
        Parse, ParseStream,
        Result as ParseResult,
    },
    parse_macro_input,
    Ident,
};

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct MacroRules2 {
    name: Ident,
    body: MacroRulesDef,
}

impl Parse for MacroRules2 {
    fn parse(_input: ParseStream) -> ParseResult<Self> {
        unimplemented!()
        // let name: Ident = input.parse()?;
        // let x: MacroDelimiter = input.parse()?;
        // let body = contents.parse()?;

        // Ok(MacroRules2 { name, body })
    }
}

impl ToTokens for MacroRules2 {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self { name, body } = self;
        *tokens = quote! {
            quote! {
                macro_rules! #name {
                    #body
                }
            };
        }
    }
}

#[proc_macro]
pub fn macro_rules2(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    eprintln!("parsing macro input");

    let parsed_macro_rules2 = parse_macro_input!(input as MacroRules2);
    eprintln!("{:?}", &parsed_macro_rules2);

    let mut tokens = TokenStream2::new();
    parsed_macro_rules2.to_tokens(&mut tokens);

    eprintln!("output tokens:\n{:#?}", tokens.to_string());
    tokens.into()
}
