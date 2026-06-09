//! Risk世界征服规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: RiskRules,
    name: "Risk世界征服规则",
    desc: "Risk桌游规则",
    origin: "法国",
    tags: ["游戏", "桌游"],
    category: RuleCategory::games("risk"),
    sections: [("游戏目标", section_0), ("回合", section_1)]
}

impl RiskRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["占领全部领土获胜"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["获得增援", "进攻相邻", "调防"]
    }
}
