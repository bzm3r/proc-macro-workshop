use proc_macro2::TokenStream as TokenStream2;

use quote::ToTokens;

use std::fmt::Debug;
use syn::parse::{Parse, ParseStream, Result as ParseResult};

use crate::repetition::OneOrMore;
use crate::rule::MacroRule;

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct MacroRulesDef {
    rules: OneOrMore<MacroRule>,
}

impl Parse for MacroRulesDef {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        Ok(MacroRulesDef {
            rules: input.parse()?,
        })
    }
}

impl ToTokens for MacroRulesDef {
    fn to_tokens(&self, _tokens: &mut TokenStream2) {
        unimplemented!()
    }
}
