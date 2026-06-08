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
    resolve_extended(relations)
}

fn resolve_extended(relations: &[RelationType]) -> Option<&'static str> {
    if relations.len() < 2 {
        return None;
    }

    let (prefix, last) = relations.split_at(relations.len() - 1);
    let parent_title = resolve_title(prefix)?;

    match last.first()? {
        RelationType::Son => title_of_descendant(parent_title, true),
        RelationType::Daughter => title_of_descendant(parent_title, false),
        RelationType::Father => title_of_ancestor(parent_title, true),
        RelationType::Mother => title_of_ancestor(parent_title, false),
        RelationType::Husband => title_of_spouse(parent_title, true),
        RelationType::Wife => title_of_spouse(parent_title, false),
        _ => None,
    }

}

fn title_of_descendant(parent_title: &str, male: bool) -> Option<&'static str> {
    descendant_seed(parent_title, male).or_else(|| descendant_branch_of(parent_title)?.descend(male))
}

fn title_of_ancestor(parent_title: &str, father: bool) -> Option<&'static str> {
    ancestor_seed(parent_title, father).or_else(|| ancestor_branch_of(parent_title)?.ascend(father))
}

fn title_of_spouse(parent_title: &str, husband: bool) -> Option<&'static str> {
    spouse_seed(parent_title, husband).or_else(|| spouse_branch_of(parent_title)?.spouse(husband))
}

#[derive(Debug, Clone, Copy)]
enum DescendantBranch {
    DirectPatrilineal(u8),
    DirectMaternal(u8),
    Nephew(u8),
    MaternalNephew(u8),
    PaternalCousin(u8),
    MaternalCousin(u8),
}

#[derive(Debug, Clone, Copy)]
enum AncestorBranch {
    Paternal(u8),
    Maternal(u8),
    HusbandPaternal(u8),
    HusbandMaternal(u8),
    WifePaternal(u8),
    WifeMaternal(u8),
}

#[derive(Debug, Clone, Copy)]
enum SpouseBranch {
    DirectPatrilineal(u8),
    DirectMaternal(u8),
    Nephew(u8),
    MaternalNephew(u8),
    PaternalCousin(u8),
    MaternalCousin(u8),
}

impl AncestorBranch {
    fn ascend(self, father: bool) -> Option<&'static str> {
        match self {
            Self::Paternal(depth) => paternal_ancestor_title(depth + 1, father),
            Self::Maternal(depth) => maternal_ancestor_title(depth + 1, father),
            Self::HusbandPaternal(depth) => husband_paternal_ancestor_title(depth + 1, father),
            Self::HusbandMaternal(depth) => husband_maternal_ancestor_title(depth + 1, father),
            Self::WifePaternal(depth) => wife_paternal_ancestor_title(depth + 1, father),
            Self::WifeMaternal(depth) => wife_maternal_ancestor_title(depth + 1, father),
        }
    }
}

impl SpouseBranch {
    fn spouse(self, husband: bool) -> Option<&'static str> {
        match self {
            Self::DirectPatrilineal(depth) => direct_patrilineal_spouse_title(depth, husband),
            Self::DirectMaternal(depth) => direct_maternal_spouse_title(depth, husband),
            Self::Nephew(depth) => nephew_spouse_title(depth, husband),
            Self::MaternalNephew(depth) => maternal_nephew_spouse_title(depth, husband),
            Self::PaternalCousin(depth) => paternal_cousin_spouse_title(depth, husband),
            Self::MaternalCousin(depth) => maternal_cousin_spouse_title(depth, husband),
        }
    }
}

impl DescendantBranch {
    fn descend(self, male: bool) -> Option<&'static str> {
        match self {
            Self::DirectPatrilineal(depth) => direct_patrilineal_title(depth + 1, male),
            Self::DirectMaternal(depth) => direct_maternal_title(depth + 1, male),
            Self::Nephew(depth) => nephew_title(depth + 1, male),
            Self::MaternalNephew(depth) => maternal_nephew_title(depth + 1, male),
            Self::PaternalCousin(depth) => paternal_cousin_title(depth + 1, male),
            Self::MaternalCousin(depth) => maternal_cousin_title(depth + 1, male),
        }
    }
}

