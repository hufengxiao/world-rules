//! 保险法详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: InsuranceDetailedRules,
    name: "保险法详解",
    desc: "保险法律规则",
    origin: "中国",
    tags: ["法律", "金融"],
    category: RuleCategory::law("insurance_law"),
    sections: [("合同", section_0), ("监管", section_1)]
}

impl InsuranceDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["保险利益", "如实告知"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["偿付能力", "准备金"]
    }
}
