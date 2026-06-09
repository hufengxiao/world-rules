//! 美学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: AestheticsRules,
    name: "美学定律",
    desc: "美学定律",
    origin: "国际",
    tags: ["科学", "艺术"],
    category: RuleCategory::science("aesthetics"),
    sections: [("理论", section_0), ("应用", section_1)]
}

impl AestheticsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["审美经验", "形式美法则"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["设计美学", "建筑美学"]
    }
}
