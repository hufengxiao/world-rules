//! UNCITRAL规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: UncitralLawRules, name: "UNCITRAL规则", desc: "UNCITRAL仲裁规则", origin: "国际", tags: ["法律", "国际"] }
impl UncitralLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["仲裁规则"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["调解规则"]
    }
}
impl Rule for UncitralLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("uncitral_law")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "UNCITRAL规则",
            &[("规则", &self.section_0()), ("调解", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = UncitralLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
