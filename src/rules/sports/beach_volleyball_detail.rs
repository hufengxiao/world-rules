//! 沙排详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: BeachVolleyballDetailRules,
    name: "沙排详细规则",
    desc: "沙滩排球详细规则",
    origin: "FIVB",
    tags: ["体育", "沙滩"],
    category: RuleCategory::sports("beach_volleyball_detail"),
    sections: [("比赛", section_0), ("特殊", section_1)]
}

impl BeachVolleyballDetailRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["2人制", "15分制"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["换人限制"]
    }
}
