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

use paste::paste;
use proc_macro2::{
    extra::DelimSpan as DelimSpan2, Delimiter as Delimiter2, Spacing as Spacing2,
    TokenStream as TokenStream2, TokenTree as TokenTree2,
};
use proc_macro2::{Group, Punct, Span};
use quote::{quote, ToTokens};
use std::fmt::{Display, Error as FmtError, Formatter, Result as FmtResult};
use std::iter::once;
use std::ops::{Deref, DerefMut};
use std::{fmt::Debug, marker::PhantomData, ops::Range};
use syn::{
    buffer::Cursor,
    parse::{
        discouraged::AnyDelimiter, Error as ParseError, Parse, ParseBuffer, ParseStream,
        Result as ParseResult,
    },
    parse_macro_input,
    token::{Brace as SynBrace, Bracket as SynBracket, Paren as SynParen, SelfType, Token},
    Ident, ItemMacro, MacroDelimiter, Token,
};

#[allow(dead_code)]
#[derive(Debug)]
struct MacroRules2 {
    name: Ident,
    body: MacroRulesDef,
}

impl Parse for MacroRules2 {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let name: Ident = input.parse()?;
        let x: MacroDelimiter = input.parse()?;
        let body = contents.parse()?;

        Ok(MacroRules2 { name, body })
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
