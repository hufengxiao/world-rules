//! 高血压管理规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: HypertensionManagementRules, name: "高血压管理规则", desc: "高血压管理规则", origin: "国际", tags: ["健康", "慢性病"] }
impl HypertensionManagementRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["低盐饮食"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["规律运动"]
    }
}
impl Rule for HypertensionManagementRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("hypertension_management")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "高血压管理规则",
            &[("饮食", &self.section_0()), ("运动", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = HypertensionManagementRules::new();
        assert!(!r.explain().is_empty());
    }
}
