//! 掰手腕规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ArmWrestlingRules, name: "掰手腕规则", desc: "掰手腕竞赛规则", origin: "国际", tags: ["体育", "力量"] }
impl ArmWrestlingRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["肘部不得离桌"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["体重分级"]
    }
}
impl Rule for ArmWrestlingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("arm_wrestling")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "掰手腕规则",
            &[("规则", &self.section_0()), ("级别", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ArmWrestlingRules::new();
        assert!(!r.explain().is_empty());
    }
}
