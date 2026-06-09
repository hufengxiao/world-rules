//! 射箭详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: ArcheryDetailedRules,
    name: "射箭详细规则",
    desc: "射箭详细比赛规则",
    origin: "WA",
    tags: ["体育", "精准"],
    category: RuleCategory::sports("archery_detailed"),
    sections: [("室外", section_0), ("室内", section_1)]
}

impl ArcheryDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["70米靶", "10环制"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["18米靶", "淘汰赛"]
    }
}
