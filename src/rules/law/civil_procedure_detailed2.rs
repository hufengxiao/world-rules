//! 民诉法详解2
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: CivilProcedureDetailed2Rules,
    name: "民诉法详解2",
    desc: "民诉法详解2",
    origin: "中国",
    tags: ["法律", "诉讼"],
    category: RuleCategory::law("civil_procedure_detailed2"),
    sections: [("执行", section_0), ("保全", section_1)]
}

impl CivilProcedureDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["执行措施", "执行异议"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["财产保全", "行为保全"]
    }
}
