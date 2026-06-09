//! 排球详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: VolleyballDetailedRules,
    name: "排球详细规则",
    desc: "排球详细比赛规则",
    origin: "FIVB",
    tags: ["体育", "球类"],
    category: RuleCategory::sports("volleyball_detailed"),
    sections: [("轮转", section_0), ("犯规", section_1)]
}

impl VolleyballDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["6人轮转", "自由人"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["触网", "过中线"]
    }
}
