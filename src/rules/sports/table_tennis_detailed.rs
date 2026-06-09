//! 乒乓球详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: TableTennisDetailedRules,
    name: "乒乓球详细规则",
    desc: "乒乓球详细比赛规则",
    origin: "ITTF",
    tags: ["体育", "球类"],
    category: RuleCategory::sports("table_tennis_detailed"),
    sections: [("计分", section_0), ("技术", section_1)]
}

impl TableTennisDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["11分制", "轮换发球"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["合法发球", "擦网重发"]
    }
}
