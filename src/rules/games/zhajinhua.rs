//! 炸金花规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: ZhajinhuaRules,
    name: "炸金花规则",
    desc: "炸金花扑克游戏规则",
    origin: "中国",
    tags: ["游戏", "扑克"],
    category: RuleCategory::games("zhajinhua"),
    sections: [("牌型大小", section_0), ("游戏流程", section_1), ("特殊规则", section_2)]
}

impl ZhajinhuaRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "豹子最大",
            "同花顺次之",
            "同花第三",
            "顺子第四",
            "对子第五",
            "散牌最小",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["每人发3张牌", "下注/跟注/比牌/弃牌", "最后剩两人比牌"]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec!["235吃豹子", "比牌需等额下注", "封顶规则防无限加注"]
    }
}
