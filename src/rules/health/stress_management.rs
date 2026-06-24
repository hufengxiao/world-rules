//! 压力管理规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: StressManagementRules, name: "压力管理规则", desc: "压力管理规则", origin: "国际", tags: ["健康", "心理"] }
impl StressManagementRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["深呼吸冥想"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["运动社交"]
    }
}
impl Rule for StressManagementRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("stress_management")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "压力管理规则",
            &[("技巧", &self.section_0()), ("生活", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = StressManagementRules::new();
        assert!(!r.explain().is_empty());
    }
}
