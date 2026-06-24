//! 环境伦理定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: EnvironmentalEthicsRules, name: "环境伦理定律", desc: "环境伦理定律", origin: "国际", tags: ["科学", "伦理"] }
impl EnvironmentalEthicsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["代际公平"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["气候变化"]
    }
}
impl Rule for EnvironmentalEthicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("environmental_ethics")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "环境伦理定律",
            &[("原则", &self.section_0()), ("问题", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = EnvironmentalEthicsRules::new();
        assert!(!r.explain().is_empty());
    }
}
