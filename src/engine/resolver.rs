use crate::data::titles::TITLE_MAP;
use crate::models::RelationType;

pub fn encode_path(relations: &[RelationType]) -> String {
    relations
        .iter()
        .map(|relation| relation.code())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn resolve_title(relations: &[RelationType]) -> Option<&'static str> {
    let code_path = encode_path(relations);

    TITLE_MAP
        .get(code_path.as_str())
        .copied()
        .or_else(|| resolve_generic(relations))
}

fn resolve_generic(relations: &[RelationType]) -> Option<&'static str> {
    if let Some(title) = resolve_extended(relations) {
        return Some(title);
    }

    match relations {
        [RelationType::Father, middle, RelationType::Son]
            if matches!(
                *middle,
                RelationType::OlderBrother | RelationType::YoungerBrother
            ) =>
        {
            Some("堂兄弟")
        }
        [RelationType::Father, middle, RelationType::Daughter]
            if matches!(
                *middle,
                RelationType::OlderBrother | RelationType::YoungerBrother
            ) =>
        {
            Some("堂姐妹")
        }
        [RelationType::Father, middle, RelationType::Son]
            if matches!(
                *middle,
                RelationType::OlderSister | RelationType::YoungerSister
            ) =>
        {
            Some("表兄弟")
        }
        [RelationType::Father, middle, RelationType::Daughter]
            if matches!(
                *middle,
                RelationType::OlderSister | RelationType::YoungerSister
            ) =>
        {
            Some("表姐妹")
        }
        [RelationType::Mother, middle, RelationType::Son]
            if matches!(
                *middle,
                RelationType::OlderBrother
                    | RelationType::YoungerBrother
                    | RelationType::OlderSister
                    | RelationType::YoungerSister
            ) =>
        {
            Some("表兄弟")
        }
        [RelationType::Mother, middle, RelationType::Daughter]
            if matches!(
                *middle,
                RelationType::OlderBrother
                    | RelationType::YoungerBrother
                    | RelationType::OlderSister
                    | RelationType::YoungerSister
            ) =>
        {
            Some("表姐妹")
        }
        [RelationType::OlderBrother, _, RelationType::Son]
        | [RelationType::YoungerBrother, _, RelationType::Son] => Some("侄子"),
        [RelationType::OlderBrother, _, RelationType::Daughter]
        | [RelationType::YoungerBrother, _, RelationType::Daughter] => Some("侄女"),
        [RelationType::OlderSister, _, RelationType::Son]
        | [RelationType::YoungerSister, _, RelationType::Son] => Some("外甥"),
        [RelationType::OlderSister, _, RelationType::Daughter]
        | [RelationType::YoungerSister, _, RelationType::Daughter] => Some("外甥女"),
        _ => None,
    }
}

fn resolve_extended(relations: &[RelationType]) -> Option<&'static str> {
    if relations.len() < 2 {
        return None;
    }

    let (prefix, last) = relations.split_at(relations.len() - 1);
    let child_is_male = match last.first()? {
        RelationType::Son => true,
        RelationType::Daughter => false,
        _ => return None,
    };

    let parent_title = resolve_title(prefix)?;
    title_of_descendant(parent_title, child_is_male)
}

fn title_of_descendant(parent_title: &str, male: bool) -> Option<&'static str> {
    match (parent_title, male) {
        ("伯父", true) | ("叔叔", true) => Some("堂兄弟"),
        ("伯父", false) | ("叔叔", false) => Some("堂姐妹"),
        ("姑妈", true) | ("小姑", true) | ("舅舅", true) | ("姨妈", true) | ("小姨", true) => {
            Some("表兄弟")
        }
        ("姑妈", false)
        | ("小姑", false)
        | ("舅舅", false)
        | ("姨妈", false)
        | ("小姨", false) => Some("表姐妹"),

        ("哥哥", true) | ("弟弟", true) => Some("侄子"),
        ("哥哥", false) | ("弟弟", false) => Some("侄女"),
        ("姐姐", true) | ("妹妹", true) => Some("外甥"),
        ("姐姐", false) | ("妹妹", false) => Some("外甥女"),

        ("堂兄弟", true) | ("堂姐妹", true) => Some("堂侄"),
        ("堂兄弟", false) | ("堂姐妹", false) => Some("堂侄女"),
        ("表兄弟", true) | ("表姐妹", true) => Some("表侄"),
        ("表兄弟", false) | ("表姐妹", false) => Some("表侄女"),

        ("侄子", true) | ("侄女", true) => Some("侄孙"),
        ("侄子", false) | ("侄女", false) => Some("侄孙女"),
        ("外甥", true) | ("外甥女", true) => Some("外甥孙"),
        ("外甥", false) | ("外甥女", false) => Some("外甥孙女"),

        _ => None,
    }
}
