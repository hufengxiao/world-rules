//! 婚姻法详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: MarriageDetailedRules,
    name: "婚姻法详解",
    desc: "婚姻法详解",
    origin: "中国",
    tags: ["法律", "家庭"],
    category: RuleCategory::law("marriage_detailed"),
    sections: [("结婚", section_0), ("离婚", section_1)]
}

impl MarriageDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["结婚条件", "无效婚姻"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["协议离婚", "子女抚养"]
    }
}
