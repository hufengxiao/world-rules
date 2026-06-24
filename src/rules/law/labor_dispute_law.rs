//! 劳动争议法详解
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: LaborDisputeLawRules, name: "劳动争议法详解", desc: "劳动争议法详解", origin: "中国", tags: ["法律", "劳动"] }
impl LaborDisputeLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["劳动仲裁"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["劳动诉讼"]
    }
}
impl Rule for LaborDisputeLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("labor_dispute_law")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "劳动争议法详解",
            &[("调解", &self.section_0()), ("诉讼", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = LaborDisputeLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
