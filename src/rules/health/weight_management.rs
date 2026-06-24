//! 体重管理规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: WeightManagementRules, name: "体重管理规则", desc: "体重管理规则", origin: "国际", tags: ["健康", "营养"] }
impl WeightManagementRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["热量控制"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["有氧力量"]
    }
}
impl Rule for WeightManagementRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("weight_management")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "体重管理规则",
            &[("饮食", &self.section_0()), ("运动", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = WeightManagementRules::new();
        assert!(!r.explain().is_empty());
    }
}
