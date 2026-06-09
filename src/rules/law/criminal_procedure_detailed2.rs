//! 刑诉法详解2
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: CriminalProcedureDetailed2Rules,
    name: "刑诉法详解2",
    desc: "刑诉法详解2",
    origin: "中国",
    tags: ["法律", "诉讼"],
    category: RuleCategory::law("criminal_procedure_detailed2"),
    sections: [("侦查", section_0), ("检察", section_1)]
}

impl CriminalProcedureDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["讯问", "搜查", "扣押"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["审查起诉", "不起诉", "附条件不起诉"]
    }
}
