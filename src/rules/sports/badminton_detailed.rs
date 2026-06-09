//! 羽毛球详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: BadmintonDetailedRules,
    name: "羽毛球详细规则",
    desc: "羽毛球详细比赛规则",
    origin: "BWF",
    tags: ["体育", "球类"],
    category: RuleCategory::sports("badminton_detailed"),
    sections: [("计分", section_0), ("发球", section_1)]
}

impl BadmintonDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["21分制", "三局两胜"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["发球区"]
    }
}
