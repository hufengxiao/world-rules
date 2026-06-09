//! 德州扑克详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: TexasHoldemDetailedRules,
    name: "德州扑克详细规则",
    desc: "德州扑克详细比赛规则",
    origin: "美国",
    tags: ["游戏", "扑克"],
    category: RuleCategory::games("texas_holdem_detailed"),
    sections: [("位置", section_0), ("下注轮", section_1)]
}

impl TexasHoldemDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["庄家", "小盲注", "大盲注"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["翻牌前", "翻牌", "转牌", "河牌"]
    }
}
