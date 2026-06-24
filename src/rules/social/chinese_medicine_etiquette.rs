//! 中医问诊礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChineseMedicineEtiquetteRules, name: "中医问诊礼仪", desc: "中医问诊礼节", origin: "中国", tags: ["社交", "医疗"] }
impl ChineseMedicineEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["问诊礼节"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["尊重医师"]
    }
}
impl Rule for ChineseMedicineEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_medicine_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中医问诊礼仪",
            &[("望闻问切", &self.section_0()), ("医患", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChineseMedicineEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
