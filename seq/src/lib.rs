use proc_macro2::TokenStream as TokenStream2;
use std::fmt::Debug;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, LitInt, Token,
};
use quote::quote;
use crate::template::Template;

mod template;
mod partial_stream;

#[derive(Debug)]
struct Seq {
    loop_var: Ident,
    range_start: LitInt,
    range_end: LitInt,
    body: TokenStream2,
}

impl Parse for Seq {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let loop_var = input.parse()?;
        input.parse::<Token![in]>()?;
        let range_start = input.parse()?;
        input.parse::<Token![..]>()?;
        let range_end = input.parse()?;
        let body;
        braced!(body in input);
        let body = TokenStream2::parse(&body)?;

        Ok(Seq {
            loop_var,
            range_start,
            range_end,
            body,
        })
    }
}

#[proc_macro]
pub fn seq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    eprintln!("parsing macro input");
    let Seq {
        loop_var,
        range_start,
        range_end,
        body,
    } = parse_macro_input!(input as Seq);
    eprintln!(
        "{:?}, {:?}, {:?}, {:?}",
        loop_var, range_start, range_end, body
    );

    eprintln!("doing loop variable replacement");
    let template = Template::new(body, loop_var);

    let tokens = quote! {

    };

    eprintln!("output tokens:\n{:#?}", tokens.to_string());
    tokens.into()
}
