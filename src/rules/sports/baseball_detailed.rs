//! 棒球详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: BaseballDetailedRules,
    name: "棒球详细规则",
    desc: "棒球详细比赛规则",
    origin: "MLB",
    tags: ["体育", "球类"],
    category: RuleCategory::sports("baseball_detailed"),
    sections: [("比赛", section_0), ("进攻", section_1)]
}

impl BaseballDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["9局制", "延长赛"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["安打", "全垒打", "盗垒"]
    }
}
