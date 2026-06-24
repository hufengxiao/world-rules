//! 智能合约法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SmartContractLawRules, name: "智能合约法", desc: "智能合约法律规则", origin: "国际", tags: ["法律", "科技"] }
impl SmartContractLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["法律效力"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["争议解决"]
    }
}
impl Rule for SmartContractLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("smart_contract_law")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "智能合约法",
            &[("效力", &self.section_0()), ("争议", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SmartContractLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
