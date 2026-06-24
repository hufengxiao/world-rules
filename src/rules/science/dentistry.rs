//! 口腔医学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DentistryRules, name: "口腔医学定律", desc: "口腔医学定律", origin: "国际", tags: ["科学", "医学"] }
impl DentistryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["预防治疗"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["矫正原理"]
    }
}
impl Rule for DentistryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("dentistry")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "口腔医学定律",
            &[("龋病", &self.section_0()), ("正畸", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DentistryRules::new();
        assert!(!r.explain().is_empty());
    }
}
