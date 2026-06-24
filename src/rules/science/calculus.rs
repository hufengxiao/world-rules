//! 微积分定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CalculusRules, name: "微积分定律", desc: "微积分定律", origin: "国际", tags: ["科学", "数学"] }
impl CalculusRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["导数链式法则"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["不定积分定积分"]
    }
}
impl Rule for CalculusRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("calculus")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "微积分定律",
            &[("微分", &self.section_0()), ("积分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CalculusRules::new();
        assert!(!r.explain().is_empty());
    }
}
