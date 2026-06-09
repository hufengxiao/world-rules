//! 刑诉法详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: CriminalProcedureDetailedRules,
    name: "刑诉法详解",
    desc: "刑事诉讼法详解",
    origin: "中国",
    tags: ["法律", "诉讼"],
    category: RuleCategory::law("criminal_procedure_detailed"),
    sections: [("强制措施", section_0), ("审判", section_1)]
}

impl CriminalProcedureDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["取保候审", "逮捕"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["一审", "二审", "再审"]
    }
}
