use std::convert::TryFrom;

use crate::{diagnostic::Diagnostic, span::Span};

#[derive(Debug, PartialEq)]
pub struct WithinTag<'a> {
    pub name: Span<'a>,
    pub source: Span<'a>,
}

impl<'a> TryFrom<Span<'a>> for WithinTag<'a> {
    type Error = Diagnostic;

    fn try_from(span: Span<'a>) -> Result<Self, Self::Error> {
        if span.is_empty() {
            return Err(span.diagnostic("This tag has stuff after it"));
        }

        Ok(Self {
            name: span,
            source: span,
        })
    }
}