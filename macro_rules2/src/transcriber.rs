
use proc_macro2::{
    TokenStream as TokenStream2,
};





use std::{fmt::Debug};
use syn::{
    parse::{
        Parse, ParseStream,
        Result as ParseResult,
    },
};

#[derive(Debug)]
pub(crate) struct MacroTranscriber {
    body: TokenStream2,
}

impl Parse for MacroTranscriber {
    fn parse(_input: ParseStream) -> ParseResult<Self> {
        unimplemented!()
    }
}
