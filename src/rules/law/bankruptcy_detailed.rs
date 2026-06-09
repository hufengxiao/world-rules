//! 破产法详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: BankruptcyDetailedRules,
    name: "破产法详解",
    desc: "破产法详解",
    origin: "中国",
    tags: ["法律", "商法"],
    category: RuleCategory::law("bankruptcy_detailed"),
    sections: [("程序", section_0), ("重整", section_1)]
}

impl BankruptcyDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["破产申请", "管理人"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["重整计划", "和解"]
    }
}
