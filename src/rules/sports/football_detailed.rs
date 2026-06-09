//! 足球详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: FootballDetailedRules,
    name: "足球详细规则",
    desc: "足球详细比赛规则",
    origin: "FIFA",
    tags: ["体育", "球类"],
    category: RuleCategory::sports("football_detailed"),
    sections: [("比赛", section_0), ("技术", section_1)]
}

impl FootballDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["越位详解", "犯规处罚", "VAR助理"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["任意球", "角球", "点球"]
    }
}
