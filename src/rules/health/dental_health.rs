//! 牙齿健康规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DentalHealthRules, name: "牙齿健康规则", desc: "牙齿健康规则", origin: "国际", tags: ["健康", "口腔"] }
impl DentalHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["刷牙牙线"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["半年检查"]
    }
}
impl Rule for DentalHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("dental_health")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "牙齿健康规则",
            &[("清洁", &self.section_0()), ("检查", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DentalHealthRules::new();
        assert!(!r.explain().is_empty());
    }
}
