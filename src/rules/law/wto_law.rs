//! WTO法律规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: WtoLawRules, name: "WTO法律规则", desc: "WTO国际贸易规则", origin: "国际", tags: ["法律", "国际"] }
impl WtoLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["最惠国国民待遇"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["争端解决机制"]
    }
}
impl Rule for WtoLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("wto_law")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "WTO法律规则",
            &[("原则", &self.section_0()), ("争端", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = WtoLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
