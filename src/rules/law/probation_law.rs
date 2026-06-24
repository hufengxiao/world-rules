//! 缓刑法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ProbationLawRules, name: "缓刑法", desc: "缓刑法律规则", origin: "国际", tags: ["法律", "刑事"] }
impl ProbationLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["缓刑条件"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["社区监管"]
    }
}
impl Rule for ProbationLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("probation_law")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "缓刑法",
            &[("条件", &self.section_0()), ("监管", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ProbationLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
