//! 水分摄入规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: HydrationRulesRules, name: "水分摄入规则", desc: "水分摄入健康规则", origin: "国际", tags: ["健康", "饮水"] }
impl HydrationRulesRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["每天8杯"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["运动前后"]
    }
}
impl Rule for HydrationRulesRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("hydration_rules")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "水分摄入规则",
            &[("建议", &self.section_0()), ("时机", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = HydrationRulesRules::new();
        assert!(!r.explain().is_empty());
    }
}
