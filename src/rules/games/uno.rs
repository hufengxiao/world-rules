//! UNO规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: UnoRules,
    name: "UNO规则",
    desc: "UNO卡牌游戏规则",
    origin: "美国",
    tags: ["游戏", "卡牌"],
    category: RuleCategory::games("uno"),
    sections: [("特殊牌", section_0), ("出牌规则", section_1)]
}

impl UnoRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "跳过牌跳过下家",
            "反转牌改变方向",
            "+2牌下家摸2张",
            "万能牌变色",
            "+4万能牌变色+摸4",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["同色出牌", "同数出牌", "特殊牌叠加", "喊UNO规则", "罚摸2张"]
    }
}
