//! 行为经济学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: BehavioralEconomicsRules,
    name: "行为经济学定律",
    desc: "行为经济学定律",
    origin: "国际",
    tags: ["科学", "经济"],
    category: RuleCategory::science("behavioral_economics"),
    sections: [("偏差", section_0), ("理论", section_1)]
}

impl BehavioralEconomicsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["锚定效应", "损失厌恶"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["前景理论", "助推理论"]
    }
}
