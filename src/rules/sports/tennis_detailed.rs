//! 网球详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: TennisDetailedRules,
    name: "网球详细规则",
    desc: "网球详细比赛规则",
    origin: "ITF",
    tags: ["体育", "球类"],
    category: RuleCategory::sports("tennis_detailed"),
    sections: [("计分", section_0), ("发球", section_1)]
}

impl TennisDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["15-30-40", "抢七"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["一发二发", "ACE球"]
    }
}
