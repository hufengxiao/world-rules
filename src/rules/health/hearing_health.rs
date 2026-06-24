//! 听力健康规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: HearingHealthRules, name: "听力健康规则", desc: "听力健康规则", origin: "国际", tags: ["健康", "听力"] }
impl HearingHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["音量限制"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["定期检查"]
    }
}
impl Rule for HearingHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("hearing_health")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "听力健康规则",
            &[("保护", &self.section_0()), ("检查", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = HearingHealthRules::new();
        assert!(!r.explain().is_empty());
    }
}
