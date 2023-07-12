
use proc_macro2::{
    extra::DelimSpan as DelimSpan2, Delimiter as Delimiter2,
};





use std::{fmt::Debug};
use syn::{
    parse::{
        discouraged::AnyDelimiter, Parse, ParseStream,
        Result as ParseResult,
    },
};

use crate::macro_match::MacroMatch;
use crate::repetition::ZeroOrMore;

#[derive(Debug, Clone, Copy)]
pub(crate) struct MacroMatcherDelimiter {
    delimiter: Delimiter2,
    span: DelimSpan2,
}

#[derive(Debug)]
pub(crate) struct MacroMatcher {
    matches: ZeroOrMore<MacroMatch>,
    delimiter: MacroMatcherDelimiter,
}

impl Parse for MacroMatcher {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let (delimiter, span, contents) = input.parse_any_delimiter()?;
        Ok(MacroMatcher {
            matches: contents.parse()?,
            delimiter: MacroMatcherDelimiter { delimiter, span },
        })
    }
}
