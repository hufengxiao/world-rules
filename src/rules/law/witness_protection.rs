//! 证人保护法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: WitnessProtectionRules, name: "证人保护法", desc: "证人保护法律规则", origin: "国际", tags: ["法律", "保护"] }
impl WitnessProtectionRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["人身保护"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["身份变更"]
    }
}
impl Rule for WitnessProtectionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("witness_protection")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "证人保护法",
            &[("保护", &self.section_0()), ("措施", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = WitnessProtectionRules::new();
        assert!(!r.explain().is_empty());
    }
}
