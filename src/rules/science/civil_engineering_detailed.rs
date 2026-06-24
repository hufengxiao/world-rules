//! 土木工程详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CivilEngineeringDetailedRules, name: "土木工程详细定律", desc: "土木工程定律", origin: "国际", tags: ["科学", "工程"] }
impl CivilEngineeringDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["梁柱力学"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["地基承载"]
    }
}
impl Rule for CivilEngineeringDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("civil_engineering_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "土木工程详细定律",
            &[("结构", &self.section_0()), ("岩土", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CivilEngineeringDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
