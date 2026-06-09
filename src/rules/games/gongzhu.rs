//! 拱猪规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: GongzhuRules,
    name: "拱猪规则",
    desc: "拱猪卡牌游戏规则",
    origin: "中国",
    tags: ["游戏", "卡牌"],
    category: RuleCategory::games("gongzhu"),
    sections: [("特殊牌", section_0), ("出牌规则", section_1)]
}

impl GongzhuRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "猪(黑桃Q)负100分",
            "羊(方块J)正100分",
            "变压器(梅花10)翻倍",
            "红心每张负分",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["梅花2先出", "同花跟牌", "无同花可垫牌"]
    }
}
