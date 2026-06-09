//! 西洋双陆棋规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: BackgammonRules,
    name: "西洋双陆棋规则",
    desc: "西洋双陆棋规则",
    origin: "国际",
    tags: ["游戏", "棋类"],
    category: RuleCategory::games("backgammon"),
    sections: [("走法", section_0), ("策略", section_1)]
}

impl BackgammonRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["掷骰子移动", "点数对应步数"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["建立防线", "封锁对手", " bearing off"]
    }
}