fn ancestor_seed(parent_title: &str, father: bool) -> Option<&'static str> {
    match parent_title {
        "哥哥" | "弟弟" | "姐姐" | "妹妹" => Some(if father { "爸爸" } else { "妈妈" }),
        "伯父" | "叔叔" | "姑妈" | "小姑" | "伯母" | "婶婶" | "姑父" => {
            paternal_ancestor_title(2, father)
        }
        "舅舅" | "姨妈" | "小姨" | "舅妈" | "姨父" => maternal_ancestor_title(2, father),
        "大伯子" | "小叔子" | "大姑子" | "小姑子" => {
            husband_paternal_ancestor_title(2, father)
        }
        "大舅哥" | "小舅子" | "大姨子" | "小姨子" => wife_paternal_ancestor_title(2, father),
        _ => None,
    }
}

fn spouse_seed(parent_title: &str, husband: bool) -> Option<&'static str> {
    match (parent_title, husband) {
        ("哥哥", false) => Some("嫂子"),
        ("弟弟", false) => Some("弟妹"),
        ("姐姐", true) => Some("姐夫"),
        ("妹妹", true) => Some("妹夫"),

        ("伯父", false) => Some("伯母"),
        ("叔叔", false) => Some("婶婶"),
        ("姑妈", true) | ("小姑", true) => Some("姑父"),
        ("舅舅", false) => Some("舅妈"),
        ("姨妈", true) | ("小姨", true) => Some("姨父"),

        ("公公", false) => Some("婆婆"),
        ("婆婆", true) => Some("公公"),
        ("岳父", false) => Some("岳母"),
        ("岳母", true) => Some("岳父"),

        ("爷爷", false) => Some("奶奶"),
        ("奶奶", true) => Some("爷爷"),
        ("外公", false) => Some("外婆"),
        ("外婆", true) => Some("外公"),
        ("太爷爷", false) => Some("太奶奶"),
        ("太奶奶", true) => Some("太爷爷"),
        ("外曾外祖父", false) => Some("外曾外祖母"),
        ("外曾外祖母", true) => Some("外曾外祖父"),
        ("高祖父", false) => Some("高祖母"),
        ("高祖母", true) => Some("高祖父"),
        ("外高祖父", false) => Some("外高祖母"),
        ("外高祖母", true) => Some("外高祖父"),
        ("曾高祖父", false) => Some("曾高祖母"),
        ("曾高祖母", true) => Some("曾高祖父"),
        ("外曾高祖父", false) => Some("外曾高祖母"),
        ("外曾高祖母", true) => Some("外曾高祖父"),

        ("夫祖父", false) => Some("夫祖母"),
        ("夫祖母", true) => Some("夫祖父"),
        ("夫外祖父", false) => Some("夫外祖母"),
        ("夫外祖母", true) => Some("夫外祖父"),
        ("夫曾祖父", false) => Some("夫曾祖母"),
        ("夫曾祖母", true) => Some("夫曾祖父"),
        ("夫外曾祖父", false) => Some("夫外曾祖母"),
        ("夫外曾祖母", true) => Some("夫外曾祖父"),
        ("夫高祖父", false) => Some("夫高祖母"),
        ("夫高祖母", true) => Some("夫高祖父"),
        ("夫外高祖父", false) => Some("夫外高祖母"),
        ("夫外高祖母", true) => Some("夫外高祖父"),
        ("夫曾高祖父", false) => Some("夫曾高祖母"),
        ("夫曾高祖母", true) => Some("夫曾高祖父"),
        ("夫外曾高祖父", false) => Some("夫外曾高祖母"),
        ("夫外曾高祖母", true) => Some("夫外曾高祖父"),

        ("妻祖父", false) => Some("妻祖母"),
        ("妻祖母", true) => Some("妻祖父"),
        ("妻外祖父", false) => Some("妻外祖母"),
        ("妻外祖母", true) => Some("妻外祖父"),
        ("妻曾祖父", false) => Some("妻曾祖母"),
        ("妻曾祖母", true) => Some("妻曾祖父"),
        ("妻外曾祖父", false) => Some("妻外曾祖母"),
        ("妻外曾祖母", true) => Some("妻外曾祖父"),
        ("妻高祖父", false) => Some("妻高祖母"),
        ("妻高祖母", true) => Some("妻高祖父"),
        ("妻外高祖父", false) => Some("妻外高祖母"),
        ("妻外高祖母", true) => Some("妻外高祖父"),
        ("妻曾高祖父", false) => Some("妻曾高祖母"),
        ("妻曾高祖母", true) => Some("妻曾高祖父"),
        ("妻外曾高祖父", false) => Some("妻外曾高祖母"),
        ("妻外曾高祖母", true) => Some("妻外曾高祖父"),
        _ => None,
    }
}

