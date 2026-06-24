//! 离散数学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DiscreteMathRules, name: "离散数学定律", desc: "离散数学定律", origin: "国际", tags: ["科学", "数学"] }
impl DiscreteMathRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["欧拉路径哈密顿"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["排列组合"]
    }
}
impl Rule for DiscreteMathRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("discrete_math")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "离散数学定律",
            &[("图论", &self.section_0()), ("组合", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DiscreteMathRules::new();
        assert!(!r.explain().is_empty());
    }
}
