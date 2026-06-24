//! 德国礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: GermanEtiquetteRules, name: "德国礼仪", desc: "德国社交礼仪", origin: "德国", tags: ["社交", "文化"] }
impl GermanEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["严格守时"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["握手有力"]
    }
}
impl Rule for GermanEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("german_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "德国礼仪",
            &[("守时", &self.section_0()), ("握手", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = GermanEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
