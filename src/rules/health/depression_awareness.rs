//! 抑郁症认知规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DepressionAwarenessRules, name: "抑郁症认知规则", desc: "抑郁症认知规则", origin: "国际", tags: ["健康", "心理"] }
impl DepressionAwarenessRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["症状识别"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["专业帮助"]
    }
}
impl Rule for DepressionAwarenessRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("depression_awareness")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "抑郁症认知规则",
            &[("识别", &self.section_0()), ("求助", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DepressionAwarenessRules::new();
        assert!(!r.explain().is_empty());
    }
}
