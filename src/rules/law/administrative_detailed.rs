//! 行政法详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: AdministrativeDetailedRules,
    name: "行政法详解",
    desc: "行政法详解",
    origin: "中国",
    tags: ["法律", "行政"],
    category: RuleCategory::law("administrative_detailed"),
    sections: [("行为", section_0), ("救济", section_1)]
}

impl AdministrativeDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["行政处罚", "行政许可"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["行政复议", "国家赔偿"]
    }
}
