//! 国际消费者保护
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ConsumerProtectionIntlRules, name: "国际消费者保护", desc: "国际消费者保护法", origin: "国际", tags: ["法律", "消费"] }
impl ConsumerProtectionIntlRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["安全知情选择"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["集体诉讼"]
    }
}
impl Rule for ConsumerProtectionIntlRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("consumer_protection_intl")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "国际消费者保护",
            &[("权利", &self.section_0()), ("救济", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ConsumerProtectionIntlRules::new();
        assert!(!r.explain().is_empty());
    }
}
