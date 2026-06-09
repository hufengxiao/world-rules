//! 神经科学详细定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: NeuroscienceDetailedRules,
    name: "神经科学详细定律",
    desc: "神经科学详细定律",
    origin: "国际",
    tags: ["科学", "生物"],
    category: RuleCategory::science("neuroscience_detailed"),
    sections: [("神经元", section_0), ("脑区", section_1)]
}

impl NeuroscienceDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["动作电位", "突触传递", "神经可塑性"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["大脑皮层", "海马体"]
    }
}
