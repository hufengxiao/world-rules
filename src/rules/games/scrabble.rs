//! 拼字游戏规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: ScrabbleRules,
    name: "拼字游戏规则",
    desc: "Scrabble拼字游戏规则",
    origin: "美国",
    tags: ["游戏", "桌游"],
    category: RuleCategory::games("scrabble"),
    sections: [("游戏流程", section_0), ("特殊格", section_1)]
}

impl ScrabbleRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["从字母袋抽7个字母", "在棋盘上拼单词", "按字母分值计分"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["双倍字母/三倍字母", "双倍单词/三倍单词"]
    }
}
