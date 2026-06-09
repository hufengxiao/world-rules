//! 三人篮球详细
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: Basketball3x3DetailRules,
    name: "三人篮球详细",
    desc: "三人篮球详细规则",
    origin: "FIBA",
    tags: ["体育", "球类"],
    category: RuleCategory::sports("basketball_3x3_detail"),
    sections: [("比赛", section_0), ("特殊", section_1)]
}

impl Basketball3x3DetailRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["10分钟", "21分获胜"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["12秒进攻"]
    }
}
