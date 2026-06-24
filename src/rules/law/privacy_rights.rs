//! 隐私权法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PrivacyRightsRules, name: "隐私权法", desc: "隐私权保障法律", origin: "国际", tags: ["法律", "隐私"] }
impl PrivacyRightsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["数据保护"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["加州隐私"]
    }
}
impl Rule for PrivacyRightsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("privacy_rights")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "隐私权法",
            &[("GDPR", &self.section_0()), ("CCPA", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PrivacyRightsRules::new();
        assert!(!r.explain().is_empty());
    }
}
