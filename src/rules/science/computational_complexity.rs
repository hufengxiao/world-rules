//! 计算复杂性定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ComputationalComplexityRules, name: "计算复杂性定律", desc: "计算复杂性定律", origin: "国际", tags: ["科学", "计算机"] }
impl ComputationalComplexityRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["P NP NPC"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["多项式归约"]
    }
}
impl Rule for ComputationalComplexityRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("computational_complexity")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "计算复杂性定律",
            &[("类", &self.section_0()), ("归约", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ComputationalComplexityRules::new();
        assert!(!r.explain().is_empty());
    }
}
