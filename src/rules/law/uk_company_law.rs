//! 英国公司法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: UkCompanyLawRules, name: "英国公司法", desc: "英国公司法规则", origin: "英国", tags: ["法律", "公司"] }
impl UkCompanyLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["公司注册"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["董事义务"]
    }
}
impl Rule for UkCompanyLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("uk_company_law")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "英国公司法",
            &[("设立", &self.section_0()), ("治理", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = UkCompanyLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
