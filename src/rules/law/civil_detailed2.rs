//! 民法详解2
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: CivilDetailed2Rules,
    name: "民法详解2",
    desc: "民法详解2",
    origin: "中国",
    tags: ["法律", "民法"],
    category: RuleCategory::law("civil_detailed2"),
    sections: [("物权", section_0), ("债权", section_1)]
}

impl CivilDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["所有权", "用益物权", "担保物权"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["合同之债", "侵权之债"]
    }
}
