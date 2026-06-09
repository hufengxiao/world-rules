//! 24点规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: TwentyFourPointRules,
    name: "24点规则",
    desc: "24点数学卡牌游戏规则",
    origin: "中国",
    tags: ["游戏", "益智"],
    category: RuleCategory::games("twenty_four_point"),
    sections: [("基本规则", section_0), ("计分规则", section_1)]
}

impl TwentyFourPointRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "抽取4张牌",
            "用加减乘除凑24",
            "每张牌用且仅用一次",
            "先算出者得分",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["正确解答得1分", "无解可跳过", "累计得分制"]
    }
}
