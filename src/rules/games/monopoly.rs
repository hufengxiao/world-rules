//! 大富翁规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: MonopolyRules,
    name: "大富翁规则",
    desc: "大富翁棋盘游戏规则",
    origin: "美国",
    tags: ["游戏", "桌游"],
    category: RuleCategory::games("monopoly"),
    sections: [("游戏流程", section_0), ("特殊格", section_1)]
}

impl MonopolyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["掷骰子移动", "买地建房收租", "破产淘汰"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["机会卡/命运卡", "监狱/免费停车", "起点领工资"]
    }
}
