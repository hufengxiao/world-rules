//! 矿产法详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: MineralDetailedRules,
    name: "矿产法详解",
    desc: "矿产资源法详解",
    origin: "中国",
    tags: ["法律", "资源"],
    category: RuleCategory::law("mineral_detailed"),
    sections: [("勘查", section_0), ("开采", section_1)]
}

impl MineralDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["探矿权", "勘查许可"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["采矿权", "矿山安全"]
    }
}
