use proc_macro2::TokenStream as TokenStream2;

use std::fmt::Debug;
use syn::{
    parse::{Parse, ParseStream, Result as ParseResult},
    token::Token,
    Token,
};

use crate::match_rep::MacroMatchRep;
use crate::matcher::MacroMatcher;
use crate::meta_var::MacroMetaVar;

#[derive(Debug)]
pub(crate) enum MacroMatchEscape {
    Rep(MacroMatchRep),
    MetaVar(MacroMetaVar),
}

impl Parse for MacroMatchEscape {
    fn parse(_input: ParseStream) -> ParseResult<Self> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub(crate) enum MacroMatch {
    Empty,
    Tokens(TokenStream2),
    Matcher(Box<MacroMatcher>),
    Escaped(MacroMatchEscape),
}

impl Parse for MacroMatch {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        if input.is_empty() {
            Ok(MacroMatch::Empty)
        } else if input.peek(Token![$]) {
            Ok(MacroMatch::Escaped(input.parse()?))
        } else {
            let fork = input.fork();
            if let Ok(macro_matcher) = fork.parse::<MacroMatcher>() {
                Ok(MacroMatch::Matcher(Box::new(macro_matcher)))
            } else {
                Ok(MacroMatch::Tokens(TokenStream2::parse(input)?))
            }
        }
    }
}
