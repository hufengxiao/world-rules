//! 四子棋规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: ConnectFourRules,
    name: "四子棋规则",
    desc: "四子棋规则",
    origin: "美国",
    tags: ["游戏", "棋类"],
    category: RuleCategory::games("connect_four"),
    sections: [("棋盘", section_0), ("胜负", section_1)]
}

impl ConnectFourRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["7列x6行竖立棋盘", "从顶部放入棋子"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["先连成4子者胜", "横竖斜均可"]
    }
}
