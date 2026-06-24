//! 海关法详解
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CustomsLawDetailedRules, name: "海关法详解", desc: "海关法详解", origin: "中国", tags: ["法律", "海关"] }
impl CustomsLawDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["货物监管"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["征收减免"]
    }
}
impl Rule for CustomsLawDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("customs_law_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "海关法详解",
            &[("监管", &self.section_0()), ("关税", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CustomsLawDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
