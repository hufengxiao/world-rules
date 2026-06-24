//! 德国公司法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: GermanCompanyLawRules, name: "德国公司法", desc: "德国公司法规则", origin: "德国", tags: ["法律", "公司"] }
impl GermanCompanyLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["GmbH AG"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["监事会"]
    }
}
impl Rule for GermanCompanyLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("german_company_law")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "德国公司法",
            &[("类型", &self.section_0()), ("治理", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = GermanCompanyLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
