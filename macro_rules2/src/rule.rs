use std::fmt::Debug;
use syn::{
    parse::{discouraged::AnyDelimiter, Parse, ParseStream, Result as ParseResult},
    Token,
};

use crate::matcher::MacroMatcher;
use crate::transcriber::MacroTranscriber;

#[derive(Debug)]
pub(crate) struct MacroRule {
    matcher: MacroMatcher,
    transcriber: MacroTranscriber,
}

impl Parse for MacroRule {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let matcher = input.parse()?;
        input.parse::<Token![=>]>()?;
        let (_, _, _contents) = input.parse_any_delimiter()?;
        let transcriber = input.parse()?;
        Ok(MacroRule {
            matcher,
            transcriber,
        })
    }
}
