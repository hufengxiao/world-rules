//! 药品管理法详解
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DrugMgmtDetailedRules, name: "药品管理法详解", desc: "药品管理法详解", origin: "中国", tags: ["法律", "药品"] }
impl DrugMgmtDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["临床试验"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["GMP"]
    }
}
impl Rule for DrugMgmtDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("drug_mgmt_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "药品管理法详解",
            &[("研发", &self.section_0()), ("生产", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DrugMgmtDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
