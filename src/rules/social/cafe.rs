//! 咖啡礼仪
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: CafeRules,
    name: "咖啡礼仪",
    desc: "咖啡社交礼仪",
    origin: "国际",
    tags: ["社交", "咖啡"],
    category: RuleCategory::social("cafe"),
    sections: [("点单", section_0), ("饮用", section_1)]
}

impl CafeRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["了解咖啡种类", "注意排队礼仪"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["不发出声响", "使用杯把", "搅拌后取出勺子"]
    }
}
