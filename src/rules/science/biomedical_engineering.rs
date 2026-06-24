//! 生物医学工程定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BiomedicalEngineeringRules, name: "生物医学工程定律", desc: "生物医学工程定律", origin: "国际", tags: ["科学", "工程"] }
impl BiomedicalEngineeringRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["医疗器械设计"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["生物相容性"]
    }
}
impl Rule for BiomedicalEngineeringRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("biomedical_engineering")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "生物医学工程定律",
            &[("器械", &self.section_0()), ("材料", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BiomedicalEngineeringRules::new();
        assert!(!r.explain().is_empty());
    }
}
