//! 概率论定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: ProbabilityTheoryRules,
    name: "概率论定律",
    desc: "概率论基础定律",
    origin: "国际",
    tags: ["科学", "数学"],
    category: RuleCategory::science("probability_theory"),
    sections: [("基础", section_0), ("分布", section_1)]
}

impl ProbabilityTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["大数定律", "中心极限定理", "贝叶斯定理"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["正态分布", "泊松分布"]
    }
}
