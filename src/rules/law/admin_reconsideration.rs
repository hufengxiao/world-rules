//! 行政复议法详解
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: AdminReconsiderationRules, name: "行政复议法详解", desc: "行政复议法详解", origin: "中国", tags: ["法律", "行政"] }
impl AdminReconsiderationRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["复议申请"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["复议决定"]
    }
}
impl Rule for AdminReconsiderationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("admin_reconsideration")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "行政复议法详解",
            &[("申请", &self.section_0()), ("决定", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AdminReconsiderationRules::new();
        assert!(!r.explain().is_empty());
    }
}
