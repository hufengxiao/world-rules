//! 纽约公约规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: NewYorkConventionRules, name: "纽约公约规则", desc: "纽约公约仲裁裁决", origin: "国际", tags: ["法律", "国际"] }
impl NewYorkConventionRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["仲裁裁决承认"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["裁决执行"]
    }
}
impl Rule for NewYorkConventionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("new_york_convention")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "纽约公约规则",
            &[("承认", &self.section_0()), ("执行", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = NewYorkConventionRules::new();
        assert!(!r.explain().is_empty());
    }
}
