//! 人工智能定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ArtificialIntelligenceRules, name: "人工智能定律", desc: "人工智能定律", origin: "国际", tags: ["科学", "计算机"] }
impl ArtificialIntelligenceRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["监督无监督强化"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["CNN RNN Transformer"]
    }
}
impl Rule for ArtificialIntelligenceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("artificial_intelligence")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "人工智能定律",
            &[
                ("机器学习", &self.section_0()),
                ("深度学习", &self.section_1()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ArtificialIntelligenceRules::new();
        assert!(!r.explain().is_empty());
    }
}
