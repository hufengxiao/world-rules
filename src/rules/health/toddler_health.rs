//! 幼儿健康规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ToddlerHealthRules, name: "幼儿健康规则", desc: "幼儿健康规则", origin: "国际", tags: ["健康", "幼儿"] }
impl ToddlerHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["均衡饮食"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["防跌防烫"]
    }
}
impl Rule for ToddlerHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("toddler_health")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "幼儿健康规则",
            &[("营养", &self.section_0()), ("安全", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ToddlerHealthRules::new();
        assert!(!r.explain().is_empty());
    }
}
