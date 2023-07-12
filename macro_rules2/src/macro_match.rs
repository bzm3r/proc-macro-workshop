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

#[derive(Debug)]
enum MacroMatchEscape {
    Rep(MacroMatchRep),
    MetaVar(MacroMetaVar),
}

impl Parse for MacroMatchEscape {
    fn parse(input: ParseStream) -> ParseResult<Self> {}
}

#[derive(Debug)]
enum MacroMatch {
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
                Ok(MacroMatch::Tokens(TokenStream2::parse(&input)?))
            }
        }
    }
}
