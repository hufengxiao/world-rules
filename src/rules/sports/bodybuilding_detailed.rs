//! 健美详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: BodybuildingDetailedRules,
    name: "健美详细规则",
    desc: "健美比赛详细规则",
    origin: "IFBB",
    tags: ["体育", "健身"],
    category: RuleCategory::sports("bodybuilding_detailed"),
    sections: [("评分", section_0), ("造型", section_1)]
}

impl BodybuildingDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["肌肉量", "对称性"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["规定造型", "自由造型"]
    }
}
