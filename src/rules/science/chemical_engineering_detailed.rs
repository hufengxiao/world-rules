//! 化学工程详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChemicalEngineeringDetailedRules, name: "化学工程详细定律", desc: "化学工程定律", origin: "国际", tags: ["科学", "工程"] }
impl ChemicalEngineeringDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["反应动力学"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["蒸馏萃取"]
    }
}
impl Rule for ChemicalEngineeringDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("chemical_engineering_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "化学工程详细定律",
            &[("反应", &self.section_0()), ("分离", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChemicalEngineeringDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
