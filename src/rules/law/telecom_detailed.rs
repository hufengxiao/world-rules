//! 电信法详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: TelecomDetailedRules,
    name: "电信法详解",
    desc: "电信法详解",
    origin: "中国",
    tags: ["法律", "通信"],
    category: RuleCategory::law("telecom_detailed"),
    sections: [("许可", section_0), ("监管", section_1)]
}

impl TelecomDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["基础电信", "增值电信"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["互联互通", "资费管理"]
    }
}
