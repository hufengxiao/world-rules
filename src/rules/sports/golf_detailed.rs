//! 高尔夫详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: GolfDetailedRules,
    name: "高尔夫详细规则",
    desc: "高尔夫详细比赛规则",
    origin: "R&A",
    tags: ["体育", "球类"],
    category: RuleCategory::sports("golf_detailed"),
    sections: [("比赛", section_0), ("障碍", section_1)]
}

impl GolfDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["比杆赛", "比洞赛"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["沙坑", "水障碍"]
    }
}
