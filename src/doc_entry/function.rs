use crate::{
    diagnostic::Diagnostics,
    doc_comment::DocComment,
    tags::{DeprecatedTag, MarkerTag, ParamTag, ReturnTag, Tag},
};
use serde::Serialize;

use super::DocEntryParseArguments;

/// Used to separate functions (called with a dot) from methods (called with a colon)
#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FunctionType {
    Method,
    Static,
}

/// A DocEntry for a function or method.
#[derive(Debug, PartialEq, Serialize)]
pub struct FunctionDocEntry<'a> {
    pub name: String,
    pub desc: String,
    pub within: String,
    pub params: Vec<ParamTag<'a>>,
    pub returns: Vec<ReturnTag<'a>>,
    pub markers: Vec<MarkerTag<'a>>,
    pub function_type: FunctionType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<DeprecatedTag<'a>>,

    #[serde(skip)]
    pub source: &'a DocComment,
}

impl<'a> FunctionDocEntry<'a> {
    pub(super) fn parse(
        args: DocEntryParseArguments<'a>,
        function_type: FunctionType,
    ) -> Result<Self, Diagnostics> {
        let DocEntryParseArguments {
            name,
            desc,
            within,
            tags,
            source,
        } = args;

        let within = within.unwrap();
        let mut params = Vec::new();
        let mut returns = Vec::new();
        let mut markers = Vec::new();
        let mut unused_tags = Vec::new();
        let mut deprecated = None;
        let mut since = None;

        for tag in tags {
            match tag {
                Tag::Param(param) => params.push(param),
                Tag::Return(return_tag) => returns.push(return_tag),
                Tag::Marker(marker) => markers.push(marker),
                Tag::Deprecated(deprecated_tag) => deprecated = Some(deprecated_tag),
                Tag::Since(since_tag) => since = Some(since_tag.version.to_string()),
                _ => unused_tags.push(tag),
            }
        }

        if !unused_tags.is_empty() {
            let mut diagnostics = Vec::new();
            for tag in unused_tags {
                diagnostics.push(tag.diagnostic("This tag is unused by function doc entries."));
            }

            return Err(Diagnostics::from(diagnostics));
        }

        Ok(Self {
            name,
            desc,
            params,
            returns,
            markers,
            function_type,
            within,
            deprecated,
            source,
            since,
        })
    }
}
