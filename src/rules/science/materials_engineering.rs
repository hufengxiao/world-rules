//! 材料工程定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: MaterialsEngineeringRules,
    name: "材料工程定律",
    desc: "材料工程定律",
    origin: "国际",
    tags: ["科学", "材料"],
    category: RuleCategory::science("materials_engineering"),
    sections: [("金属", section_0), ("复合材料", section_1)]
}

impl MaterialsEngineeringRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["合金设计", "热处理", "腐蚀防护"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["纤维增强", "层合板"]
    }
}
