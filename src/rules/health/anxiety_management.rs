//! 焦虑管理规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: AnxietyManagementRules, name: "焦虑管理规则", desc: "焦虑管理规则", origin: "国际", tags: ["健康", "心理"] }
impl AnxietyManagementRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["症状识别"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["认知行为"]
    }
}
impl Rule for AnxietyManagementRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("anxiety_management")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "焦虑管理规则",
            &[("识别", &self.section_0()), ("应对", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AnxietyManagementRules::new();
        assert!(!r.explain().is_empty());
    }
}
