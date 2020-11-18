use crate::{diagnostic::Diagnostics, tags::Tag};

use super::DocEntryParseArguments;

/// A DocEntry for a class which contains functions, properties, and types
#[derive(Debug, PartialEq)]
pub struct ClassDocEntry<'a> {
    name: String,
    desc: String,
    blah: Tag<'a>,
}

impl<'a> ClassDocEntry<'a> {
    pub(super) fn parse(_args: DocEntryParseArguments) -> Result<Self, Diagnostics> {
        unimplemented!()
    }
}