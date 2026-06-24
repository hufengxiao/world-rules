//! 最优化定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: OptimizationRules, name: "最优化定律", desc: "最优化定律", origin: "国际", tags: ["科学", "数学"] }
impl OptimizationRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["单纯形法"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["梯度下降"]
    }
}
impl Rule for OptimizationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("optimization")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "最优化定律",
            &[("线性", &self.section_0()), ("非线性", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = OptimizationRules::new();
        assert!(!r.explain().is_empty());
    }
}
