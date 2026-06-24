//! 保护生物学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ConservationBiologyRules, name: "保护生物学定律", desc: "保护生物学定律", origin: "国际", tags: ["科学", "环境"] }
impl ConservationBiologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["物种多样性"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["就地迁地"]
    }
}
impl Rule for ConservationBiologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("conservation_biology")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "保护生物学定律",
            &[("多样性", &self.section_0()), ("保护", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ConservationBiologyRules::new();
        assert!(!r.explain().is_empty());
    }
}
