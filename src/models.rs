use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationType {
    Father,
    Mother,
    OlderBrother,
    YoungerBrother,
    OlderSister,
    YoungerSister,
    Son,
    Daughter,
    Husband,
    Wife,
}

impl RelationType {
    pub const ALL: [Self; 10] = [
        Self::Father,
        Self::Mother,
        Self::OlderBrother,
        Self::YoungerBrother,
        Self::OlderSister,
        Self::YoungerSister,
        Self::Son,
        Self::Daughter,
        Self::Husband,
        Self::Wife,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Father => "爸爸",
            Self::Mother => "妈妈",
            Self::OlderBrother => "哥哥",
            Self::YoungerBrother => "弟弟",
            Self::OlderSister => "姐姐",
            Self::YoungerSister => "妹妹",
            Self::Son => "儿子",
            Self::Daughter => "女儿",
            Self::Husband => "丈夫",
            Self::Wife => "妻子",
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            Self::Father => "F",
            Self::Mother => "M",
            Self::OlderBrother => "OB",
            Self::YoungerBrother => "YB",
            Self::OlderSister => "OS",
            Self::YoungerSister => "YS",
            Self::Son => "S",
            Self::Daughter => "D",
            Self::Husband => "H",
            Self::Wife => "W",
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            Self::Father => "直系上一代男性",
            Self::Mother => "直系上一代女性",
            Self::OlderBrother => "同辈年长男性",
            Self::YoungerBrother => "同辈年幼男性",
            Self::OlderSister => "同辈年长女性",
            Self::YoungerSister => "同辈年幼女性",
            Self::Son => "直系下一代男性",
            Self::Daughter => "直系下一代女性",
            Self::Husband => "配偶中的男性",
            Self::Wife => "配偶中的女性",
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RelationResult {
    pub standard_title: String,
    pub aliases: Vec<String>,
    pub common_aliases: Vec<String>,
    pub dialect_aliases: Vec<String>,
    pub neutral_title: Option<String>,
    pub relation_path_text: String,
    pub code_path_text: String,
    pub is_matched: bool,
    pub message: String,
}

impl RelationResult {
    pub fn prompt() -> Self {
        Self {
            standard_title: "等待计算".to_owned(),
            aliases: Vec::new(),
            common_aliases: Vec::new(),
            dialect_aliases: Vec::new(),
            neutral_title: None,
            relation_path_text: "我".to_owned(),
            code_path_text: "-".to_owned(),
            is_matched: false,
            message: "请选择关系步骤后开始计算。".to_owned(),
        }
    }

    pub fn to_clipboard_text(&self) -> String {
        let aliases = if self.aliases.is_empty() {
            "无".to_owned()
        } else {
            self.aliases.join("、")
        };
        let common_aliases = if self.common_aliases.is_empty() {
            "无".to_owned()
        } else {
            self.common_aliases.join("、")
        };
        let dialect_aliases = if self.dialect_aliases.is_empty() {
            "无".to_owned()
        } else {
            self.dialect_aliases.join("、")
        };
        let neutral_title = self.neutral_title.as_deref().unwrap_or("无");

        format!(
            "标准称呼：{}\n常见叫法：{}\n方言叫法：{}\n中性称呼：{}\n全部别名：{}\n关系路径：{}\n关系编码：{}\n说明：{}",
            self.standard_title,
            common_aliases,
            dialect_aliases,
            neutral_title,
            aliases,
            self.relation_path_text,
            self.code_path_text,
            self.message
        )
    }
}
