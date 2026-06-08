use relationship_calculator::{RelationCalculator, RelationType};

#[test]
fn computes_uncle_from_mother_and_younger_brother() {
    let calculator = RelationCalculator::default();
    let relations = [RelationType::Mother, RelationType::YoungerBrother];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert_eq!(result.standard_title, "舅舅");
    assert!(result.aliases.iter().any(|alias| alias == "舅"));
}

#[test]
fn computes_paternal_uncle_from_father_and_older_brother() {
    let calculator = RelationCalculator::default();
    let relations = [RelationType::Father, RelationType::OlderBrother];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert_eq!(result.standard_title, "伯父");
}

#[test]
fn computes_nephew_from_older_brother_and_son() {
    let calculator = RelationCalculator::default();
    let relations = [RelationType::OlderBrother, RelationType::Son];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert_eq!(result.standard_title, "侄子");
}

#[test]
fn computes_aunt_from_mother_and_older_sister() {
    let calculator = RelationCalculator::default();
    let relations = [RelationType::Mother, RelationType::OlderSister];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert_eq!(result.standard_title, "姨妈");
}

#[test]
fn computes_cousin_group_title() {
    let calculator = RelationCalculator::default();
    let relations = [
        RelationType::Mother,
        RelationType::OlderSister,
        RelationType::Daughter,
    ];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert_eq!(result.standard_title, "表姐妹");
    assert_eq!(result.neutral_title.as_deref(), Some("表亲"));
}

#[test]
fn rejects_overlong_relation_chain() {
    let calculator = RelationCalculator::default();
    let relations = [
        RelationType::Father,
        RelationType::Mother,
        RelationType::Father,
        RelationType::Mother,
        RelationType::Father,
        RelationType::Mother,
        RelationType::Father,
        RelationType::Mother,
        RelationType::Father,
        RelationType::Mother,
    ];
    let result = calculator.calculate(&relations);

    assert!(!result.is_matched);
    assert!(!result.message.is_empty());
}

#[test]
fn computes_mothers_younger_sisters_son_as_male_cousin() {
    let calculator = RelationCalculator::default();
    let relations = [
        RelationType::Mother,
        RelationType::YoungerSister,
        RelationType::Son,
    ];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert_eq!(result.standard_title, "表兄弟");
    assert!(result.aliases.iter().any(|alias| alias.contains("表")));
}

#[test]
fn computes_mothers_younger_sisters_son_then_son() {
    let calculator = RelationCalculator::default();
    let relations = [
        RelationType::Mother,
        RelationType::YoungerSister,
        RelationType::Son,
        RelationType::Son,
    ];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert_eq!(result.standard_title, "表侄");
}

#[test]
fn does_not_overmatch_maternal_grandfathers_son_as_cousin() {
    let calculator = RelationCalculator::default();
    let relations = [
        RelationType::Mother,
        RelationType::Father,
        RelationType::Son,
    ];
    let result = calculator.calculate(&relations);

    assert!(!result.is_matched);
}

#[test]
fn includes_common_and_dialect_aliases() {
    let calculator = RelationCalculator::default();
    let relations = [RelationType::Father];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert!(result.aliases.iter().any(|alias| alias == "阿爸"));
    assert!(result.common_aliases.iter().any(|alias| alias == "爸爸"));
    assert!(result.dialect_aliases.iter().any(|alias| alias == "阿爸"));
}

#[test]
fn provides_neutral_title_for_nephew_branch() {
    let calculator = RelationCalculator::default();
    let relations = [RelationType::OlderBrother, RelationType::Son];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert_eq!(result.standard_title, "侄子");
    assert_eq!(result.neutral_title.as_deref(), Some("侄辈"));
}

#[test]
fn allows_longer_chain_within_limit() {
    let calculator = RelationCalculator::default();
    let relations = [
        RelationType::Mother,
        RelationType::YoungerSister,
        RelationType::Son,
        RelationType::Son,
        RelationType::Daughter,
        RelationType::Son,
        RelationType::Daughter,
        RelationType::Son,
        RelationType::Daughter,
        RelationType::Son,
    ];
    let result = calculator.calculate(&relations);

    assert!(result.relation_path_text.contains("我 ->"));
    assert!(!result.message.contains("最多支持"));
}

