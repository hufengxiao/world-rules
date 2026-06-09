//! 保龄球详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: BowlingDetailedRules,
    name: "保龄球详细规则",
    desc: "保龄球详细比赛规则",
    origin: "WBSC",
    tags: ["体育", "休闲"],
    category: RuleCategory::sports("bowling_detailed"),
    sections: [("计分", section_0), ("比赛", section_1)]
}

impl BowlingDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["全倒", "补中"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["个人赛", "团体赛"]
    }
}
