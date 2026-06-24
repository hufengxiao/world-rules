//! 演讲礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PresentationEtiquetteRules, name: "演讲礼仪", desc: "商务演讲礼仪", origin: "国际", tags: ["社交", "职场"] }
impl PresentationEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["充分准备"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["眼神交流"]
    }
}
impl Rule for PresentationEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("presentation_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "演讲礼仪",
            &[("准备", &self.section_0()), ("互动", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PresentationEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
