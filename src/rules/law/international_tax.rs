//! 国际税法详细
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: InternationalTaxRules, name: "国际税法详细", desc: "国际税收法律规则", origin: "国际", tags: ["法律", "税法"] }
impl InternationalTaxRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["税收协定"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["反避税"]
    }
}
impl Rule for InternationalTaxRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("international_tax")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "国际税法详细",
            &[("协定", &self.section_0()), ("避税", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = InternationalTaxRules::new();
        assert!(!r.explain().is_empty());
    }
}
