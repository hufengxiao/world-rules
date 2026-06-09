//! 双骰规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: CrapsRules,
    name: "双骰规则",
    desc: "双骰(Craps)游戏规则",
    origin: "美国",
    tags: ["游戏", "骰子"],
    category: RuleCategory::games("craps"),
    sections: [("基本规则", section_0), ("点数阶段", section_1)]
}

impl CrapsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["掷两枚骰子", "首掷7或11赢", "首掷2/3/12输"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["其他数字成为目标点", "再次掷到目标点赢", "掷到7输"]
    }
}
