//! 量子计算理论
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: QuantumComputingRules,
    name: "量子计算理论",
    desc: "量子计算理论定律",
    origin: "国际",
    tags: ["科学", "计算机"],
    category: RuleCategory::science("quantum_computing"),
    sections: [("量子比特", section_0), ("算法", section_1)]
}

impl QuantumComputingRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["叠加态", "纠缠态", "量子门"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["Shor算法", "Grover搜索", "量子纠错"]
    }
}
