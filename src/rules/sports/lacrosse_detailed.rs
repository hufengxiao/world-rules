//! 长曲棍球规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: LacrosseDetailedRules,
    name: "长曲棍球规则",
    desc: "长曲棍球详细规则",
    origin: "FIL",
    tags: ["体育", "球类"],
    category: RuleCategory::sports("lacrosse_detailed"),
    sections: [("比赛", section_0), ("技术", section_1)]
}

impl LacrosseDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["4节", "越位"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["传球", "射门"]
    }
}
