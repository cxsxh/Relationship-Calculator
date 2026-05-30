use crate::models::RelationType;

#[derive(Debug, Clone, Copy)]
pub struct RelationDefinition {
    pub relation: RelationType,
    pub description: &'static str,
}

pub const RELATION_DEFINITIONS: [RelationDefinition; 10] = [
    RelationDefinition {
        relation: RelationType::Father,
        description: "直系父系关系",
    },
    RelationDefinition {
        relation: RelationType::Mother,
        description: "直系母系关系",
    },
    RelationDefinition {
        relation: RelationType::OlderBrother,
        description: "同辈年长男性",
    },
    RelationDefinition {
        relation: RelationType::YoungerBrother,
        description: "同辈年幼男性",
    },
    RelationDefinition {
        relation: RelationType::OlderSister,
        description: "同辈年长女性",
    },
    RelationDefinition {
        relation: RelationType::YoungerSister,
        description: "同辈年幼女性",
    },
    RelationDefinition {
        relation: RelationType::Son,
        description: "下一代男性",
    },
    RelationDefinition {
        relation: RelationType::Daughter,
        description: "下一代女性",
    },
    RelationDefinition {
        relation: RelationType::Husband,
        description: "配偶中的男性",
    },
    RelationDefinition {
        relation: RelationType::Wife,
        description: "配偶中的女性",
    },
];
