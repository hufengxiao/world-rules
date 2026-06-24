//! 数值分析定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: NumericalAnalysisRules, name: "数值分析定律", desc: "数值分析定律", origin: "国际", tags: ["科学", "数学"] }
impl NumericalAnalysisRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["欧拉龙格库塔"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["截断误差"]
    }
}
impl Rule for NumericalAnalysisRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("numerical_analysis")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "数值分析定律",
            &[("方法", &self.section_0()), ("误差", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = NumericalAnalysisRules::new();
        assert!(!r.explain().is_empty());
    }
}
