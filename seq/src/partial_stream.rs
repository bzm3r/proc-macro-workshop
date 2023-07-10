use proc_macro2::token_stream::IntoIter as TokenTreeIter2;
use proc_macro2::Ident;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::TokenTree as TokenTree2;
use quote::ToTokens;
use std::iter::once as iter_once;
use std::iter::Once;
use std::iter::Peekable;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Clone)]
pub struct PartialStream(Option<TokenStream2>);

impl From<TokenStream2> for PartialStream {
    fn from(value: TokenStream2) -> Self {
        PartialStream(value.into())
    }
}

impl From<TokenTree2> for PartialStream {
    fn from(value: TokenTree2) -> Self {
        value.into_token_stream().into()
    }
}

impl Deref for PartialStream {
    type Target = Option<TokenStream2>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PartialStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PartialStream {
    fn meta() -> Self {
        PartialStream(None)
    }

    fn is_meta(&self) -> bool {
        self.0.is_none()
    }
}

pub struct FlatPartialIter<'a> {
    trees: TokenTreeIter2,
    partial_iter_stack: Vec<Peekable<PartialStreamIter<'a>>>,
    meta_var: &'a Ident,
}

impl<'a> FlatPartialIter<'a> {
    fn new(trees: TokenTreeIter2, meta_var: &'a Ident) -> Self {
        Self {
            trees,
            partial_iter_stack: vec![],
            meta_var,
        }
    }
}

impl<'a> Iterator for FlatPartialIter<'a> {
    type Item = PartialStream;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current_iter) = self.partial_iter_stack.last_mut() {
            if current_iter.peek().is_some() {
                return current_iter.next();
            } else {
                self.partial_iter_stack.pop();
            }
        }

        if let Some(tree) = self.trees.next() {
            self.partial_iter_stack
                .push(PartialStreamIter::from_tree(tree, &self.meta_var).peekable());
            return self.next();
        }

        None
    }
}

#[derive(Default)]
pub enum PartialStreamIter<'a> {
    FromStream(FlatPartialIter<'a>),
    Once(Once<PartialStream>),
    #[default]
    Empty,
}

impl<'a> PartialStreamIter<'a> {}

impl<'a> Iterator for PartialStreamIter<'a> {
    type Item = PartialStream;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            PartialStreamIter::FromStream(iter) => iter.next(),
            PartialStreamIter::Once(iter) => iter.next(),
            PartialStreamIter::Empty => None,
        }
    }
}

impl<'a> From<PartialStream> for PartialStreamIter<'a> {
    fn from(value: PartialStream) -> Self {
        PartialStreamIter::Once(iter_once(value))
    }
}

// struct MapAdaptor<'a> {
//     iter: TokenTreeIter2,
//     meta_var: &'a Ident,
// }

// impl<'a> Iterator for MapAdaptor<'a> {
//     type Item = PartialStreamIter<'a>;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.iter
//             .next()
//             .map(|tree| PartialStreamIter::from_tree(tree, &self.meta_var))
//     }
// }

// impl<'a> MapAdaptor<'a> {
//     fn new(iter: TokenTreeIter2, meta_var: &'a Ident) -> Self {
//         MapAdaptor { iter, meta_var }
//     }
// }

impl<'a> PartialStreamIter<'a> {
    fn from_stream(stream: TokenStream2, meta_var: &Ident) -> PartialStreamIter {
        //PartialStreamIter::FromStream(MapAdaptor::new(stream.into_iter(), meta_var).flatten())
        PartialStreamIter::FromStream(FlatPartialIter::new(stream.into_iter(), meta_var))
    }

    fn from_tree(tree: TokenTree2, meta_var: &Ident) -> PartialStreamIter {
        match tree {
            TokenTree2::Group(g) => Self::from_stream(g.stream(), meta_var),
            TokenTree2::Ident(id) if &id == meta_var => PartialStream::meta().into(),
            other => PartialStream::from(other).into(),
        }
    }
}
