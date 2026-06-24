//! 仲裁法详解
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ArbitrationLawDetailedRules, name: "仲裁法详解", desc: "仲裁法详解", origin: "中国", tags: ["法律", "仲裁"] }
impl ArbitrationLawDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["仲裁条款"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["开庭裁决"]
    }
}
impl Rule for ArbitrationLawDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("arbitration_law_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "仲裁法详解",
            &[("协议", &self.section_0()), ("程序", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ArbitrationLawDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
