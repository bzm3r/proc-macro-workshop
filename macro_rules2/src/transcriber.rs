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
struct MacroTranscriber {
    body: TokenStream2,
}

impl Parse for MacroTranscriber {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        unimplemented!()
    }
}
