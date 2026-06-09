//! 海洋学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: OceanographyRules,
    name: "海洋学定律",
    desc: "海洋学定律",
    origin: "国际",
    tags: ["科学", "地球"],
    category: RuleCategory::science("oceanography"),
    sections: [("环流", section_0), ("化学", section_1)]
}

impl OceanographyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["洋流", "潮汐", "波浪"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["盐度", "溶解氧", "碳循环"]
    }
}
