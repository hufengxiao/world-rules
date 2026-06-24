//! 印度礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: IndianEtiquetteRules, name: "印度礼仪", desc: "印度传统礼仪", origin: "印度", tags: ["社交", "文化"] }
impl IndianEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["合十礼"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["右手用餐"]
    }
}
impl Rule for IndianEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("indian_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "印度礼仪",
            &[("问候", &self.section_0()), ("饮食", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = IndianEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
