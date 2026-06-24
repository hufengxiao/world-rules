//! 头发健康规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: HairHealthRules, name: "头发健康规则", desc: "头发健康规则", origin: "国际", tags: ["健康", "头发"] }
impl HairHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["适度洗发"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["蛋白质铁"]
    }
}
impl Rule for HairHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("hair_health")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "头发健康规则",
            &[("清洁", &self.section_0()), ("营养", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = HairHealthRules::new();
        assert!(!r.explain().is_empty());
    }
}
