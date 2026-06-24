//! 外科学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SurgeryRules, name: "外科学定律", desc: "外科学定律", origin: "国际", tags: ["科学", "医学"] }
impl SurgeryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["无菌麻醉"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["普外骨外"]
    }
}
impl Rule for SurgeryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("surgery")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "外科学定律",
            &[("总论", &self.section_0()), ("各论", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SurgeryRules::new();
        assert!(!r.explain().is_empty());
    }
}
