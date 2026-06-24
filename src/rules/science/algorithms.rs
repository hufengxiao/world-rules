//! 算法设计定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: AlgorithmsRules, name: "算法设计定律", desc: "算法设计定律", origin: "国际", tags: ["科学", "计算机"] }
impl AlgorithmsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["归并排序"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["背包问题"]
    }
}
impl Rule for AlgorithmsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("algorithms")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "算法设计定律",
            &[("分治", &self.section_0()), ("动态规划", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AlgorithmsRules::new();
        assert!(!r.explain().is_empty());
    }
}
