//! 素食礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: VeganEtiquetteRules, name: "素食礼仪", desc: "素食者社交礼仪", origin: "国际", tags: ["社交", "饮食"] }
impl VeganEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["提前告知"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["互相尊重"]
    }
}
impl Rule for VeganEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("vegan_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "素食礼仪",
            &[("聚餐", &self.section_0()), ("尊重", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = VeganEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
