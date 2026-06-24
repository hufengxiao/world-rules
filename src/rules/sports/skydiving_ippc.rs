//! 跳伞IPPC规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SkydivingIppcRules, name: "跳伞IPPC规则", desc: "跳伞国际规则", origin: "国际", tags: ["体育", "极限"] }
impl SkydivingIppcRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["自由落体滑翔"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["备用伞"]
    }
}
impl Rule for SkydivingIppcRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("skydiving_ippc")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "跳伞IPPC规则",
            &[("类型", &self.section_0()), ("安全", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SkydivingIppcRules::new();
        assert!(!r.explain().is_empty());
    }
}
