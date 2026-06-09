//! 国际人道法
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: HumanitarianLawRules,
    name: "国际人道法",
    desc: "国际人道法律规则",
    origin: "国际",
    tags: ["法律", "人道"],
    category: RuleCategory::law("humanitarian_law"),
    sections: [("战争法", section_0), ("保护", section_1)]
}

impl HumanitarianLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["区分原则", "比例原则"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["战俘待遇", "平民保护"]
    }
}
