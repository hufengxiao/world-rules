//! 手球详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: HandballDetailedRulesRules,
    name: "手球详细规则",
    desc: "手球详细比赛规则",
    origin: "IHF",
    tags: ["体育", "球类"],
    category: RuleCategory::sports("handball_detailed_rules"),
    sections: [("比赛", section_0), ("规则", section_1)]
}

impl HandballDetailedRulesRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["2x30分钟"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["3步规则", "3秒持球"]
    }
}
