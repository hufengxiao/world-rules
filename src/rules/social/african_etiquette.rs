//! 非洲礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: AfricanEtiquetteRules, name: "非洲礼仪", desc: "非洲地区礼仪", origin: "非洲", tags: ["社交", "文化"] }
impl AfricanEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["长篇问候"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["右手递物"]
    }
}
impl Rule for AfricanEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("african_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "非洲礼仪",
            &[("问候", &self.section_0()), ("右手", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AfricanEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
