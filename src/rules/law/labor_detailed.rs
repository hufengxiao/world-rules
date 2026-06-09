//! 劳动法详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: LaborDetailedRules,
    name: "劳动法详解",
    desc: "劳动法详解",
    origin: "中国",
    tags: ["法律", "劳动"],
    category: RuleCategory::law("labor_detailed"),
    sections: [("合同", section_0), ("保护", section_1)]
}

impl LaborDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["固定期限", "试用期"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["工时制度", "带薪年假"]
    }
}
