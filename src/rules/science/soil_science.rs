//! 土壤科学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: SoilScienceRules,
    name: "土壤科学定律",
    desc: "土壤科学定律",
    origin: "国际",
    tags: ["科学", "地球"],
    category: RuleCategory::science("soil_science"),
    sections: [("组成", section_0), ("过程", section_1)]
}

impl SoilScienceRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["矿物质", "有机质", "水分", "空气"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["风化", "淋溶", "淀积"]
    }
}
