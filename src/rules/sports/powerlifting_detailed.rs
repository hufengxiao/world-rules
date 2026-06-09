//! 力量举详细
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: PowerliftingDetailedRules,
    name: "力量举详细",
    desc: "力量举详细规则",
    origin: "IPF",
    tags: ["体育", "力量"],
    category: RuleCategory::sports("powerlifting_detailed"),
    sections: [("项目", section_0), ("规则", section_1)]
}

impl PowerliftingDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["深蹲", "卧推", "硬拉"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["3次试举", "重量递增"]
    }
}
