use proc_macro2::{
    extra::DelimSpan as DelimSpan2, Delimiter as Delimiter2, TokenStream as TokenStream2,
};
use std::fmt::Debug;
use syn::{
    parse::{discouraged::AnyDelimiter, Parse, ParseStream, Result as ParseResult},
    parse_macro_input, Ident,
};

#[derive(Debug)]
struct Delimited {
    delimiter: Delimiter2,
    span: DelimSpan2,
    contents: TokenStream2,
}

impl Parse for Delimited {
    fn parse(input: ParseStream<'_>) -> ParseResult<Self> {
        let (delimiter, span, content_cursor) = input.parse_any_delimiter()?;
        Ok(Delimited {
            delimiter,
            span,
            contents: TokenStream2::parse(&content_cursor)?,
        })
    }
}

#[derive(Debug)]
struct MacroRules2 {
    macro_name: Ident,
    macro_def: Delimited,
}

impl Parse for MacroRules2 {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let macro_name = input.parse()?;
        let macro_def = input.parse()?;

        Ok(MacroRules2 {
            macro_name,
            macro_def,
        })
    }
}

#[proc_macro]
pub fn macro_rules2(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    eprintln!("parsing macro input");

    let MacroRules2 { macro_def, .. } = {
        let result = parse_macro_input!(input as MacroRules2);
        eprintln!("{:?}", &result);
        result
    };

    let tokens = macro_def.contents;

    eprintln!("output tokens:\n{:#?}", tokens.to_string());
    tokens.into()
}