#[test]
fn computes_direct_descendants_to_fifth_generation() {
    let calculator = RelationCalculator::default();
    let relations = [
        RelationType::Son,
        RelationType::Son,
        RelationType::Son,
        RelationType::Son,
        RelationType::Son,
    ];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert_eq!(result.standard_title, "来孙");
}

#[test]
fn computes_maternal_descendants_to_fifth_generation() {
    let calculator = RelationCalculator::default();
    let relations = [
        RelationType::Daughter,
        RelationType::Son,
        RelationType::Son,
        RelationType::Son,
        RelationType::Daughter,
    ];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert_eq!(result.standard_title, "外来孙女");
}

#[test]
fn computes_nephew_branch_to_fifth_generation() {
    let calculator = RelationCalculator::default();
    let relations = [
        RelationType::OlderBrother,
        RelationType::Son,
        RelationType::Son,
        RelationType::Son,
        RelationType::Son,
        RelationType::Son,
    ];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert_eq!(result.standard_title, "侄来孙");
}

#[test]
fn computes_paternal_cousin_branch_to_fifth_generation() {
    let calculator = RelationCalculator::default();
    let relations = [
        RelationType::Father,
        RelationType::OlderBrother,
        RelationType::Son,
        RelationType::Son,
        RelationType::Son,
        RelationType::Son,
        RelationType::Son,
        RelationType::Daughter,
    ];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert_eq!(result.standard_title, "堂侄来孙女");
}

#[test]
fn computes_maternal_cousin_branch_to_fifth_generation() {
    let calculator = RelationCalculator::default();
    let relations = [
        RelationType::Mother,
        RelationType::YoungerSister,
        RelationType::Son,
        RelationType::Son,
        RelationType::Son,
        RelationType::Son,
        RelationType::Son,
        RelationType::Son,
    ];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert_eq!(result.standard_title, "表侄来孙");
}

#[test]
fn provides_display_metadata_for_fifth_generation_direct_descendant() {
    let calculator = RelationCalculator::default();
    let relations = [
        RelationType::Son,
        RelationType::Son,
        RelationType::Son,
        RelationType::Son,
        RelationType::Daughter,
    ];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert_eq!(result.standard_title, "来孙女");
    assert!(result.aliases.iter().any(|alias| alias == "来孙姑娘"));
    assert_eq!(result.neutral_title.as_deref(), Some("来孙辈"));
    assert!(result.message.contains("玄孙辈"));
}

#[test]
fn provides_display_metadata_for_fifth_generation_cousin_branch() {
    let calculator = RelationCalculator::default();
    let relations = [
        RelationType::Father,
        RelationType::OlderBrother,
        RelationType::Son,
        RelationType::Son,
        RelationType::Son,
        RelationType::Son,
        RelationType::Son,
        RelationType::Son,
    ];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert_eq!(result.standard_title, "堂侄来孙");
    assert!(result.aliases.iter().any(|alias| alias == "堂侄来孙子"));
    assert_eq!(result.neutral_title.as_deref(), Some("堂侄来孙辈"));
    assert!(result.message.contains("堂侄玄孙辈"));
}

#[test]
fn computes_paternal_ancestors_to_fifth_generation() {
    let calculator = RelationCalculator::default();
    let relations = [
        RelationType::Father,
        RelationType::Father,
        RelationType::Father,
        RelationType::Father,
        RelationType::Father,
    ];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert_eq!(result.standard_title, "曾高祖父");
    assert!(result.aliases.iter().any(|alias| alias == "曾高祖"));
    assert_eq!(result.neutral_title.as_deref(), Some("曾高祖辈"));
}

#[test]
fn computes_maternal_ancestors_to_fifth_generation() {
    let calculator = RelationCalculator::default();
    let relations = [
        RelationType::Mother,
        RelationType::Mother,
        RelationType::Father,
        RelationType::Father,
        RelationType::Mother,
    ];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert_eq!(result.standard_title, "外曾高祖母");
    assert!(result.aliases.iter().any(|alias| alias == "外曾高祖婆"));
    assert_eq!(result.neutral_title.as_deref(), Some("外曾高祖辈"));
}

