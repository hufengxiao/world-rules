//! 植物生理学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: PlantPhysiologyRules,
    name: "植物生理学定律",
    desc: "植物生理学定律",
    origin: "国际",
    tags: ["科学", "生物"],
    category: RuleCategory::science("plant_physiology"),
    sections: [("光合", section_0), ("激素", section_1)]
}

impl PlantPhysiologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["光反应", "暗反应", "C3/C4/CAM"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["生长素", "赤霉素", "脱落酸"]
    }
}
