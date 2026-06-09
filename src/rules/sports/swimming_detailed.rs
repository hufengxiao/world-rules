//! 游泳详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: SwimmingDetailedRules,
    name: "游泳详细规则",
    desc: "游泳详细比赛规则",
    origin: "FINA",
    tags: ["体育", "水上"],
    category: RuleCategory::sports("swimming_detailed"),
    sections: [("泳姿", section_0), ("转身", section_1)]
}

impl SwimmingDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["自由泳", "蛙泳", "蝶泳", "仰泳"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["转身规则"]
    }
}
