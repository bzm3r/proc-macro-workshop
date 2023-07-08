use proc_macro::TokenStream;
use proc_macro2::token_stream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::TokenTree;
use std::borrow::Cow;
use std::cell::Ref;
use std::cell::RefCell;
use std::iter::once;
use std::iter::Chain;
use std::iter::Empty;
use std::iter::Flatten;
use std::iter::Map;
use std::iter::Once;
use std::ops::Deref;
use std::ops::Range;
use std::slice;
use std::vec;
use syn::Ident;

pub(crate) struct Template {
    meta_var: Ident,
    meta_stream: TokenStream2,
    partial_stream: Vec<Option<TokenStream2>>,
    range: Range<usize>,
}

impl From<&Template> for TokenStream2 {
    fn from(value: &Template) -> TokenStream2 {
        TokenStream2::from_iter(
            value
                .partial_stream
                .iter()
                .map(|partial| partial.unwrap_or(value.meta_stream.clone())),
        )
    }
}

impl Template {
    pub(crate) fn new(stream: TokenStream2, meta_var: Ident, range: Range<usize>) -> Self {
        let meta_stream = TokenStream2::from(TokenTree::from(meta_var.clone()));
        let partial_stream = PartialStreamIter::from_stream(stream, &meta_var).collect();

        Self {
            meta_var,
            meta_stream,
            partial_stream,
            range,
        }
    }

    pub(crate) fn set_meta_var(&self, id: Ident) {
        self.meta_var.replace(id);
        self.meta_stream.replace(self.meta_var.stream())
    }

    fn stream(&self) -> TokenStream2 {
        self.partial.stream(&self.meta_stream)
    }
}

enum PartialStreamIter<F>
where
    F: FnMut(TokenTree) -> PartialStreamIter<F>,
{
    Flatten(Flatten<Map<token_stream::IntoIter, F>>),
    Once(Once<Option<TokenStream2>>),
    Empty(Empty<Option<TokenStream2>>),
}

impl<F> Iterator for PartialStreamIter<F>
where
    F: FnMut(TokenTree) -> PartialStreamIter<F>,
{
    type Item = Option<TokenStream2>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            PartialStreamIter::Flatten(iter) => iter.next(),
            PartialStreamIter::Once(iter) => iter.next(),
            PartialStreamIter::Empty(iter) => iter.next(),
        }
    }
}

impl<F> From<TokenTree> for PartialStreamIter<F>
where
    F: FnMut(TokenTree) -> PartialStreamIter<F>,
{
    fn from(value: TokenTree) -> Self {
        PartialStreamIter::Once(once(TokenStream2::from(value).into()))
    }
}

impl<F> From<Option<TokenStream2>> for PartialStreamIter<F>
where
    F: FnMut(TokenTree) -> PartialStreamIter<F>,
{
    fn from(value: Option<TokenStream2>) -> Self {
        PartialStreamIter::Once(value)
    }
}

impl<F> PartialStreamIter<F>
where
    F: FnMut(TokenTree) -> PartialStreamIter<F>,
{
    fn from_stream<G: FnMut(TokenTree) -> PartialStreamIter<G>>(
        stream: TokenStream2,
        meta_var: &Ident,
    ) -> PartialStreamIter<G> {
        PartialStreamIter::Flatten(stream.into_iter().map(G))
    }

    fn from_tree(tree: TokenTree, meta_var: &Ident) -> PartialStreamIter<F> {
        match tree {
            TokenTree::Group(g) => Self::from_stream(g.stream(), meta_var),
            TokenTree::Ident(id) if &id == meta_var => PartialStreamIter::Once(once(None)),
            other => PartialStreamIter::Once(once(TokenStream2::from(other).into())),
        }
    }
}