fn descendant_seed(parent_title: &str, male: bool) -> Option<&'static str> {
    match parent_title {
        "丈夫" | "妻子" => Some(if male { "儿子" } else { "女儿" }),
        "儿媳" => direct_patrilineal_title(2, male),
        "女婿" => direct_maternal_title(2, male),

        "伯父" | "叔叔" | "伯母" | "婶婶" => paternal_cousin_title(0, male),
        "姑妈" | "小姑" | "姑父" | "舅舅" | "舅妈" | "姨妈" | "小姨" | "姨父" => {
            maternal_cousin_title(0, male)
        }

        "哥哥" | "弟弟" | "嫂子" | "弟妹" | "大伯子" | "小叔子" | "大舅哥" | "小舅子" => {
            nephew_title(1, male)
        }
        "姐姐" | "妹妹" | "姐夫" | "妹夫" | "大姑子" | "小姑子" | "大姨子" | "小姨子" => {
            maternal_nephew_title(1, male)
        }
        _ => None,
    }
}

fn ancestor_branch_of(title: &str) -> Option<AncestorBranch> {
    match title {
        "爸爸" => Some(AncestorBranch::Paternal(1)),
        "妈妈" => Some(AncestorBranch::Maternal(1)),
        "公公" => Some(AncestorBranch::HusbandPaternal(1)),
        "婆婆" => Some(AncestorBranch::HusbandMaternal(1)),
        "岳父" => Some(AncestorBranch::WifePaternal(1)),
        "岳母" => Some(AncestorBranch::WifeMaternal(1)),
        "爷爷" | "奶奶" => Some(AncestorBranch::Paternal(2)),
        "外公" | "外婆" => Some(AncestorBranch::Maternal(2)),
        "夫祖父" | "夫祖母" => Some(AncestorBranch::HusbandPaternal(2)),
        "夫外祖父" | "夫外祖母" => Some(AncestorBranch::HusbandMaternal(2)),
        "妻祖父" | "妻祖母" => Some(AncestorBranch::WifePaternal(2)),
        "妻外祖父" | "妻外祖母" => Some(AncestorBranch::WifeMaternal(2)),
        "太爷爷" | "太奶奶" => Some(AncestorBranch::Paternal(3)),
        "外曾外祖父" | "外曾外祖母" => Some(AncestorBranch::Maternal(3)),
        "夫曾祖父" | "夫曾祖母" => Some(AncestorBranch::HusbandPaternal(3)),
        "夫外曾祖父" | "夫外曾祖母" => Some(AncestorBranch::HusbandMaternal(3)),
        "妻曾祖父" | "妻曾祖母" => Some(AncestorBranch::WifePaternal(3)),
        "妻外曾祖父" | "妻外曾祖母" => Some(AncestorBranch::WifeMaternal(3)),
        "高祖父" | "高祖母" => Some(AncestorBranch::Paternal(4)),
        "外高祖父" | "外高祖母" => Some(AncestorBranch::Maternal(4)),
        "夫高祖父" | "夫高祖母" => Some(AncestorBranch::HusbandPaternal(4)),
        "夫外高祖父" | "夫外高祖母" => Some(AncestorBranch::HusbandMaternal(4)),
        "妻高祖父" | "妻高祖母" => Some(AncestorBranch::WifePaternal(4)),
        "妻外高祖父" | "妻外高祖母" => Some(AncestorBranch::WifeMaternal(4)),
        "曾高祖父" | "曾高祖母" => Some(AncestorBranch::Paternal(5)),
        "外曾高祖父" | "外曾高祖母" => Some(AncestorBranch::Maternal(5)),
        "夫曾高祖父" | "夫曾高祖母" => Some(AncestorBranch::HusbandPaternal(5)),
        "夫外曾高祖父" | "夫外曾高祖母" => Some(AncestorBranch::HusbandMaternal(5)),
        "妻曾高祖父" | "妻曾高祖母" => Some(AncestorBranch::WifePaternal(5)),
        "妻外曾高祖父" | "妻外曾高祖母" => Some(AncestorBranch::WifeMaternal(5)),
        _ => None,
    }
}

