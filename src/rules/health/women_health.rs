//! 女性健康规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: WomenHealthRules, name: "女性健康规则", desc: "女性健康规则", origin: "国际", tags: ["健康", "女性"] }
impl WomenHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["乳腺宫颈"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["铁钙"]
    }
}
impl Rule for WomenHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("women_health")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "女性健康规则",
            &[("筛查", &self.section_0()), ("营养", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = WomenHealthRules::new();
        assert!(!r.explain().is_empty());
    }
}
