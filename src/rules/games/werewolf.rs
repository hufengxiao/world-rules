//! 狼人杀规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: WerewolfRules,
    name: "狼人杀规则",
    desc: "狼人杀派对游戏规则",
    origin: "中国",
    tags: ["游戏", "派对"],
    category: RuleCategory::games("werewolf"),
    sections: [("角色", section_0), ("流程", section_1), ("胜负", section_2)]
}

impl WerewolfRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["狼人/村民/预言家/女巫/猎人/守卫"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["天黑闭眼", "狼人/神职依次行动", "天亮讨论投票"]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec!["狼人全出局村民胜", "狼人>=村民数狼人胜"]
    }
}
