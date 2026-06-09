//! 牛牛规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: NiuniuRules,
    name: "牛牛规则",
    desc: "牛牛扑克游戏规则",
    origin: "中国",
    tags: ["游戏", "扑克"],
    category: RuleCategory::games("niuniu"),
    sections: [("牌型规则", section_0), ("倍数规则", section_1)]
}

impl NiuniuRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["牛牛=3张凑10倍数", "有牛/无牛", "牛1到牛9"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["牛牛3倍", "牛7到牛9两倍", "牛1到牛6一倍", "无牛1倍"]
    }
}
