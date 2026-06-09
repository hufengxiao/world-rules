//! 谁是卧底规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: WhoIsSpyRules,
    name: "谁是卧底规则",
    desc: "谁是卧底派对游戏规则",
    origin: "中国",
    tags: ["游戏", "派对"],
    category: RuleCategory::games("who_is_spy"),
    sections: [("游戏流程", section_0), ("胜负", section_1)]
}

impl WhoIsSpyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["每人获一个词", "卧底词与众人不同", "轮流描述"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["卧底被投出平民胜", "卧底存活到最后卧底胜"]
    }
}
