//! 机密代号规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: CodenamesRules,
    name: "机密代号规则",
    desc: "Codenames桌游规则",
    origin: "捷克",
    tags: ["游戏", "桌游"],
    category: RuleCategory::games("codenames"),
    sections: [("角色", section_0), ("流程", section_1), ("胜负", section_2)]
}

impl CodenamesRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["两名间谍头目", "其余为特工"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["间谍头目给提示词+数字", "己方特工猜词", "翻牌确认"]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec!["先翻完己方所有词胜", "翻到暗杀者则输"]
    }
}
