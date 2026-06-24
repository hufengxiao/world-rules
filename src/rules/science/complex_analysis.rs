//! 复分析定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ComplexAnalysisRules, name: "复分析定律", desc: "复分析定律", origin: "国际", tags: ["科学", "数学"] }
impl ComplexAnalysisRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["柯西积分"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["留数定理"]
    }
}
impl Rule for ComplexAnalysisRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("complex_analysis")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "复分析定律",
            &[("基本", &self.section_0()), ("应用", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ComplexAnalysisRules::new();
        assert!(!r.explain().is_empty());
    }
}
