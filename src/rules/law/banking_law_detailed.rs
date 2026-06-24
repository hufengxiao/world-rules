//! 银行法详解3
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BankingLawDetailedRules, name: "银行法详解3", desc: "银行法详解3", origin: "中国", tags: ["法律", "银行"] }
impl BankingLawDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["资本充足"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["存贷款"]
    }
}
impl Rule for BankingLawDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("banking_law_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "银行法详解3",
            &[("监管", &self.section_0()), ("业务", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BankingLawDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
