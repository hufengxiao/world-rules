//! 篮球详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: BasketballDetailedRules,
    name: "篮球详细规则",
    desc: "篮球详细比赛规则",
    origin: "NBA",
    tags: ["体育", "球类"],
    category: RuleCategory::sports("basketball_detailed"),
    sections: [("比赛", section_0), ("犯规", section_1)]
}

impl BasketballDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["24秒规则", "8秒过半场"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["个人犯规", "技术犯规"]
    }
}
