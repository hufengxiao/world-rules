//! 国际人权法
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: InternationalHumanRightsRules,
    name: "国际人权法",
    desc: "国际人权法律规则",
    origin: "国际",
    tags: ["法律", "人权"],
    category: RuleCategory::law("international_human_rights"),
    sections: [("公约", section_0), ("机制", section_1)]
}

impl InternationalHumanRightsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["公民权利", "经济社会文化权利"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["人权理事会"]
    }
}
