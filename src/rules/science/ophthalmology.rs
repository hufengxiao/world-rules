//! 眼科学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: OphthalmologyRules, name: "眼科学定律", desc: "眼科学定律", origin: "国际", tags: ["科学", "医学"] }
impl OphthalmologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["近视远视散光"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["白内障青光眼"]
    }
}
impl Rule for OphthalmologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("ophthalmology")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "眼科学定律",
            &[("屈光", &self.section_0()), ("疾病", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = OphthalmologyRules::new();
        assert!(!r.explain().is_empty());
    }
}
