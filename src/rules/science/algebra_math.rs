//! 代数学定律

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: AlgebraMathRules,
    name: "代数学定律",
    desc: "代数学定律",
    origin: "国际",
    tags: ["科学", "数学"]
}

impl AlgebraMathRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["群的定义与性质", "拉格朗日定理", "同态基本定理"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["特征值与特征向量", "矩阵分解", "线性变换"]
    }
}

impl Rule for AlgebraMathRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("algebra_math")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "代数学定律",
            &[("群论", &self.section_0()), ("线性代数", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_algebra_math_rules() {
        let r = AlgebraMathRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
