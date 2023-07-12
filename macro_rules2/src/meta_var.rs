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

macro_rules! preferred_stringify {
    ( $preferred:ident $($rest:tt)* ) => {
        stringify!($preferred)
    };
}

macro_rules! keyword_enum {
    ( $visibility:vis $enum_id:ident ; $($keyword:ident$(($out_string:ident))?)|+ ) => {
        paste! {
            #[derive(Debug)]
            $visibility enum $enum_id {
                $([< $keyword:camel >]),+
            }

            impl TryFrom<Ident> for $enum_id {
                type Error = ParseError;
                fn try_from(value: Ident) -> Result<$enum_id, ParseError> {
                    match &value.to_string() {
                        $(id_str if preferred_stringify!($($out_string, )? $keyword) == id_str => {
                           Ok( Self::[< $keyword:camel >] )
                        }),+
                        _ => {
                            Err(ParseError::new(value.span(), format!("The identifier {} does not match any known ", value)))
                        }
                    }
                }
            }
        }
    }
}

keyword_enum! {
    MacroMetaVarType ;
    block | expr | ident | item | lifetime | literal | meta | pat | pat_param | path | stmt | token_tree(tt) | ty | vis
}

#[derive(Debug)]
enum MacroIdent {
    Ident(Ident),
    Underscore(Token![_]),
}

#[derive(Debug)]
struct MacroMetaVar {
    id: MacroIdent,
    ty: MacroMetaVarType,
}
