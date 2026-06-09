//! 快艇骰子规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: YahtzeeRules,
    name: "快艇骰子规则",
    desc: "快艇骰子游戏规则",
    origin: "美国",
    tags: ["游戏", "骰子"],
    category: RuleCategory::games("yahtzee"),
    sections: [("游戏流程", section_0), ("得分类别", section_1)]
}

impl YahtzeeRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["每轮掷3次骰子", "选择得分类别", "13轮后总分最高者胜"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "一点到六点",
            "三条/四条/五条",
            "小顺/大顺",
            "满堂红",
            "快艇",
        ]
    }
}
