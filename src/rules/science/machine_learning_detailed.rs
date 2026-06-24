//! 机器学习详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MachineLearningDetailedRules, name: "机器学习详细定律", desc: "机器学习定律", origin: "国际", tags: ["科学", "计算机"] }
impl MachineLearningDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["回归分类"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["聚类降维"]
    }
}
impl Rule for MachineLearningDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("machine_learning_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "机器学习详细定律",
            &[("监督", &self.section_0()), ("无监督", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MachineLearningDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
