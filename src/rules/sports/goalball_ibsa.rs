//! 门球IBSA规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: GoalballIbsaRules, name: "门球IBSA规则", desc: "IBSA盲人门球规则", origin: "国际", tags: ["体育", "残疾人"] }
impl GoalballIbsaRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["发声球眼罩"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["2x10分钟"]
    }
}
impl Rule for GoalballIbsaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("goalball_ibsa")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "门球IBSA规则",
            &[("特殊", &self.section_0()), ("比赛", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = GoalballIbsaRules::new();
        assert!(!r.explain().is_empty());
    }
}
