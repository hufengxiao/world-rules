//! 纳米技术定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: NanotechnologyRules,
    name: "纳米技术定律",
    desc: "纳米技术定律",
    origin: "国际",
    tags: ["科学", "材料"],
    category: RuleCategory::science("nanotechnology"),
    sections: [("制备", section_0), ("性质", section_1)]
}

impl NanotechnologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["自组装", "化学气相沉积"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["量子尺寸效应", "表面效应"]
    }
}
