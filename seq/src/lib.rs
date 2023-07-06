use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input,
    token::Brace,
    ExprBlock, Ident, LitInt, Token,
};

#[derive(Debug)]
struct Seq {
    loop_var: Ident,
    range_start: LitInt,
    range_end: LitInt,
    body: proc_macro2::TokenStream,
}

impl Parse for Seq {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let loop_var = input.parse()?;
        input.parse::<Token![in]>()?;
        let range_start = input.parse()?;
        let range_end = input.parse()?;
        let body;
        braced!(body in input);
        let body = proc_macro2::TokenStream::parse(&body)?;

        eprintln!("seq");
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
    let seq = parse_macro_input!(input as Seq);
    // let Seq {
    //     loop_var,
    //     range_start,
    //     range_end,
    // }

    let tokens = quote! {};

    eprintln!("{:#?}", seq);
    tokens.into()
}
