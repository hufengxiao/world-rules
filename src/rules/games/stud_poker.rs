//! 梭哈规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: StudPokerRules,
    name: "梭哈规则",
    desc: "梭哈扑克游戏规则",
    origin: "美国",
    tags: ["游戏", "扑克"],
    category: RuleCategory::games("stud_poker"),
    sections: [("游戏流程", section_0), ("牌型大小", section_1)]
}

impl StudPokerRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["发1张底牌", "逐张发面牌并下注", "5张后比牌"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["同花顺>四条>葫芦>同花>顺子>三条>两对>一对>高牌"]
    }
}
