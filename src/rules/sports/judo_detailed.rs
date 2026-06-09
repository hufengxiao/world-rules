//! 柔道详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: JudoDetailedRules,
    name: "柔道详细规则",
    desc: "柔道详细比赛规则",
    origin: "IJF",
    tags: ["体育", "格斗"],
    category: RuleCategory::sports("judo_detailed"),
    sections: [("得分", section_0), ("犯规", section_1)]
}

impl JudoDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["一本", "技有"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["禁止动作", "消极比赛"]
    }
}
