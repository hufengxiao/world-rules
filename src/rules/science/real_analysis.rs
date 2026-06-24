//! 实分析定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: RealAnalysisRules, name: "实分析定律", desc: "实分析定律", origin: "国际", tags: ["科学", "数学"] }
impl RealAnalysisRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["极限连续"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["勒贝格积分"]
    }
}
impl Rule for RealAnalysisRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("real_analysis")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "实分析定律",
            &[("基本", &self.section_0()), ("测度", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = RealAnalysisRules::new();
        assert!(!r.explain().is_empty());
    }
}
