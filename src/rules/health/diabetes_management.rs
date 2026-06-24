//! 糖尿病管理规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DiabetesManagementRules, name: "糖尿病管理规则", desc: "糖尿病管理规则", origin: "国际", tags: ["健康", "慢性病"] }
impl DiabetesManagementRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["控制碳水"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["血糖监测"]
    }
}
impl Rule for DiabetesManagementRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("diabetes_management")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "糖尿病管理规则",
            &[("饮食", &self.section_0()), ("监测", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DiabetesManagementRules::new();
        assert!(!r.explain().is_empty());
    }
}
