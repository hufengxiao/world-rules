//! 举报人保护法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: WhistleblowerProtectionRules, name: "举报人保护法", desc: "举报人保护法律规则", origin: "国际", tags: ["法律", "保护"] }
impl WhistleblowerProtectionRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["不报复"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["举报奖励"]
    }
}
impl Rule for WhistleblowerProtectionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("whistleblower_protection")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "举报人保护法",
            &[("保护", &self.section_0()), ("激励", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = WhistleblowerProtectionRules::new();
        assert!(!r.explain().is_empty());
    }
}
