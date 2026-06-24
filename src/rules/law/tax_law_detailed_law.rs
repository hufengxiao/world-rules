//! 税法详解3
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: TaxLawDetailedLawRules, name: "税法详解3", desc: "税法详解3", origin: "中国", tags: ["法律", "税法"] }
impl TaxLawDetailedLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["税率抵扣"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["企业个人"]
    }
}
impl Rule for TaxLawDetailedLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("tax_law_detailed_law")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "税法详解3",
            &[("增值税", &self.section_0()), ("所得税", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = TaxLawDetailedLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
