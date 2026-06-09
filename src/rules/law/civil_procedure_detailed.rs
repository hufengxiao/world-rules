//! 民诉法详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: CivilProcedureDetailedRules,
    name: "民诉法详解",
    desc: "民事诉讼法详解",
    origin: "中国",
    tags: ["法律", "诉讼"],
    category: RuleCategory::law("civil_procedure_detailed"),
    sections: [("管辖", section_0), ("证据", section_1)]
}

impl CivilProcedureDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["级别管辖", "地域管辖"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["举证责任", "电子证据"]
    }
}
