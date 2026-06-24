//! 深度学习详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DeepLearningDetailedRules, name: "深度学习详细定律", desc: "深度学习定律", origin: "国际", tags: ["科学", "计算机"] }
impl DeepLearningDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["CNN RNN"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["反向传播"]
    }
}
impl Rule for DeepLearningDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("deep_learning_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "深度学习详细定律",
            &[("架构", &self.section_0()), ("训练", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DeepLearningDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
