//! 火山学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: VolcanologyRules,
    name: "火山学定律",
    desc: "火山学定律",
    origin: "国际",
    tags: ["科学", "地球"],
    category: RuleCategory::science("volcanology"),
    sections: [("类型", section_0), ("喷发", section_1)]
}

impl VolcanologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["盾状火山", "层状火山", "复式火山"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["喷发指数", "熔岩流", "火山灰"]
    }
}
