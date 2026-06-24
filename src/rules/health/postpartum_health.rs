//! 产后健康规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PostpartumHealthRules, name: "产后健康规则", desc: "产后健康规则", origin: "国际", tags: ["健康", "产后"] }
impl PostpartumHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["身体恢复"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["产后抑郁"]
    }
}
impl Rule for PostpartumHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("postpartum_health")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "产后健康规则",
            &[("恢复", &self.section_0()), ("心理", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PostpartumHealthRules::new();
        assert!(!r.explain().is_empty());
    }
}
