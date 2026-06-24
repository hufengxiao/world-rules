//! 研究伦理定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ResearchEthicsRules, name: "研究伦理定律", desc: "科学研究伦理定律", origin: "国际", tags: ["科学", "伦理"] }
impl ResearchEthicsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["诚信知情同意"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["伦理审查"]
    }
}
impl Rule for ResearchEthicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("research_ethics")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "研究伦理定律",
            &[("原则", &self.section_0()), ("监督", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ResearchEthicsRules::new();
        assert!(!r.explain().is_empty());
    }
}
