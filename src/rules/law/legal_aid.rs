//! 法律援助法
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: LegalAidRules,
    name: "法律援助法",
    desc: "法律援助法律规则",
    origin: "中国",
    tags: ["法律", "援助"],
    category: RuleCategory::law("legal_aid"),
    sections: [("范围", section_0), ("条件", section_1)]
}

impl LegalAidRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["刑事辩护", "民事代理"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["经济困难", "值班律师"]
    }
}
