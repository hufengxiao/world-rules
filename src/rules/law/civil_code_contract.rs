//! 民法典合同详解
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CivilCodeContractRules, name: "民法典合同详解", desc: "民法典合同详解", origin: "中国", tags: ["法律", "民法"] }
impl CivilCodeContractRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["合同订立效力"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["买卖租赁借款"]
    }
}
impl Rule for CivilCodeContractRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("civil_code_contract")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "民法典合同详解",
            &[("通则", &self.section_0()), ("典型", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CivilCodeContractRules::new();
        assert!(!r.explain().is_empty());
    }
}
