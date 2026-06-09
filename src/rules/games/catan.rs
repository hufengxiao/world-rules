//! 卡坦岛规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: CatanRules,
    name: "卡坦岛规则",
    desc: "卡坦岛桌游规则",
    origin: "德国",
    tags: ["游戏", "桌游"],
    category: RuleCategory::games("catan"),
    sections: [("资源", section_0), ("建设", section_1), ("交易", section_2)]
}

impl CatanRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["木材/砖块/羊毛/麦子/矿石"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["道路/村庄/城市/发展卡"]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec!["玩家间交易", "港口交易", "银行4:1交易"]
    }
}
