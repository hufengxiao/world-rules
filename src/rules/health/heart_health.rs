//! 心脏健康规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: HeartHealthRules, name: "心脏健康规则", desc: "心脏健康护理规则", origin: "国际", tags: ["健康", "心脏"] }
impl HeartHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["控制血压血脂"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["有氧运动"]
    }
}
impl Rule for HeartHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("heart_health")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "心脏健康规则",
            &[("预防", &self.section_0()), ("运动", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = HeartHealthRules::new();
        assert!(!r.explain().is_empty());
    }
}
