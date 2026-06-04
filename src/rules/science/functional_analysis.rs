//! 泛函分析定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: FunctionalAnalysisRules, name: "泛函分析定律", desc: "泛函分析定律", origin: "国际", tags: ["科学", "数学"] }
impl FunctionalAnalysisRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["巴拿赫空间", "希尔伯特空间"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["有界算子", "紧算子", "谱理论"]
    }
}
impl Rule for FunctionalAnalysisRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("functional_analysis")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "泛函分析定律",
            &[("空间", &self.section_0()), ("算子", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = FunctionalAnalysisRules::new();
        assert!(!r.explain().is_empty());
    }
}