fn spouse_branch_of(title: &str) -> Option<SpouseBranch> {
    match title {
        "儿子" => Some(SpouseBranch::DirectPatrilineal(1)),
        "女儿" => Some(SpouseBranch::DirectMaternal(1)),
        "孙子" | "孙女" => Some(SpouseBranch::DirectPatrilineal(2)),
        "外孙" | "外孙女" => Some(SpouseBranch::DirectMaternal(2)),
        "曾孙" | "曾孙女" => Some(SpouseBranch::DirectPatrilineal(3)),
        "外曾孙" | "外曾孙女" => Some(SpouseBranch::DirectMaternal(3)),
        "玄孙" | "玄孙女" => Some(SpouseBranch::DirectPatrilineal(4)),
        "外玄孙" | "外玄孙女" => Some(SpouseBranch::DirectMaternal(4)),
        "来孙" | "来孙女" => Some(SpouseBranch::DirectPatrilineal(5)),
        "外来孙" | "外来孙女" => Some(SpouseBranch::DirectMaternal(5)),

        "侄子" | "侄女" => Some(SpouseBranch::Nephew(1)),
        "外甥" | "外甥女" => Some(SpouseBranch::MaternalNephew(1)),
        "侄孙" | "侄孙女" => Some(SpouseBranch::Nephew(2)),
        "外甥孙" | "外甥孙女" => Some(SpouseBranch::MaternalNephew(2)),
        "侄曾孙" | "侄曾孙女" => Some(SpouseBranch::Nephew(3)),
        "外甥曾孙" | "外甥曾孙女" => Some(SpouseBranch::MaternalNephew(3)),
        "侄玄孙" | "侄玄孙女" => Some(SpouseBranch::Nephew(4)),
        "外甥玄孙" | "外甥玄孙女" => Some(SpouseBranch::MaternalNephew(4)),
        "侄来孙" | "侄来孙女" => Some(SpouseBranch::Nephew(5)),
        "外甥来孙" | "外甥来孙女" => Some(SpouseBranch::MaternalNephew(5)),

        "堂兄弟" | "堂姐妹" => Some(SpouseBranch::PaternalCousin(0)),
        "表兄弟" | "表姐妹" => Some(SpouseBranch::MaternalCousin(0)),
        "堂侄" | "堂侄女" => Some(SpouseBranch::PaternalCousin(1)),
        "表侄" | "表侄女" => Some(SpouseBranch::MaternalCousin(1)),
        "堂侄孙" | "堂侄孙女" => Some(SpouseBranch::PaternalCousin(2)),
        "表侄孙" | "表侄孙女" => Some(SpouseBranch::MaternalCousin(2)),
        "堂侄曾孙" | "堂侄曾孙女" => Some(SpouseBranch::PaternalCousin(3)),
        "表侄曾孙" | "表侄曾孙女" => Some(SpouseBranch::MaternalCousin(3)),
        "堂侄玄孙" | "堂侄玄孙女" => Some(SpouseBranch::PaternalCousin(4)),
        "表侄玄孙" | "表侄玄孙女" => Some(SpouseBranch::MaternalCousin(4)),
        "堂侄来孙" | "堂侄来孙女" => Some(SpouseBranch::PaternalCousin(5)),
        "表侄来孙" | "表侄来孙女" => Some(SpouseBranch::MaternalCousin(5)),
        _ => None,
    }
}

