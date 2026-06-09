//! 田径详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: AthleticsDetailedRules,
    name: "田径详细规则",
    desc: "田径详细比赛规则",
    origin: "IAAF",
    tags: ["体育", "田径"],
    category: RuleCategory::sports("athletics_detailed"),
    sections: [("径赛", section_0), ("田赛", section_1)]
}

impl AthleticsDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["起跑规则", "抢跑判罚"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["试跳试投", "成绩测量"]
    }
}
