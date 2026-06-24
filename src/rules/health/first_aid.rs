//! 急救规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: FirstAidRules, name: "急救规则", desc: "基本急救规则", origin: "国际", tags: ["健康", "急救"] }
impl FirstAidRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["评估呼救处置"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["CPR止血"]
    }
}
impl Rule for FirstAidRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("first_aid")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "急救规则",
            &[("步骤", &self.section_0()), ("技能", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = FirstAidRules::new();
        assert!(!r.explain().is_empty());
    }
}
