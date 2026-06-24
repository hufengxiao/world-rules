//! 法国礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: FrenchEtiquetteRules, name: "法国礼仪", desc: "法国社交礼仪", origin: "法国", tags: ["社交", "文化"] }
impl FrenchEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["贴面礼"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["法式用餐"]
    }
}
impl Rule for FrenchEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("french_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "法国礼仪",
            &[("贴面", &self.section_0()), ("用餐", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = FrenchEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
