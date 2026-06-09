//! 测度论定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: MeasureTheoryRules,
    name: "测度论定律",
    desc: "测度论定律",
    origin: "国际",
    tags: ["科学", "数学"],
    category: RuleCategory::science("measure_theory"),
    sections: [("测度", section_0), ("积分", section_1)]
}

impl MeasureTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["勒贝格测度", "测度空间"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["勒贝格积分", "控制收敛"]
    }
}
