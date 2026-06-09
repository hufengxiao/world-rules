//! 统计力学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: StatisticalMechanicsRules,
    name: "统计力学定律",
    desc: "统计力学定律",
    origin: "国际",
    tags: ["科学", "物理"],
    category: RuleCategory::science("statistical_mechanics"),
    sections: [("基础", section_0), ("应用", section_1)]
}

impl StatisticalMechanicsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["玻尔兹曼分布", "配分函数"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["相变理论", "临界现象"]
    }
}
