//! 野生动物保护法
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: WildlifeProtectionLawRules,
    name: "野生动物保护法",
    desc: "野生动物保护法律规则",
    origin: "中国",
    tags: ["法律", "环境"],
    category: RuleCategory::law("wildlife_protection_law"),
    sections: [("保护分级", section_0), ("利用限制", section_1)]
}

impl WildlifeProtectionLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["国家重点保护", "省级保护", "三有动物"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["特许猎捕", "经营利用许可", "进出口管理"]
    }
}
