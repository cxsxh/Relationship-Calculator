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
