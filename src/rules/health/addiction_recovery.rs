//! 成瘾康复规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: AddictionRecoveryRules, name: "成瘾康复规则", desc: "成瘾康复规则", origin: "国际", tags: ["健康", "心理"] }
impl AddictionRecoveryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["酒精烟草"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["12步"]
    }
}
impl Rule for AddictionRecoveryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("addiction_recovery")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "成瘾康复规则",
            &[("类型", &self.section_0()), ("康复", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AddictionRecoveryRules::new();
        assert!(!r.explain().is_empty());
    }
}
