//! 劳动合同法详解
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: LaborContractLawRules, name: "劳动合同法详解", desc: "劳动合同法详解", origin: "中国", tags: ["法律", "劳动"] }
impl LaborContractLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["签订解除"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["工资休假社保"]
    }
}
impl Rule for LaborContractLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("labor_contract_law")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "劳动合同法详解",
            &[("合同", &self.section_0()), ("权益", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = LaborContractLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
