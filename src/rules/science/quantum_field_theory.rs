//! 量子场论定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: QuantumFieldTheoryRules,
    name: "量子场论定律",
    desc: "量子场论定律",
    origin: "国际",
    tags: ["科学", "物理"],
    category: RuleCategory::science("quantum_field_theory"),
    sections: [("基础", section_0), ("应用", section_1)]
}

impl QuantumFieldTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["场量子化", "费曼图", "重整化"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["QED", "QCD"]
    }
}
