//! 跆拳道详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: TaekwondoDetailedRules,
    name: "跆拳道详细规则",
    desc: "跆拳道详细比赛规则",
    origin: "WTF",
    tags: ["体育", "格斗"],
    category: RuleCategory::sports("taekwondo_detailed"),
    sections: [("得分", section_0), ("电子护具", section_1)]
}

impl TaekwondoDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["1分踢躯干", "3分旋转踢头"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["感应区域"]
    }
}
