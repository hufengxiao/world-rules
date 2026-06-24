//! 排毒规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DetoxRulesRules, name: "排毒规则", desc: "身体排毒健康规则", origin: "国际", tags: ["健康", "排毒"] }
impl DetoxRulesRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["果蔬汁"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["肝脏肾脏"]
    }
}
impl Rule for DetoxRulesRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("detox_rules")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "排毒规则",
            &[("饮食", &self.section_0()), ("器官", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DetoxRulesRules::new();
        assert!(!r.explain().is_empty());
    }
}
