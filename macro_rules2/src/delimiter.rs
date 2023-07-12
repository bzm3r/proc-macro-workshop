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

trait LegalDelimPairSeal {
    fn left(&self) -> LegalDelimiter;
    fn right(&self) -> LegalDelimiter;
    fn left_span(&self) -> Span;
    fn right_span(&self) -> Span;
}

enum LegalDelimPair {
    Brace(SynBrace),
    Bracket(SynBracket),
    Paren(SynParen),
    Custom {
        left: LegalDelimiter,
        right: LegalDelimiter,
    },
}

#[derive(Debug, Clone)]
struct PunctString(Vec<LegalPunct>);

impl Parse for PunctString {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        unimplemented!()
    }
}

trait LegalDelimiterSeal {}

enum LegalDelimiter {
    Punct {
        start: LegalPunct,
        end: LegalPunct,
    },
    PunctString {
        start: PunctString,
        end: PunctString,
    },
    None,
}

impl LegalDelimiterSeal for LegalDelimiter {}

// const PUNCT_EQ: Punct = Punct::new("=", Spacing2::Alone);

macro_rules! legal_punct {
    ( $($id:ident : $p:literal),* ) => {
        #[derive(Debug, Clone)]
        enum LegalPunct {
            $($id {
                punct: Punct,
                span: Span,
            },)*
        }

        impl LegalPunct {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$id { .. } => $p,)*
                }
            }

            pub fn punct(&self) -> Punct {
                match self {
                    $(Self::$id { punct, .. } => punct.clone(),)*
                }
            }

            pub fn span(&self) -> Span {
                match self {
                    $(Self::$id { span, .. } => span.clone(),)*
                }
            }
        }

        impl Display for LegalPunct {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                write!(f, "{}", self.as_str())?;
                Ok(())
            }
        }

        impl From<(Punct, Span)> for LegalDelimiter {
            fn from(punct_and_span: (Punct, Span)) -> Self {
                let (punct, span) = punct_and_span;
                match punct.as_char().to_string().as_str() {
                    $($p => {
                        Self::$id {
                            punct,
                            span,
                        }
                    }),*
                }
            }
        }
    }
}

legal_punct!(
    Eq: "=", LeftAngle: "<", RightAngle: ">", Bang: "!", Tilde: "~", Plus: "+",
    Dash: "-", Star: "*", ForwardSlash: "/", Percent: "%", Caret: "^",
    And: "&", Bar: "|", At: "@", Dot: ".", Comma: ",", SemiColon: ";",
    Colon: ":", Hash: "#", Dollar: "$", Question: "?", Backslash: "\\"
);

trait LegalPunctSeal {}

impl LegalPunctSeal for LegalPunct {}

impl Parse for LegalDelimiter {
    fn parse(input: ParseStream) -> ParseResult<LegalDelimiter> {
        input.step(|cursor| cursor.punct()?)
    }
}

// // macro_rules! parenthesized {
// //     ($content:ident in $cursor:expr) => {
// //         match $crate::__private::parse_parens(&$cursor) {
// //             $crate::__private::Ok(parens) => {
// //                 $content = parens.content;
// //                 parens.token
// //             }
// //             $crate::__private::Err(error) => {
// //                 return $crate::__private::Err(error);
// //             }
// //         }
// //     };
// // }

// // fn parse_parens<'a>(input: &ParseBuffer<'a>) -> Result<Parens<'a>> {
// //     parse_delimited(input, Delimiter::Parenthesis).map(|(span, content)| Parens {
// //         token: token::Paren(span),
// //         content,
// //     })
// // }

// // fn parse_delimited<'a>(
// //     input: &ParseBuffer<'a>,
// //     delimiter: Delimiter,
// // ) -> Result<(DelimSpan, ParseBuffer<'a>)> {
// //     input.step(|cursor| {
// //         if let Some((content, span, rest)) = cursor.group(delimiter) {
// //             let scope = crate::buffer::close_span_of_group(*cursor);
// //             let nested = crate::parse::advance_step_cursor(cursor, content);
// //             let unexpected = crate::parse::get_unexpected(input);
// //             let content = crate::parse::new_parse_buffer(scope, nested, unexpected);
// //             Ok(((span, content), rest))
// //         } else {
// //             let message = match delimiter {
// //                 Delimiter::Parenthesis => "expected parentheses",
// //                 Delimiter::Brace => "expected curly braces",
// //                 Delimiter::Bracket => "expected square brackets",
// //                 Delimiter::None => "expected invisible group",
// //             };
// //             Err(cursor.error(message))
// //         }
// //     })
// // }
