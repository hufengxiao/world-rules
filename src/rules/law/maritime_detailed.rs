//! 海商法详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: MaritimeDetailedRules,
    name: "海商法详解",
    desc: "海商法详解",
    origin: "中国",
    tags: ["法律", "商法"],
    category: RuleCategory::law("maritime_detailed"),
    sections: [("船舶", section_0), ("运输", section_1)]
}

impl MaritimeDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["船舶登记", "优先权"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["提单", "货物运输"]
    }
}
