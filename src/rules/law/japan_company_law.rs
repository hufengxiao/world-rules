//! 日本公司法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: JapanCompanyLawRules, name: "日本公司法", desc: "日本公司法规则", origin: "日本", tags: ["法律", "公司"] }
impl JapanCompanyLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["株式会社"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["取缔役"]
    }
}
impl Rule for JapanCompanyLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("japan_company_law")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "日本公司法",
            &[("类型", &self.section_0()), ("治理", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = JapanCompanyLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
