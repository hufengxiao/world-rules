//! 医院礼仪

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: HospitalEtiquetteRules,
    name: "医院礼仪",
    desc: "医院就诊礼仪",
    origin: "中国",
    tags: ["社交", "医疗"]
}

impl HospitalEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["按号就诊", "如实描述病情", "尊重医生"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["保持安静", "不占用急救通道", "照顾老弱"]
    }
}

impl Rule for HospitalEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("hospital_etiquette")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "医院礼仪",
            &[("就诊", &self.section_0()), ("候诊", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hospital_etiquette_rules() {
        let r = HospitalEtiquetteRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
