//! 国际数据保护法
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: DataProtectionIntlRules,
    name: "国际数据保护法",
    desc: "国际数据保护法",
    origin: "国际",
    tags: ["法律", "数据"],
    category: RuleCategory::law("data_protection_intl"),
    sections: [("GDPR", section_0), ("执法", section_1)]
}

impl DataProtectionIntlRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["合法性基础", "数据主体权利"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["罚款", "影响评估"]
    }
}
