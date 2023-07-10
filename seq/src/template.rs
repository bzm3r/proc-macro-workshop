use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::TokenTree as TokenTree2;
use std::ops::Range;
use syn::Ident;

use crate::partial_stream::PartialStream;
use crate::partial_stream::PartialStreamIter;

pub struct Template {
    meta_var: Ident,
    meta_stream: TokenStream2,
    partial_stream: Vec<PartialStream>,
}

impl From<&Template> for TokenStream2 {
    fn from(value: &Template) -> TokenStream2 {
        TokenStream2::from_iter(
            value
                .partial_stream
                .iter()
                .map(|partial| partial.clone_or(value.meta_stream.clone())),
        )
    }
}

impl Template {
    pub fn new(stream: TokenStream2, meta_var: Ident) -> Self {
        let meta_stream = TokenStream2::from(TokenTree2::from(meta_var.clone()));
        let partial_stream = PartialStreamIter::from_stream(stream, &meta_var).collect();

        Self {
            meta_var,
            meta_stream,
            partial_stream,
        }
    }

    pub fn set_meta_var(&mut self, id: Ident) {
        self.meta_var = id;
        self.meta_stream = TokenStream2::from(TokenTree2::from(self.meta_var.clone()));
    }

    fn stream(&self) -> TokenStream2 {
        TokenStream2::from_iter(
            self.partial_stream
                .iter()
                .map(|maybe_ts| maybe_ts.clone_or(self.meta_stream.clone())),
        )
    }
}

pub struct RangeTemplate {
    range: Range<usize>,
    template: Template,
}

impl RangeTemplate {
    fn new(stream: TokenStream2, meta_var: Ident, range: Range<usize>) -> Self {
        RangeTemplate {
            range,
            template: Template::new(stream, meta_var),
        }
    }
}
