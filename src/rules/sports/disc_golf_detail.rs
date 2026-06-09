//! 飞盘高尔夫详细
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: DiscGolfDetailRules,
    name: "飞盘高尔夫详细",
    desc: "飞盘高尔夫详细规则",
    origin: "PDGA",
    tags: ["体育", "休闲"],
    category: RuleCategory::sports("disc_golf_detail"),
    sections: [("比赛", section_0), ("盘型", section_1)]
}

impl DiscGolfDetailRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["18洞", "最少投掷"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["推杆盘", "远距离盘"]
    }
}
