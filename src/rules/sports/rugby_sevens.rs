//! 七人制橄榄球
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: RugbySevensRules,
    name: "七人制橄榄球",
    desc: "七人制橄榄球规则",
    origin: "WR",
    tags: ["体育", "球类"],
    category: RuleCategory::sports("rugby_sevens"),
    sections: [("比赛", section_0), ("得分", section_1)]
}

impl RugbySevensRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["7分钟半场"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["达阵5分"]
    }
}
