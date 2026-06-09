//! 税法详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: TaxDetailedRules,
    name: "税法详解",
    desc: "税法详解",
    origin: "中国",
    tags: ["法律", "税法"],
    category: RuleCategory::law("tax_detailed"),
    sections: [("流转税", section_0), ("所得税", section_1)]
}

impl TaxDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["增值税", "消费关税"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["企业所得税", "个人所得税"]
    }
}
