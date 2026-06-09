//! 拳击详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: BoxingDetailedRules,
    name: "拳击详细规则",
    desc: "拳击详细比赛规则",
    origin: "WBA",
    tags: ["体育", "格斗"],
    category: RuleCategory::sports("boxing_detailed"),
    sections: [("回合", section_0), ("得分", section_1)]
}

impl BoxingDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["3分钟一回合", "KO判定"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["有效打击", "点数判定"]
    }
}
