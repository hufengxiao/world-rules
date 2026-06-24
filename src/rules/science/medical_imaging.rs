//! 医学影像学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MedicalImagingRules, name: "医学影像学定律", desc: "医学影像学定律", origin: "国际", tags: ["科学", "医学"] }
impl MedicalImagingRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["X线CT"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["核磁共振"]
    }
}
impl Rule for MedicalImagingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("medical_imaging")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "医学影像学定律",
            &[("X线", &self.section_0()), ("MRI", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MedicalImagingRules::new();
        assert!(!r.explain().is_empty());
    }
}
