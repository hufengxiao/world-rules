//! 狭义相对论定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: RelativitySpecialRules, name: "狭义相对论定律", desc: "狭义相对论定律", origin: "国际", tags: ["科学", "物理"] }
impl RelativitySpecialRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["时间膨胀", "长度收缩"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["质能方程"]
    }
}
impl Rule for RelativitySpecialRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("relativity_special")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "狭义相对论定律",
            &[("基本", &self.section_0()), ("推论", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = RelativitySpecialRules::new();
        assert!(!r.explain().is_empty());
    }
}
