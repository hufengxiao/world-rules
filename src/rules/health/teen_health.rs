//! 青少年健康规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: TeenHealthRules, name: "青少年健康规则", desc: "青少年健康规则", origin: "国际", tags: ["健康", "青少年"] }
impl TeenHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["钙铁锌"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["心理健康"]
    }
}
impl Rule for TeenHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("teen_health")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "青少年健康规则",
            &[("营养", &self.section_0()), ("心理", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = TeenHealthRules::new();
        assert!(!r.explain().is_empty());
    }
}
