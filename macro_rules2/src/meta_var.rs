use paste::paste;

use std::fmt::Debug;
use syn::{parse::Error as ParseError, Ident, Token};

macro_rules! preferred_stringify {
    ( $preferred:ident $($rest:tt)* ) => {
        stringify!($preferred)
    };
}

macro_rules! keyword_enum {
    ( $visibility:vis $enum_id:ident ; $($keyword:ident$(($out_string:ident))?)|+ ) => {
        paste! {
            #[derive(Debug)]
            $visibility pub(crate) enum $enum_id {
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
pub(crate) enum MacroIdent {
    Ident(Ident),
    Underscore(Token![_]),
}

#[derive(Debug)]
pub(crate) struct MacroMetaVar {
    id: MacroIdent,
    ty: MacroMetaVarType,
}
