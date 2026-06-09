//! 击剑详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: FencingDetailedRules,
    name: "击剑详细规则",
    desc: "击剑详细比赛规则",
    origin: "FIE",
    tags: ["体育", "格斗"],
    category: RuleCategory::sports("fencing_detailed"),
    sections: [("剑种", section_0), ("得分", section_1)]
}

impl FencingDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["花剑", "重剑", "佩剑"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["优先裁判权", "电子裁判"]
    }
}
