//! 精神卫生法
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: MentalHealthLawRules,
    name: "精神卫生法",
    desc: "精神卫生法律规则",
    origin: "中国",
    tags: ["法律", "医疗"],
    category: RuleCategory::law("mental_health_law"),
    sections: [("患者权益", section_0), ("诊疗规范", section_1)]
}

impl MentalHealthLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["人格尊严保护", "隐私保护", "通信自由"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["自愿原则", "非自愿住院条件", "治疗规范"]
    }
}
