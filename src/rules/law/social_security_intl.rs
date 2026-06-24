//! 国际社会保障
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SocialSecurityIntlRules, name: "国际社会保障", desc: "国际社会保障法", origin: "国际", tags: ["法律", "社保"] }
impl SocialSecurityIntlRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["养老医疗失业"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["社保协调"]
    }
}
impl Rule for SocialSecurityIntlRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("social_security_intl")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "国际社会保障",
            &[("类型", &self.section_0()), ("协调", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SocialSecurityIntlRules::new();
        assert!(!r.explain().is_empty());
    }
}
