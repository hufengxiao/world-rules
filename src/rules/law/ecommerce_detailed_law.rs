//! 电商法详解
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: EcommerceDetailedLawRules, name: "电商法详解", desc: "电商法详解", origin: "中国", tags: ["法律", "电商"] }
impl EcommerceDetailedLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["平台责任"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["登记纳税"]
    }
}
impl Rule for EcommerceDetailedLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("ecommerce_detailed_law")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "电商法详解",
            &[("平台", &self.section_0()), ("经营", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = EcommerceDetailedLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
