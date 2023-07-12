use std::fmt::Debug;

use crate::macro_match::MacroMatch;
use crate::repetition::{MacroRepOp, MacroRepSep, OneOrMore};

#[derive(Debug)]
pub(crate) struct MacroMatchRep {
    matches: OneOrMore<Box<MacroMatch>>,
    sep: MacroRepSep,
    op: MacroRepOp,
}
