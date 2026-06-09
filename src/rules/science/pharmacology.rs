//! 药理学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: PharmacologyRules,
    name: "药理学定律",
    desc: "药理学定律",
    origin: "国际",
    tags: ["科学", "医学"],
    category: RuleCategory::science("pharmacology"),
    sections: [("基础", section_0), ("分类", section_1)]
}

impl PharmacologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["药代动力学", "药效动力学"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["受体理论", "酶抑制"]
    }
}
