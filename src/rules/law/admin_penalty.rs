//! 行政处罚法详解
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: AdminPenaltyRules, name: "行政处罚法详解", desc: "行政处罚法详解", origin: "中国", tags: ["法律", "行政"] }
impl AdminPenaltyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["警告罚款"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["听证"]
    }
}
impl Rule for AdminPenaltyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("admin_penalty")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "行政处罚法详解",
            &[("种类", &self.section_0()), ("程序", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AdminPenaltyRules::new();
        assert!(!r.explain().is_empty());
    }
}
