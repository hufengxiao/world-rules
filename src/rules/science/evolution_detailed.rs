//! 进化生物学详细
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: EvolutionDetailedRules, name: "进化生物学详细", desc: "进化生物学定律", origin: "国际", tags: ["科学", "生物"] }
impl EvolutionDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["适者生存"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["基因漂变"]
    }
}
impl Rule for EvolutionDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("evolution_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "进化生物学详细",
            &[("自然选择", &self.section_0()), ("机制", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = EvolutionDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
