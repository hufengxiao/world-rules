//! 电力法详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: ElectricityDetailedRules,
    name: "电力法详解",
    desc: "电力法详解",
    origin: "中国",
    tags: ["法律", "能源"],
    category: RuleCategory::law("electricity_detailed"),
    sections: [("发电", section_0), ("供应", section_1)]
}

impl ElectricityDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["可再生能源", "核电安全"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["电力调度", "电价管理"]
    }
}
