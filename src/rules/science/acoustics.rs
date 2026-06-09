//! 声学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: AcousticsRules,
    name: "声学定律",
    desc: "声学物理定律",
    origin: "国际",
    tags: ["科学", "物理"],
    category: RuleCategory::science("acoustics"),
    sections: [("波动", section_0), ("共振", section_1)]
}

impl AcousticsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["声波传播方程", "多普勒效应", "驻波"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["共振频率", "共振条件", "阻尼振动"]
    }
}
