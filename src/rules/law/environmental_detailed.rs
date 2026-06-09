//! 环保法详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: EnvironmentalDetailedRules,
    name: "环保法详解",
    desc: "环保法详解",
    origin: "中国",
    tags: ["法律", "环境"],
    category: RuleCategory::law("environmental_detailed"),
    sections: [("制度", section_0), ("责任", section_1)]
}

impl EnvironmentalDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["环评制度", "排污许可"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["公益诉讼", "按日计罚"]
    }
}
