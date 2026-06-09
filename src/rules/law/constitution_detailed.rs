//! 宪法详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: ConstitutionDetailedRules,
    name: "宪法详解",
    desc: "宪法基本权利详解",
    origin: "中国",
    tags: ["法律", "宪法"],
    category: RuleCategory::law("constitution_detailed"),
    sections: [("基本权利", section_0), ("国家机构", section_1)]
}

impl ConstitutionDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["平等权", "自由权", "社会权"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["人大制度", "国务院"]
    }
}
