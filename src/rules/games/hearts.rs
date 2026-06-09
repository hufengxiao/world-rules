//! 红心大战规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: HeartsRules,
    name: "红心大战规则",
    desc: "红心大战卡牌游戏规则",
    origin: "美国",
    tags: ["游戏", "卡牌"],
    category: RuleCategory::games("hearts"),
    sections: [("游戏目标", section_0), ("换牌规则", section_1)]
}

impl HeartsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "避免吃到红心每张1分",
            "避免吃黑桃Q值13分",
            "收齐全部红心可全转嫁",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["开局换3张牌", "按左/右/对面轮换"]
    }
}
