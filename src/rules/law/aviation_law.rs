//! 航空法详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: AviationLawRules,
    name: "航空法详解",
    desc: "民用航空法律规则",
    origin: "中国",
    tags: ["法律", "航空"],
    category: RuleCategory::law("aviation_law"),
    sections: [("运营", section_0), ("责任", section_1)]
}

impl AviationLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["适航管理", "航线管理"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["承运人责任", "事故调查"]
    }
}
