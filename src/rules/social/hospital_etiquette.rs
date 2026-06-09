//! 医院礼仪
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: HospitalEtiquetteRules,
    name: "医院礼仪",
    desc: "医院就诊礼仪",
    origin: "中国",
    tags: ["社交", "医疗"],
    category: RuleCategory::social("hospital_etiquette"),
    sections: [("就诊", section_0), ("候诊", section_1)]
}

impl HospitalEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["按号就诊", "如实描述病情", "尊重医生"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["保持安静", "不占用急救通道", "照顾老弱"]
    }
}
