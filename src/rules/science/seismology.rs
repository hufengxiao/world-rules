//! 地震学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SeismologyRules, name: "地震学定律", desc: "地震学定律", origin: "国际", tags: ["科学", "地球"] }
impl SeismologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["P波S波"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["里氏震级"]
    }
}
impl Rule for SeismologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("seismology")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "地震学定律",
            &[("波", &self.section_0()), ("测量", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SeismologyRules::new();
        assert!(!r.explain().is_empty());
    }
}
