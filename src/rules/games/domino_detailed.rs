//! 多米诺详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: DominoDetailedRules,
    name: "多米诺详细规则",
    desc: "多米诺骨牌详细规则",
    origin: "国际",
    tags: ["游戏", "骨牌"],
    category: RuleCategory::games("domino_detailed"),
    sections: [("接龙", section_0), ("计分", section_1)]
}

impl DominoDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["同点数相接", "双牌横向"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["剩余点数", "先出完者胜"]
    }
}
