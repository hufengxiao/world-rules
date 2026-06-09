//! 土地管理详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: LandDetailedRules,
    name: "土地管理详解",
    desc: "土地管理法详解",
    origin: "中国",
    tags: ["法律", "土地"],
    category: RuleCategory::law("land_detailed"),
    sections: [("权属", section_0), ("利用", section_1)]
}

impl LandDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["国有土地", "集体土地"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["用途管制", "耕地保护"]
    }
}
