//! 肾脏健康规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: KidneyHealthRules, name: "肾脏健康规则", desc: "肾脏健康规则", origin: "国际", tags: ["健康", "器官"] }
impl KidneyHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["充足饮水"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["低盐低蛋白"]
    }
}
impl Rule for KidneyHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("kidney_health")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "肾脏健康规则",
            &[("保护", &self.section_0()), ("饮食", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = KidneyHealthRules::new();
        assert!(!r.explain().is_empty());
    }
}
