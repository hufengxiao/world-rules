//! 眼睛健康规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: EyeHealthRules,
    name: "眼睛健康规则",
    desc: "眼睛健康保护规则",
    origin: "国际",
    tags: ["健康", "视力"],
    category: RuleCategory::health("eye_health"),
    sections: [("用眼", section_0), ("保护", section_1)]
}

impl EyeHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["20-20-20法则", "保持距离", "充足光线"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["佩戴太阳镜", "防蓝光", "眼保健操"]
    }
}
