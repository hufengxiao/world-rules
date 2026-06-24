//! 在线争议解决
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: OnlineDisputeRules, name: "在线争议解决", desc: "ODR在线争议规则", origin: "国际", tags: ["法律", "互联网"] }
impl OnlineDisputeRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["在线调解"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["电子仲裁"]
    }
}
impl Rule for OnlineDisputeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("online_dispute")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "在线争议解决",
            &[("平台", &self.section_0()), ("程序", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = OnlineDisputeRules::new();
        assert!(!r.explain().is_empty());
    }
}
