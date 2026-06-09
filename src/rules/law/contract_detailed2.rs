//! 合同法详解2
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: ContractDetailed2Rules,
    name: "合同法详解2",
    desc: "合同法详解2",
    origin: "中国",
    tags: ["法律", "民法"],
    category: RuleCategory::law("contract_detailed2"),
    sections: [("效力", section_0), ("解除", section_1)]
}

impl ContractDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["无效合同", "可撤销合同"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["法定解除", "约定解除"]
    }
}