fn descendant_branch_of(title: &str) -> Option<DescendantBranch> {
    match title {
        "儿子" => Some(DescendantBranch::DirectPatrilineal(1)),
        "女儿" => Some(DescendantBranch::DirectMaternal(1)),
        "孙子" | "孙女" => Some(DescendantBranch::DirectPatrilineal(2)),
        "外孙" | "外孙女" => Some(DescendantBranch::DirectMaternal(2)),
        "曾孙" | "曾孙女" => Some(DescendantBranch::DirectPatrilineal(3)),
        "外曾孙" | "外曾孙女" => Some(DescendantBranch::DirectMaternal(3)),
        "玄孙" | "玄孙女" => Some(DescendantBranch::DirectPatrilineal(4)),
        "外玄孙" | "外玄孙女" => Some(DescendantBranch::DirectMaternal(4)),
        "来孙" | "来孙女" => Some(DescendantBranch::DirectPatrilineal(5)),
        "外来孙" | "外来孙女" => Some(DescendantBranch::DirectMaternal(5)),

        "侄子" | "侄女" => Some(DescendantBranch::Nephew(1)),
        "外甥" | "外甥女" => Some(DescendantBranch::MaternalNephew(1)),
        "侄孙" | "侄孙女" => Some(DescendantBranch::Nephew(2)),
        "外甥孙" | "外甥孙女" => Some(DescendantBranch::MaternalNephew(2)),
        "侄曾孙" | "侄曾孙女" => Some(DescendantBranch::Nephew(3)),
        "外甥曾孙" | "外甥曾孙女" => Some(DescendantBranch::MaternalNephew(3)),
        "侄玄孙" | "侄玄孙女" => Some(DescendantBranch::Nephew(4)),
        "外甥玄孙" | "外甥玄孙女" => Some(DescendantBranch::MaternalNephew(4)),
        "侄来孙" | "侄来孙女" => Some(DescendantBranch::Nephew(5)),
        "外甥来孙" | "外甥来孙女" => Some(DescendantBranch::MaternalNephew(5)),

        "堂兄弟" | "堂姐妹" => Some(DescendantBranch::PaternalCousin(0)),
        "表兄弟" | "表姐妹" => Some(DescendantBranch::MaternalCousin(0)),
        "堂侄" | "堂侄女" => Some(DescendantBranch::PaternalCousin(1)),
        "表侄" | "表侄女" => Some(DescendantBranch::MaternalCousin(1)),
        "堂侄孙" | "堂侄孙女" => Some(DescendantBranch::PaternalCousin(2)),
        "表侄孙" | "表侄孙女" => Some(DescendantBranch::MaternalCousin(2)),
        "堂侄曾孙" | "堂侄曾孙女" => Some(DescendantBranch::PaternalCousin(3)),
        "表侄曾孙" | "表侄曾孙女" => Some(DescendantBranch::MaternalCousin(3)),
        "堂侄玄孙" | "堂侄玄孙女" => Some(DescendantBranch::PaternalCousin(4)),
        "表侄玄孙" | "表侄玄孙女" => Some(DescendantBranch::MaternalCousin(4)),
        "堂侄来孙" | "堂侄来孙女" => Some(DescendantBranch::PaternalCousin(5)),
        "表侄来孙" | "表侄来孙女" => Some(DescendantBranch::MaternalCousin(5)),
        _ => None,
    }
}

fn paternal_ancestor_title(depth: u8, father: bool) -> Option<&'static str> {
    match (depth, father) {
        (2, true) => Some("爷爷"),
        (2, false) => Some("奶奶"),
        (3, true) => Some("太爷爷"),
        (3, false) => Some("太奶奶"),
        (4, true) => Some("高祖父"),
        (4, false) => Some("高祖母"),
        (5, true) => Some("曾高祖父"),
        (5, false) => Some("曾高祖母"),
        _ => None,
    }
}

fn maternal_ancestor_title(depth: u8, father: bool) -> Option<&'static str> {
    match (depth, father) {
        (2, true) => Some("外公"),
        (2, false) => Some("外婆"),
        (3, true) => Some("外曾外祖父"),
        (3, false) => Some("外曾外祖母"),
        (4, true) => Some("外高祖父"),
        (4, false) => Some("外高祖母"),
        (5, true) => Some("外曾高祖父"),
        (5, false) => Some("外曾高祖母"),
        _ => None,
    }
}

fn husband_paternal_ancestor_title(depth: u8, father: bool) -> Option<&'static str> {
    match (depth, father) {
        (2, true) => Some("夫祖父"),
        (2, false) => Some("夫祖母"),
        (3, true) => Some("夫曾祖父"),
        (3, false) => Some("夫曾祖母"),
        (4, true) => Some("夫高祖父"),
        (4, false) => Some("夫高祖母"),
        (5, true) => Some("夫曾高祖父"),
        (5, false) => Some("夫曾高祖母"),
        _ => None,
    }
}

fn husband_maternal_ancestor_title(depth: u8, father: bool) -> Option<&'static str> {
    match (depth, father) {
        (2, true) => Some("夫外祖父"),
        (2, false) => Some("夫外祖母"),
        (3, true) => Some("夫外曾祖父"),
        (3, false) => Some("夫外曾祖母"),
        (4, true) => Some("夫外高祖父"),
        (4, false) => Some("夫外高祖母"),
        (5, true) => Some("夫外曾高祖父"),
        (5, false) => Some("夫外曾高祖母"),
        _ => None,
    }
}