#[test]
fn computes_uncles_parent_branch_upwards() {
    let calculator = RelationCalculator::default();
    let relations = [
        RelationType::Father,
        RelationType::YoungerBrother,
        RelationType::Father,
        RelationType::Father,
    ];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert_eq!(result.standard_title, "太爷爷");
}

#[test]
fn computes_maternal_uncle_branch_upwards() {
    let calculator = RelationCalculator::default();
    let relations = [
        RelationType::Mother,
        RelationType::OlderBrother,
        RelationType::Mother,
        RelationType::Father,
    ];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert_eq!(result.standard_title, "外曾外祖父");
}

#[test]
fn computes_husbands_paternal_ancestors() {
    let calculator = RelationCalculator::default();
    let relations = [
        RelationType::Husband,
        RelationType::Father,
        RelationType::Father,
        RelationType::Father,
        RelationType::Father,
    ];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert_eq!(result.standard_title, "夫高祖父");
    assert!(result.aliases.iter().any(|alias| alias == "夫家高祖父"));
    assert_eq!(result.neutral_title.as_deref(), Some("夫高祖辈"));
}

#[test]
fn computes_husbands_maternal_ancestors() {
    let calculator = RelationCalculator::default();
    let relations = [
        RelationType::Husband,
        RelationType::Mother,
        RelationType::Father,
        RelationType::Father,
        RelationType::Mother,
    ];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert_eq!(result.standard_title, "夫外高祖母");
    assert!(result.aliases.iter().any(|alias| alias == "夫家外高祖母"));
}

#[test]
fn computes_wifes_paternal_ancestors() {
    let calculator = RelationCalculator::default();
    let relations = [
        RelationType::Wife,
        RelationType::Father,
        RelationType::Father,
        RelationType::Father,
        RelationType::Mother,
    ];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert_eq!(result.standard_title, "妻高祖母");
    assert!(result.aliases.iter().any(|alias| alias == "妻家高祖母"));
}

#[test]
fn computes_wifes_maternal_ancestors_to_fifth_generation() {
    let calculator = RelationCalculator::default();
    let relations = [
        RelationType::Wife,
        RelationType::Mother,
        RelationType::Father,
        RelationType::Father,
        RelationType::Father,
        RelationType::Father,
    ];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert_eq!(result.standard_title, "妻外曾高祖父");
    assert_eq!(result.neutral_title.as_deref(), Some("妻外曾高祖辈"));
}

#[test]
fn computes_spouse_of_descendant_branch() {
    let calculator = RelationCalculator::default();
    let relations = [RelationType::Son, RelationType::Wife];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert_eq!(result.standard_title, "儿媳");
}

#[test]
fn computes_spouse_of_nephew_branch() {
    let calculator = RelationCalculator::default();
    let relations = [RelationType::OlderBrother, RelationType::Son, RelationType::Wife];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert_eq!(result.standard_title, "侄媳妇");
}

#[test]
fn computes_spouse_of_cousin_branch() {
    let calculator = RelationCalculator::default();
    let relations = [
        RelationType::Mother,
        RelationType::YoungerSister,
        RelationType::Daughter,
        RelationType::Husband,
    ];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert_eq!(result.standard_title, "表姐妹夫");
}

#[test]
fn computes_spouse_of_in_law_ancestor() {
    let calculator = RelationCalculator::default();
    let relations = [RelationType::Husband, RelationType::Father, RelationType::Mother];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert_eq!(result.standard_title, "夫祖母");
}

#[test]
fn computes_spouse_of_fifth_generation_branch() {
    let calculator = RelationCalculator::default();
    let relations = [
        RelationType::OlderBrother,
        RelationType::Son,
        RelationType::Son,
        RelationType::Son,
        RelationType::Son,
        RelationType::Daughter,
        RelationType::Husband,
    ];
    let result = calculator.calculate(&relations);

    assert!(result.is_matched);
    assert_eq!(result.standard_title, "侄来孙女婿");
}
