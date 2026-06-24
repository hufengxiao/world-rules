//! 线性代数定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: LinearAlgebraRules, name: "线性代数定律", desc: "线性代数定律", origin: "国际", tags: ["科学", "数学"] }
impl LinearAlgebraRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["矩阵运算逆矩阵"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["向量空间线性变换"]
    }
}
impl Rule for LinearAlgebraRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("linear_algebra")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "线性代数定律",
            &[("矩阵", &self.section_0()), ("向量", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = LinearAlgebraRules::new();
        assert!(!r.explain().is_empty());
    }
}
