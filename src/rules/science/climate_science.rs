//! 气候科学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: ClimateScienceRules,
    name: "气候科学定律",
    desc: "气候科学定律",
    origin: "国际",
    tags: ["科学", "环境"],
    category: RuleCategory::science("climate_science"),
    sections: [("机制", section_0), ("变化", section_1)]
}

impl ClimateScienceRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["温室效应", "碳循环"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["全球变暖", "极端天气"]
    }
}
