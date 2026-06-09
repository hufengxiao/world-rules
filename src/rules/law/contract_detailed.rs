//! 合同法详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: ContractDetailedRules,
    name: "合同法详解",
    desc: "合同法详解",
    origin: "中国",
    tags: ["法律", "民法"],
    category: RuleCategory::law("contract_detailed"),
    sections: [("订立", section_0), ("违约", section_1)]
}

impl ContractDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["要约承诺", "格式条款"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["继续履行", "损害赔偿"]
    }
}
