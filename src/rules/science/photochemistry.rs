//! 光化学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: PhotochemistryRules,
    name: "光化学定律",
    desc: "光化学定律",
    origin: "国际",
    tags: ["科学", "化学"],
    category: RuleCategory::science("photochemistry"),
    sections: [("基本定律", section_0), ("应用", section_1)]
}

impl PhotochemistryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["光化学第一定律", "光化学第二定律", "量子产率"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["光合作用", "光刻技术", "光催化降解"]
    }
}
