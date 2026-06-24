//! 姿势健康规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PostureRulesRules, name: "姿势健康规则", desc: "姿势健康规则", origin: "国际", tags: ["健康", "姿势"] }
impl PostureRulesRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["正确坐姿"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["正确站姿"]
    }
}
impl Rule for PostureRulesRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("posture_rules")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "姿势健康规则",
            &[("坐姿", &self.section_0()), ("站姿", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PostureRulesRules::new();
        assert!(!r.explain().is_empty());
    }
}
