use crate::{
    diagnostic::Diagnostics,
    doc_comment::{DocComment, OutputSource},
    serde_util::is_false,
    tags::{CustomTag, FieldTag, Tag},
};
use full_moon::{
    ast::{types::TypeInfo, Stmt},
    node::Node,
    tokenizer::TokenType,
};
use serde::Serialize;

use super::DocEntryParseArguments;

#[derive(Debug, PartialEq, Serialize)]
pub struct Field {
    pub name: String,
    pub lua_type: String,
    pub desc: String,
}

impl<'a> From<FieldTag<'a>> for Field {
    fn from(field_tag: FieldTag<'a>) -> Self {
        Self {
            name: field_tag.name.as_str().to_owned(),
            lua_type: field_tag.lua_type.as_str().to_owned(),
            desc: field_tag.desc.as_str().to_owned(),
        }
    }
}

/// A DocEntry for a function or method.
#[derive(Debug, PartialEq, Serialize)]
pub struct TypeDocEntry<'a> {
    pub name: String,
    pub desc: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lua_type: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<Field>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<CustomTag<'a>>,
    #[serde(skip_serializing_if = "is_false")]
    pub private: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub ignore: bool,

    #[serde(rename = "source")]
    pub output_source: OutputSource,

    #[serde(skip)]
    pub source: &'a DocComment,
    #[serde(skip)]
    pub within: String,
}

impl<'a> TypeDocEntry<'a> {
    pub(super) fn parse(args: DocEntryParseArguments<'a>) -> Result<Self, Diagnostics> {
        let DocEntryParseArguments {
            name,
            desc,
            within,
            tags,
            source,
        } = args;

        let fields = match &source.stmt {
            Some(Stmt::ExportedTypeDeclaration(exported_type_declaration)) => {
                let type_declaration = exported_type_declaration.type_declaration();
                let type_info = type_declaration.type_definition();

                match type_info {
                    TypeInfo::Table { fields, .. } => fields
                        .iter()
                        .map(|type_field| {
                            let name = type_field
                                .key()
                                .tokens()
                                .find_map(|token| match token.token_type() {
                                    TokenType::Identifier { identifier } => {
                                        Some(identifier.to_string())
                                    }
                                    _ => None,
                                })
                                .unwrap_or(String::new());

                            let desc = type_field
                                .key()
                                .surrounding_trivia()
                                .0
                                .iter()
                                .filter_map(|trivia| match trivia.token_type() {
                                    TokenType::SingleLineComment { comment } => Some(comment),
                                    TokenType::MultiLineComment { comment, .. } => Some(comment),
                                    _ => None,
                                })
                                .map(|comment| comment.lines().map(|line| line.trim()))
                                .flatten()
                                .collect::<Vec<_>>()
                                .join("\n")
                                .trim()
                                .to_string();

                            return Field {
                                name,
                                lua_type: type_field.value().to_string(),
                                desc,
                            };
                        })
                        .collect::<Vec<_>>(),
                    _ => vec![],
                }
            }
            Some(_) => vec![],
            None => vec![],
        };

        let mut doc_entry = Self {
            name,
            desc,
            source,
            fields,
            lua_type: None,
            within: within.unwrap(),
            tags: Vec::new(),
            private: false,
            ignore: false,
            output_source: source.output_source.clone(),
        };

        let mut unused_tags = Vec::new();

        for tag in tags {
            match tag {
                Tag::Type(type_tag) => {
                    doc_entry.lua_type = Some(type_tag.lua_type.as_str().to_owned())
                }

                Tag::Field(field_tag) => doc_entry.fields.push(field_tag.into()),

                Tag::Custom(custom_tag) => doc_entry.tags.push(custom_tag),

                Tag::Private(_) => doc_entry.private = true,
                Tag::Ignore(_) => doc_entry.ignore = true,

                _ => unused_tags.push(tag),
            }
        }

        if !unused_tags.is_empty() {
            let mut diagnostics = Vec::new();
            for tag in unused_tags {
                diagnostics.push(tag.diagnostic("This tag is unused by type doc entries."));
            }

            return Err(Diagnostics::from(diagnostics));
        }

        Ok(doc_entry)
    }
}
