//! 住房公积金法详解
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: HousingFundLawRules, name: "住房公积金法详解", desc: "住房公积金法详解", origin: "中国", tags: ["法律", "住房"] }
impl HousingFundLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["缴存比例"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["贷款提取"]
    }
}
impl Rule for HousingFundLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("housing_fund_law")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "住房公积金法详解",
            &[("缴存", &self.section_0()), ("使用", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = HousingFundLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