fn wife_paternal_ancestor_title(depth: u8, father: bool) -> Option<&'static str> {
    match (depth, father) {
        (2, true) => Some("妻祖父"),
        (2, false) => Some("妻祖母"),
        (3, true) => Some("妻曾祖父"),
        (3, false) => Some("妻曾祖母"),
        (4, true) => Some("妻高祖父"),
        (4, false) => Some("妻高祖母"),
        (5, true) => Some("妻曾高祖父"),
        (5, false) => Some("妻曾高祖母"),
        _ => None,
    }
}

fn wife_maternal_ancestor_title(depth: u8, father: bool) -> Option<&'static str> {
    match (depth, father) {
        (2, true) => Some("妻外祖父"),
        (2, false) => Some("妻外祖母"),
        (3, true) => Some("妻外曾祖父"),
        (3, false) => Some("妻外曾祖母"),
        (4, true) => Some("妻外高祖父"),
        (4, false) => Some("妻外高祖母"),
        (5, true) => Some("妻外曾高祖父"),
        (5, false) => Some("妻外曾高祖母"),
        _ => None,
    }
}

fn direct_patrilineal_spouse_title(depth: u8, husband: bool) -> Option<&'static str> {
    match (depth, husband) {
        (1, false) => Some("儿媳"),
        (2, false) => Some("孙媳妇"),
        (3, false) => Some("曾孙媳妇"),
        (4, false) => Some("玄孙媳妇"),
        (5, false) => Some("来孙媳妇"),
        _ => None,
    }
}

fn direct_maternal_spouse_title(depth: u8, husband: bool) -> Option<&'static str> {
    match (depth, husband) {
        (1, true) => Some("女婿"),
        (2, true) => Some("外孙女婿"),
        (3, true) => Some("外曾孙女婿"),
        (4, true) => Some("外玄孙女婿"),
        (5, true) => Some("外来孙女婿"),
        _ => None,
    }
}

fn nephew_spouse_title(depth: u8, husband: bool) -> Option<&'static str> {
    match (depth, husband) {
        (1, false) => Some("侄媳妇"),
        (1, true) => Some("侄女婿"),
        (2, false) => Some("侄孙媳妇"),
        (2, true) => Some("侄孙女婿"),
        (3, false) => Some("侄曾孙媳妇"),
        (3, true) => Some("侄曾孙女婿"),
        (4, false) => Some("侄玄孙媳妇"),
        (4, true) => Some("侄玄孙女婿"),
        (5, false) => Some("侄来孙媳妇"),
        (5, true) => Some("侄来孙女婿"),
        _ => None,
    }
}

fn maternal_nephew_spouse_title(depth: u8, husband: bool) -> Option<&'static str> {
    match (depth, husband) {
        (1, false) => Some("外甥媳妇"),
        (1, true) => Some("外甥女婿"),
        (2, false) => Some("外甥孙媳妇"),
        (2, true) => Some("外甥孙女婿"),
        (3, false) => Some("外甥曾孙媳妇"),
        (3, true) => Some("外甥曾孙女婿"),
        (4, false) => Some("外甥玄孙媳妇"),
        (4, true) => Some("外甥玄孙女婿"),
        (5, false) => Some("外甥来孙媳妇"),
        (5, true) => Some("外甥来孙女婿"),
        _ => None,
    }
}

fn paternal_cousin_spouse_title(depth: u8, husband: bool) -> Option<&'static str> {
    match (depth, husband) {
        (0, false) => Some("堂兄弟媳"),
        (0, true) => Some("堂姐妹夫"),
        (1, false) => Some("堂侄媳妇"),
        (1, true) => Some("堂侄女婿"),
        (2, false) => Some("堂侄孙媳妇"),
        (2, true) => Some("堂侄孙女婿"),
        (3, false) => Some("堂侄曾孙媳妇"),
        (3, true) => Some("堂侄曾孙女婿"),
        (4, false) => Some("堂侄玄孙媳妇"),
        (4, true) => Some("堂侄玄孙女婿"),
        (5, false) => Some("堂侄来孙媳妇"),
        (5, true) => Some("堂侄来孙女婿"),
        _ => None,
    }
}

