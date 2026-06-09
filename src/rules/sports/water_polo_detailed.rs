//! 水球详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: WaterPoloDetailedRules,
    name: "水球详细规则",
    desc: "水球详细比赛规则",
    origin: "FINA",
    tags: ["体育", "水上"],
    category: RuleCategory::sports("water_polo_detailed"),
    sections: [("比赛", section_0), ("犯规", section_1)]
}

impl WaterPoloDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["4节", "30秒进攻"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["普通犯规", "罚出场"]
    }
}
