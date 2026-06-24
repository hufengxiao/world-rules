//! 可持续生活规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SustainableLivingRules, name: "可持续生活规则", desc: "可持续生活规则", origin: "国际", tags: ["社交", "环保"] }
impl SustainableLivingRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["减少浪费"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["回收利用"]
    }
}
impl Rule for SustainableLivingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("sustainable_living")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "可持续生活规则",
            &[("减量", &self.section_0()), ("循环", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SustainableLivingRules::new();
        assert!(!r.explain().is_empty());
    }
}
