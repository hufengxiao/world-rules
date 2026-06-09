//! 费舍尔随机棋规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: Chess960Rules,
    name: "费舍尔随机棋规则",
    desc: "Chess960规则",
    origin: "国际",
    tags: ["游戏", "棋类"],
    category: RuleCategory::games("chess960"),
    sections: [("开局", section_0), ("目的", section_1)]
}

impl Chess960Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["随机排列首排棋子", "王车易位规则不变"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["消除开局理论依赖", "增加创造力"]
    }
}
