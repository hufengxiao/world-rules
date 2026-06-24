//! 计算机网络定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ComputerNetworksRules, name: "计算机网络定律", desc: "计算机网络定律", origin: "国际", tags: ["科学", "计算机"] }
impl ComputerNetworksRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["TCP/IP HTTP"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["加密认证"]
    }
}
impl Rule for ComputerNetworksRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("computer_networks")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "计算机网络定律",
            &[("协议", &self.section_0()), ("安全", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ComputerNetworksRules::new();
        assert!(!r.explain().is_empty());
    }
}
