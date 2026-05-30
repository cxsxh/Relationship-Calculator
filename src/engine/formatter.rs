use crate::data::titles::{aliases_for, common_aliases_for, description_for, dialect_aliases_for, neutral_title_for};
use crate::models::{RelationResult, RelationType};

pub fn format_relation_path(relations: &[RelationType]) -> String {
    if relations.is_empty() {
        return "我".to_owned();
    }

    let mut parts = Vec::with_capacity(relations.len() + 1);
    parts.push("我".to_owned());
    parts.extend(relations.iter().map(|relation| relation.label().to_owned()));
    parts.join(" -> ")
}

pub fn matched_result(
    relations: &[RelationType],
    code_path_text: String,
    standard_title: &str,
) -> RelationResult {
    let common_aliases = common_aliases_for(standard_title);
    let dialect_aliases = dialect_aliases_for(standard_title);
    RelationResult {
        standard_title: standard_title.to_owned(),
        aliases: aliases_for(standard_title),
        common_aliases,
        dialect_aliases,
        neutral_title: neutral_title_for(standard_title),
        relation_path_text: format_relation_path(relations),
        code_path_text,
        is_matched: true,
        message: description_for(standard_title),
    }
}

pub fn unmatched_result(
    relations: &[RelationType],
    code_path_text: String,
    message: impl Into<String>,
) -> RelationResult {
    RelationResult {
        standard_title: "未识别".to_owned(),
        aliases: Vec::new(),
        common_aliases: Vec::new(),
        dialect_aliases: Vec::new(),
        neutral_title: None,
        relation_path_text: format_relation_path(relations),
        code_path_text,
        is_matched: false,
        message: message.into(),
    }
}
