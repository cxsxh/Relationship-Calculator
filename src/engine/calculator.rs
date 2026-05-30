use crate::engine::{formatter, resolver};
use crate::models::{RelationResult, RelationType};

pub const MAX_STEPS: usize = 10;

#[derive(Debug, Default, Clone, Copy)]
pub struct RelationCalculator;

impl RelationCalculator {
    pub fn new() -> Self {
        Self
    }

    pub fn calculate(&self, relations: &[RelationType]) -> RelationResult {
        if relations.is_empty() {
            return RelationResult::prompt();
        }

        if relations.len() > MAX_STEPS {
            return formatter::unmatched_result(
                relations,
                resolver::encode_path(relations),
                format!("当前版本最多支持 {MAX_STEPS} 步关系链，请缩短后再计算。"),
            );
        }

        let code_path = resolver::encode_path(relations);

        match resolver::resolve_title(relations) {
            Some(title) => formatter::matched_result(relations, code_path, title),
            None => formatter::unmatched_result(
                relations,
                code_path,
                "当前版本暂未收录该关系组合，但路径和编码已保留，方便后续继续扩展。",
            ),
        }
    }
}
