//! 知识产权详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: IpDetailedRules,
    name: "知识产权详解",
    desc: "知识产权法详解",
    origin: "中国",
    tags: ["法律", "知识产权"],
    category: RuleCategory::law("ip_detailed"),
    sections: [("专利", section_0), ("版权", section_1)]
}

impl IpDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["发明专利", "实用新型"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["著作权", "合理使用"]
    }
}
