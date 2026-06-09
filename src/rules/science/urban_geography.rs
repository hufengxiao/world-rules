//! 城市地理学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: UrbanGeographyRules,
    name: "城市地理学定律",
    desc: "城市地理学定律",
    origin: "国际",
    tags: ["科学", "地理"],
    category: RuleCategory::science("urban_geography"),
    sections: [("理论", section_0), ("规划", section_1)]
}

impl UrbanGeographyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["中心地理论", "城市化进程"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["功能分区", "交通网络"]
    }
}
