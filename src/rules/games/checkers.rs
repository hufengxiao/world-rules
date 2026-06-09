//! 国际跳棋规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: CheckersRules,
    name: "国际跳棋规则",
    desc: "国际跳棋规则",
    origin: "国际",
    tags: ["游戏", "棋类"],
    category: RuleCategory::games("checkers"),
    sections: [("棋盘与棋子", section_0), ("走法", section_1), ("胜负", section_2)]
}

impl CheckersRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["10x10棋盘", "20枚棋子", "深色格子走棋"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["普通棋子斜走一格", "跳吃对方棋子", "到达底线升王"]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec!["吃光对方棋子获胜", "对方无法行动获胜"]
    }
}
