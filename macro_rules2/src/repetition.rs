use std::iter::once;
use std::ops::{Deref, DerefMut};
use std::{fmt::Debug, ops::Range};
use syn::{
    parse::{Error as ParseError, Parse, ParseStream, Result as ParseResult},
    Token,
};

#[derive(Debug)]
pub(crate) enum OneOrMore<T: Parse> {
    One(T),
    More(Vec<T>),
}

impl<T: Parse> Parse for OneOrMore<T> {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let first: T = input.parse()?;

        if input.is_empty() {
            Ok(OneOrMore::One(first))
        } else {
            input.parse::<Token![;]>().map_err(|mut err| {
                let span = err.span();
                err.combine(ParseError::new(
                    span,
                    "Expected semi-colon after first macro rule, if additional follow.",
                ));
                err
            })?;
            let rest = input.parse::<ZeroOrMore<T>>()?;
            Ok(rest.into_one_or_more(first))
        }
    }
}

#[derive(Debug)]
pub(crate) struct ZeroOrMore<T: Parse>(Vec<T>);

impl<T: Parse> ZeroOrMore<T> {
    pub fn new() -> Self {
        ZeroOrMore(Vec::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        ZeroOrMore(Vec::with_capacity(capacity))
    }

    pub fn into_one_or_more(self, first: T) -> OneOrMore<T> {
        if self.len() > 0 {
            OneOrMore::More(once(first).chain(self.0.into_iter()).collect())
        } else {
            OneOrMore::One(first)
        }
    }
}

impl<T: Parse> Deref for ZeroOrMore<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Parse> DerefMut for ZeroOrMore<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Parse> From<Vec<T>> for ZeroOrMore<T> {
    fn from(value: Vec<T>) -> Self {
        ZeroOrMore(value)
    }
}

impl<T: Parse> Parse for ZeroOrMore<T> {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let mut result = ZeroOrMore::<T>::new();
        while !input.is_empty() {
            result.push(input.parse()?);
        }
        Ok(result)
    }
}

#[derive(Debug)]
pub(crate) struct MacroRepSep(&'static str);

#[derive(Debug)]
pub(crate) enum MacroRepOp {
    OneOrMore,
    ZeroOrMore,
    Exactly(usize),
    UpTo(usize),
    UpToOrEqual(usize),
    MoreThan(usize),
    MoreThanOrEqual(usize),
    Between(Range<usize>),
    OneOf(Vec<usize>),
}
