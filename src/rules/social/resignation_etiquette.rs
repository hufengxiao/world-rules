//! 辞职礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ResignationEtiquetteRules, name: "辞职礼仪", desc: "辞职离职礼仪", origin: "国际", tags: ["社交", "职场"] }
impl ResignationEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["提前通知"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["工作交接"]
    }
}
impl Rule for ResignationEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("resignation_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "辞职礼仪",
            &[("提前", &self.section_0()), ("交接", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ResignationEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
