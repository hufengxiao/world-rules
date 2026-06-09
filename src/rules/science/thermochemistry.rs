//! 热化学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: ThermochemistryRules,
    name: "热化学定律",
    desc: "热化学定律",
    origin: "国际",
    tags: ["科学", "化学"],
    category: RuleCategory::science("thermochemistry"),
    sections: [("定律", section_0), ("应用", section_1)]
}

impl ThermochemistryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["赫斯定律", "基尔霍夫定律"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["燃烧热", "生成热", "键能"]
    }
}
