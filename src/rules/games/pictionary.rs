//! 你画我猜规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: PictionaryRules,
    name: "你画我猜规则",
    desc: "你画我猜派对游戏规则",
    origin: "美国",
    tags: ["游戏", "派对"],
    category: RuleCategory::games("pictionary"),
    sections: [("游戏流程", section_0), ("规则", section_1)]
}

impl PictionaryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["抽取提示词", "限时画画", "队友猜测"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["不能写数字字母", "不能说话提示", "限时60秒"]
    }
}
