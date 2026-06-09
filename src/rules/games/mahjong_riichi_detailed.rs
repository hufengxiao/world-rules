//! 立直麻将详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: MahjongRiichiDetailedRules,
    name: "立直麻将详细规则",
    desc: "日本立直麻将详细规则",
    origin: "日本",
    tags: ["游戏", "麻将"],
    category: RuleCategory::games("mahjong_riichi_detailed"),
    sections: [("立直", section_0), ("役", section_1)]
}

impl MahjongRiichiDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["宣告立直", "一发", "里宝牌"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["役满", "振听规则", "流局"]
    }
}