fn maternal_cousin_spouse_title(depth: u8, husband: bool) -> Option<&'static str> {
    match (depth, husband) {
        (0, false) => Some("表兄弟媳"),
        (0, true) => Some("表姐妹夫"),
        (1, false) => Some("表侄媳妇"),
        (1, true) => Some("表侄女婿"),
        (2, false) => Some("表侄孙媳妇"),
        (2, true) => Some("表侄孙女婿"),
        (3, false) => Some("表侄曾孙媳妇"),
        (3, true) => Some("表侄曾孙女婿"),
        (4, false) => Some("表侄玄孙媳妇"),
        (4, true) => Some("表侄玄孙女婿"),
        (5, false) => Some("表侄来孙媳妇"),
        (5, true) => Some("表侄来孙女婿"),
        _ => None,
    }
}

fn direct_patrilineal_title(depth: u8, male: bool) -> Option<&'static str> {
    match (depth, male) {
        (2, true) => Some("孙子"),
        (2, false) => Some("孙女"),
        (3, true) => Some("曾孙"),
        (3, false) => Some("曾孙女"),
        (4, true) => Some("玄孙"),
        (4, false) => Some("玄孙女"),
        (5, true) => Some("来孙"),
        (5, false) => Some("来孙女"),
        _ => None,
    }
}

fn direct_maternal_title(depth: u8, male: bool) -> Option<&'static str> {
    match (depth, male) {
        (2, true) => Some("外孙"),
        (2, false) => Some("外孙女"),
        (3, true) => Some("外曾孙"),
        (3, false) => Some("外曾孙女"),
        (4, true) => Some("外玄孙"),
        (4, false) => Some("外玄孙女"),
        (5, true) => Some("外来孙"),
        (5, false) => Some("外来孙女"),
        _ => None,
    }
}

fn nephew_title(depth: u8, male: bool) -> Option<&'static str> {
    match (depth, male) {
        (1, true) => Some("侄子"),
        (1, false) => Some("侄女"),
        (2, true) => Some("侄孙"),
        (2, false) => Some("侄孙女"),
        (3, true) => Some("侄曾孙"),
        (3, false) => Some("侄曾孙女"),
        (4, true) => Some("侄玄孙"),
        (4, false) => Some("侄玄孙女"),
        (5, true) => Some("侄来孙"),
        (5, false) => Some("侄来孙女"),
        _ => None,
    }
}

fn maternal_nephew_title(depth: u8, male: bool) -> Option<&'static str> {
    match (depth, male) {
        (1, true) => Some("外甥"),
        (1, false) => Some("外甥女"),
        (2, true) => Some("外甥孙"),
        (2, false) => Some("外甥孙女"),
        (3, true) => Some("外甥曾孙"),
        (3, false) => Some("外甥曾孙女"),
        (4, true) => Some("外甥玄孙"),
        (4, false) => Some("外甥玄孙女"),
        (5, true) => Some("外甥来孙"),
        (5, false) => Some("外甥来孙女"),
        _ => None,
    }
}

fn paternal_cousin_title(depth: u8, male: bool) -> Option<&'static str> {
    match (depth, male) {
        (0, true) => Some("堂兄弟"),
        (0, false) => Some("堂姐妹"),
        (1, true) => Some("堂侄"),
        (1, false) => Some("堂侄女"),
        (2, true) => Some("堂侄孙"),
        (2, false) => Some("堂侄孙女"),
        (3, true) => Some("堂侄曾孙"),
        (3, false) => Some("堂侄曾孙女"),
        (4, true) => Some("堂侄玄孙"),
        (4, false) => Some("堂侄玄孙女"),
        (5, true) => Some("堂侄来孙"),
        (5, false) => Some("堂侄来孙女"),
        _ => None,
    }
}

fn maternal_cousin_title(depth: u8, male: bool) -> Option<&'static str> {
    match (depth, male) {
        (0, true) => Some("表兄弟"),
        (0, false) => Some("表姐妹"),
        (1, true) => Some("表侄"),
        (1, false) => Some("表侄女"),
        (2, true) => Some("表侄孙"),
        (2, false) => Some("表侄孙女"),
        (3, true) => Some("表侄曾孙"),
        (3, false) => Some("表侄曾孙女"),
        (4, true) => Some("表侄玄孙"),
        (4, false) => Some("表侄玄孙女"),
        (5, true) => Some("表侄来孙"),
        (5, false) => Some("表侄来孙女"),
        _ => None,
    }
}
