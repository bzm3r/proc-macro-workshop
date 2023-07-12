use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;

use crate::delimiter::LegalDelimPair;

pub(crate) struct GroupContent {
    tokens: TokenStream2,
    span: Span,
}
pub(crate) struct GeneralGroup {
    delimiters: LegalDelimPair,
    content: GroupContent,
}
