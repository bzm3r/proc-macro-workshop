use proc_macro2::{Punct, Span, Spacing as Spacing2};
use syn::parse::Parser;

use std::fmt::Debug;
use std::fmt::{Display, Formatter, Result as FmtResult};

use syn::{
    parse::{Parse, ParseStream, Result as ParseResult},
    token::{Brace as SynBrace, Bracket as SynBracket, Paren as SynParen},
};

pub(crate) trait LegalDelimPairSeal {
    fn left(&self) -> LegalDelimiter;
    fn right(&self) -> LegalDelimiter;
    fn left_span(&self) -> Span;
    fn right_span(&self) -> Span;
}

pub(crate) enum LegalDelimPair {
    Brace(SynBrace),
    Bracket(SynBracket),
    Paren(SynParen),
    Custom {
        left: LegalDelimiter,
        right: LegalDelimiter,
    },
}

trait LegalPunctForDelimSeal {}

#[derive(Debug, Clone)]
pub(crate) struct PunctString {
    span: Span,
    puncts: Vec<LegalPunct>,
    spacing: Spacing2,
}

pub(crate) struct PunctStringParser {
    puncts: Vec<Punct>,
    spacing: Spacing2,
}

impl Parser for PunctStringParser {
    fn parse2(self, tokens: proc_macro2::TokenStream) -> ParseResult<Self::Output> {
        let mut legal_puncts = Vec::with_capacity(self.puncts.len());
        self.puncts[..(self.puncts.len() - 1)].for_each(|punct| )
    }
}

impl Parse for PunctString {
    fn parse(_input: ParseStream) -> ParseResult<Self> {
        unimplemented!()
    }
}

impl LegalPunctForDelimSeal for PunctString {}

pub(crate) trait LegalDelimiterSeal {}

pub(crate) struct LegalDelimiter {
    start: PunctString,
    end: PunctString,
}

impl LegalDelimiterSeal for LegalDelimiter {}

impl Parse for LegalDelimiter {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        match self {}
    }
}

impl LegalPunctForDelimSeal for LegalPunct {}

macro_rules! legal_punct {
    ( $($id:ident : $p:literal),* ) => {
        #[derive(Debug, Clone)]
        pub(crate) enum LegalPunct {
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

        impl From<(Punct, Span)> for LegalPunct {
            fn from(punct_and_span: (Punct, Span)) -> LegalPunct {
                let (punct, span) = punct_and_span;
                match punct.as_char().to_string().as_str() {
                    $($p => {
                        Self::$id {
                            punct,
                            span,
                        }
                    }),*
                    _ => {
                        unreachable!("Should be able to convert all `Punct` into `LegalPunct`. Tried to convert {}", punct.as_char());
                    },
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

impl Parse for LegalPunct {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let span = input.cursor().span();
        let punct: Punct = input.parse()?;
        Ok((punct, span).into())
    }
}

pub(crate) trait LegalPunctSeal {}

impl LegalPunctSeal for LegalPunct {}

impl Parse for LegalDelimiter {
    fn parse(_input: ParseStream) -> ParseResult<LegalDelimiter> {
        // input.step(|cursor| cursor.punct()?)
        unimplemented!()
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
