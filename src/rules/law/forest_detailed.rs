//! 森林法详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: ForestDetailedRules,
    name: "森林法详解",
    desc: "森林法详解",
    origin: "中国",
    tags: ["法律", "资源"],
    category: RuleCategory::law("forest_detailed"),
    sections: [("权属", section_0), ("保护", section_1)]
}

impl ForestDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["国有林", "集体林"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["采伐限额", "天然林保护"]
    }
}
