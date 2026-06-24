//! 巴西礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BrazilianEtiquetteRules, name: "巴西礼仪", desc: "巴西社交礼仪", origin: "巴西", tags: ["社交", "文化"] }
impl BrazilianEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["拥抱贴面"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["时间观念"]
    }
}
impl Rule for BrazilianEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("brazilian_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "巴西礼仪",
            &[("热情", &self.section_0()), ("时间", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BrazilianEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
