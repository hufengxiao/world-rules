//! 客服礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CustomerServiceEtiquetteRules, name: "客服礼仪", desc: "客户服务礼仪", origin: "国际", tags: ["社交", "职场"] }
impl CustomerServiceEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["耐心友好"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["积极解决"]
    }
}
impl Rule for CustomerServiceEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("customer_service_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "客服礼仪",
            &[("态度", &self.section_0()), ("解决", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CustomerServiceEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
