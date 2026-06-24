//! 中东礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MiddleEastEtiquetteRules, name: "中东礼仪", desc: "中东地区礼仪", origin: "中东", tags: ["社交", "文化"] }
impl MiddleEastEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["同性握手"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["清真饮食"]
    }
}
impl Rule for MiddleEastEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("middle_east_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中东礼仪",
            &[("握手", &self.section_0()), ("饮食", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MiddleEastEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
